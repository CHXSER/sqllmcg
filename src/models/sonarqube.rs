use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct SonarIssue {
    key: String,
    message: String,
    path: String,
    line: Option<i32>,
}

impl SonarIssue {
    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn line(&self) -> Option<i32> {
        self.line
    }

    pub fn from_json_value(value: &Value, project_key: &str) -> Option<Self> {
        let component = value["component"].as_str()?;
        let path = component.replace(project_key, "");

        Some(SonarIssue {
            key: value["key"].as_str()?.to_string(),
            message: value["message"].as_str()?.to_string(),
            path,
            line: value["line"].as_i64().map(|l| l as i32),
        })
    }

    pub async fn get_code_context(
        &self,
        client: &Client,
        sonar_host: &str,
        project_key: &str,
        token: &str,
    ) -> Result<String> {
        let clean_path = self.path.trim_start_matches('/');

        let source_url = format!("{}/api/sources/raw", sonar_host);

        let clean_path = clean_path.trim_start_matches(':');
        let component_key = format!("{}:{}", project_key, clean_path);

        let response = client
            .get(&source_url)
            .header("Authorization", format!("Bearer {}", token))
            .query(&[("key", &component_key)])
            .send()
            .await
            .context("Impossibile prendere il codice sorgente da SonarQube")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "Impossibile prendere il codice sorgente da SonarQube. Status: {}, Errore: {}",
                status.as_u16(),
                error_text
            ));
        }

        let content = response.text().await?;
        if content.is_empty() {
            return Ok("Nessun codice sorgente disponibile".to_string());
        }

        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return Ok("Il file è vuoto".to_string());
        }

        // Get 5 lines before and after the issue line
        let line_num = self.line.unwrap_or(0) as usize;
        if line_num == 0 || line_num > lines.len() {
            return Ok(format!("Numero di riga non valido: {}", line_num));
        }

        let start = if line_num > 10 { line_num - 10 } else { 0 };
        let end = std::cmp::min(line_num + 10, lines.len());

        let context: Vec<String> = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let line_number = start + i + 1;
                if line_number == line_num {
                    format!(">> {}: {}", line_number, line)
                } else {
                    format!("   {}: {}", line_number, line)
                }
            })
            .collect();

        Ok(context.join("\n"))
    }
}

pub fn parse_issues_from_json(json: &Value, project_key: &str) -> Vec<SonarIssue> {
    json["issues"]
        .as_array()
        .map(|issues| {
            issues
                .iter()
                .filter_map(|issue| SonarIssue::from_json_value(issue, project_key))
                .collect()
        })
        .unwrap_or_default()
}
