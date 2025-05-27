use clap::Parser;
use crate::models::config::Config;
use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// URL di SonarQube
    #[arg(short, long)]
    sonar_host: Option<String>,

    /// URL di Ollama
    #[arg(short, long)]
    ollama_url: Option<String>,

    /// Nome del progetto di SonarQube
    #[arg(short, long)]
    project_key: String,

    /// Token di SonarQube
    #[arg(long)]
    token: Option<String>,

    /// Nome del modello LLM
    #[arg(long, short)]
    model: Option<String>,

    /// Regole da segnare direttamente come false positive
    #[arg(long, short, num_args = 0..)]
    rules: Vec<String>,
}

impl Args {
    pub fn new() -> Result<Self> {
        let mut args = Args::parse();
        let mut config = Config::load()?;

        config.update(
            args.sonar_host.clone(),
            args.ollama_url.clone(),
            args.token.clone(),
            args.model.clone(),
        )?;

        args.sonar_host = Some(config.sonar_host);
        args.ollama_url = Some(config.ollama_url);
        args.token = Some(config.token);
        args.model = Some(config.model);

        Ok(args)
    }

    pub fn sonar_host(&self) -> String {
        self.sonar_host.clone().unwrap()
    }

    pub fn ollama_url(&self) -> String {
        self.ollama_url.clone().unwrap()
    }

    pub fn project_key(&self) -> String {
        self.project_key.clone()
    }

    pub fn token(&self) -> String {
        self.token.clone().unwrap()
    }

    pub fn model(&self) -> String {
        self.model.clone().unwrap()
    }

    pub fn rules(&self) -> Vec<String> {
        self.rules.clone()
    }
}
