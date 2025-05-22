use anyhow::{Context, Result};

use clap::Parser;
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

mod models;

use models::args::Args;
use models::ollama::{OllamaRequest, OllamaResponse};
use models::sonarqube::{SonarIssue, parse_issues_from_json};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::new();
    let mut all_issues: Vec<SonarIssue> = Vec::new();

    // Get issues from SonarQube
    let issues_url = format!("{}/api/issues/search", args.sonar_host());
    let response = client
        .get(&issues_url)
        .header("Authorization", format!("Bearer {}", args.token()))
        .query(&[
            ("projects", &args.project_key()),
            (
                "impactSoftwareQualities",
                &"SECURITY,RELIABILITY".to_string(),
            ),
            ("languages", &"java".to_string()),
        ])
        .send()
        .await
        .context("Impossibile prendere le issue da SonarQube")?;

    let first_response: Value = response.json().await?;
    let total_issues = first_response["total"].as_u64().unwrap_or(0);

    if total_issues > 0 {
        for page in 1..=total_issues {
            let response = client
                .get(&issues_url)
                .header("Authorization", format!("Bearer {}", args.token()))
                .query(&[
                    ("projects", &args.project_key()),
                    (
                        "impactSoftwareQualities",
                        &"SECURITY,RELIABILITY".to_string(),
                    ),
                    ("languages", &"java".to_string()),
                    ("p", &page.to_string()),
                ])
                .send()
                .await?;
            let page_json: Value = response.json().await?;
            let page_issues = parse_issues_from_json(&page_json, &args.project_key());
            all_issues.extend(page_issues);
        }
    } else {
        let issues = parse_issues_from_json(&first_response, &args.project_key());
        all_issues.extend(issues);
    }

    println!("Found {} issues", all_issues.len());

    // Process each issue with Ollama
    for issue in all_issues {
        // Get code context for the issue
        let code_context = match issue
            .get_code_context(
                &client,
                &args.sonar_host(),
                &args.project_key(),
                &args.token(),
            )
            .await
        {
            Ok(context) => context,
            Err(e) => {
                println!(
                    "Warning: Could not fetch code context for issue {}: {}",
                    issue.key(),
                    e
                );
                "No code context available".to_string()
            }
        };

        let prompt = format!(
            "You are a cybersecurity expert analyzing a SonarQube issue.
            Follow SonarQube's formatting like this: 
            for bold text use a single * character between the words, 
            and for code use a double ` (two backticks between the code)
            like this: 
            - *Bold text*
            - ``CodeClass.getCode()``
            Provide ONLY a direct analysis and fix in the following format:
            
            ISSUE ANALYSIS:
                - Brief description of the issue
                - Whether it's a false positive or not
                - If false positive say it in bold, explain why 
                - If not false positive, provide the fix
                - About the false positives, be conservative, don't say it's a false positive if you're not absolutely sure

            CODE FIX:
            ``
            // Your code fix here
            ``

            Keep your response concise and focused only on the technical analysis and fix. Do not include any introductory text, 
            explanations about your role, or general advice.

            Issue details:
            File: {}
            Line: {}
            Message: {}

            Code context:
            ```
            {}
            ```", 
            issue.path(),
            issue.line().unwrap_or(0),
            issue.message(),
            &code_context
        );

        let ollama_request = OllamaRequest::new("gemma3:12b".to_string(), prompt, false);

        let ollama_response = client
            .post(format!("{}/api/generate", args.ollama_url()))
            .json(&ollama_request)
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        let ollama_result: OllamaResponse = ollama_response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        // Add comment to SonarQube
        let comment_url = format!("{}/api/issues/add_comment", args.sonar_host());
        let mut params = HashMap::new();
        params.insert("issue", issue.key());
        params.insert("text", ollama_result.response());

        let comment_response = client
            .post(&comment_url)
            .header("Authorization", format!("Bearer {}", args.token()))
            .form(&params)
            .send()
            .await
            .context("Failed to add comment to SonarQube")?;

        // Check response status
        let status = comment_response.status();
        if !status.is_success() {
            let error_text = comment_response.text().await?;
            println!(
                "Failed to add comment for issue {}: Status {} - {}",
                issue.key(),
                status.as_u16(),
                error_text
            );
        } else {
            println!("Successfully added comment for issue: {}", issue.key());
        }
    }

    Ok(())
}
