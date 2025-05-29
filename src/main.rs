use anyhow::{Context, Result};

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

mod models;

use models::args::Args;
use models::ollama::{OllamaRequest, OllamaResponse};
use models::sonarqube::{SonarIssue, parse_issues_from_json};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::new()?;
    let client = Client::new();
    let mut all_issues: Vec<SonarIssue> = Vec::new();

    let issues_url = format!("{}/api/issues/search", args.sonar_host());
    let response = client
        .get(&issues_url)
        .header("Authorization", format!("Bearer {}", args.token()))
        .query(&[
            ("projects", &args.project_key()),
            (
                "impactSoftwareQualities",
                &"SECURITY,RELIABILITY".to_string(),
            )
        ])
        .send()
        .await
        .context("Impossibile recuperare gli issue da SonarQube")?;

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
                    ("p", &page.to_string())
                ])
                .send()
                .await?;

            let page_json: Value = response.json().await?;
            let page_issues = parse_issues_from_json(&page_json);
            all_issues.extend(page_issues);
        }
    } else {
        let issues = parse_issues_from_json(&first_response);
        all_issues.extend(issues);
    }

    println!("Trovate {} issues", all_issues.len());

    for issue in all_issues {
        let code_context = match issue
            .get_code_context(
                &client,
                &args.sonar_host(),
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

        let prompt;

        if !args.rules().is_empty() {
            prompt = format!("
            You are a cybersecurity expert specialized in static analysis and SonarQube.
            IMPORTANT: DO NOT USE MARKDOWN FORMATTING. Use SonarQube's specific formatting instead:
            - For bold text: use a single * character between the words (e.g., *Bold text*)
            - For code: use double backticks between the code (e.g., ``JavaClass.getCode()``)
            - Do not use any other formatting characters like #, -, >, etc.
            - For code blocks, use also double backticks before and after the code block.
            
            Provide ONLY a direct analysis and fix in the following format:
                - Add this to the first line of the response: *{}*;
                - Brief description of the issue;
                - Whether it's a false positive or not;
                - If false positive say it in bold, explain why (In Java you say to add an annotation to suppress 
                the warning); 
                - If not false positive, provide the fix;
                - About the false positives, be conservative, don't say it's a false positive if you're not absolutely sure
                - Keep your response concise and focused only on the technical analysis and fix. Do not include any introductory text, 
                explanations about your role, or general advice.
                - Mark this rules exclusively as false positives, and say *false positive by user input*: ``{}``

            CODE FIX:
            ``
            // Your code fix here
            ``

            Issue details:
            Rule: {}
            File: {}
            Line: {}
            Message: {}

            Code context:
            ``
            {}
            ``
            /nothink",
            args.model(),
            args.rules().join(" "),
            issue.rule(),
            issue.path(),
            issue.line().unwrap_or(0),
            issue.message(),
            &code_context);
        } else {
            prompt = format!("
            You are a cybersecurity expert specialized in static analysis and SonarQube.
            IMPORTANT: DO NOT USE MARKDOWN FORMATTING. Use SonarQube's specific formatting instead:
            - For bold text: use a single * character between the words (e.g., *Bold text*)
            - For code: use double backticks between the code (e.g., ``JavaClass.getCode()``)
            - Do not use any other formatting characters like #, -, >, etc.
            - For code blocks, use also double backticks before and after the code block.
            
            Provide ONLY a direct analysis and fix in the following format:
                - Add this to the first line of the response: *{}*;
                - Brief description of the issue;
                - Whether it's a false positive or not;
                - If false positive say it in bold, explain why (In Java you say to add an annotation to suppress 
                the warning); 
                - If not false positive, provide the fix;
                - About the false positives, be conservative, don't say it's a false positive if you're not absolutely sure
                - Keep your response concise and focused only on the technical analysis and fix. Do not include any introductory text, 
                explanations about your role, or general advice.

            CODE FIX:
            ``
            // Your code fix here
            ``

            Issue details:
            Rule: {}
            File: {}
            Line: {}
            Message: {}

            Code context:
            ``
            {}
            ``
            /nothink",
            args.model(),
            issue.rule(),
            issue.path(),
            issue.line().unwrap_or(0),
            issue.message(),
            &code_context);
        }

        let ollama_request = OllamaRequest::new(args.model(), prompt, false);

        let ollama_response = client
            .post(format!("{}/api/generate", args.ollama_url()))
            .json(&ollama_request)
            .send()
            .await
            .context("Impossibile inviare la richiesta a Ollama")?;

        let ollama_result: OllamaResponse = ollama_response
            .json()
            .await?;

        // Aggiungere tag falso positivo
        // if ollama_result.response().to_lowercase().contains("*false positive*") {
        //     let tag_url = format!("{}/api/issues/add_tags", args.sonar_host());
        //     let mut tag_params = HashMap::new();
        //     tag_params.insert("issue", issue.key());
        //     tag_params.insert("tags", "false-positive".to_string());

        //     let tag_response = client
        //         .post(&tag_url)
        //         .header("Authorization", format!("Bearer {}", args.token()))
        //         .form(&tag_params)
        //         .send()
        //         .await
        //         .context("Failed to add tag to SonarQube")?;

        //     if !tag_response.status().is_success() {
        //         let error_text = tag_response.text().await?;
        //         //let status = tag_response.status();
        //         println!(
        //             "Failed to add tag for issue {}: {}",
        //             issue.key(),
        //             //status.as_u16(),
        //             error_text
        //         );
        //     } else {
        //         println!("Successfully added false-positive tag for issue: {}", issue.key());
        //     }
        // }

        let comment_url = format!("{}/api/issues/add_comment", args.sonar_host());
        let mut params = HashMap::new();
        params.insert("issue", issue.key());
        params.insert("text", ollama_result.response());

        let _ = client
            .post(&comment_url)
            .header("Authorization", format!("Bearer {}", args.token()))
            .form(&params)
            .send()
            .await
            .context("Impossibile aggiungere il commento a SonarQube")?;

        println!("Commento aggiunto per l'issue: {}", issue.key());
    }

    Ok(())
}
