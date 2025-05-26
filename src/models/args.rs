use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// SonarQube host URL
    #[arg(short, long, default_value = "http://localhost:9000")]
    sonar_host: String,

    /// Ollama API URL
    #[arg(short, long, default_value = "http://padova.zucchettitest.it:11434")]
    ollama_url: String,

    /// SonarQube project key
    #[arg(short, long)]
    project_key: String,

    /// SonarQube token
    #[arg(long, default_value = "squ_77aafc8e5865e7c75ffcc340627cac9fbc115fb7")]
    token: String,

    #[arg(long, default_value="gemma3:4b")]
    model: String,
}

impl Args {
    pub fn sonar_host(&self) -> String {
        self.sonar_host.clone()
    }

    pub fn ollama_url(&self) -> String {
        self.ollama_url.clone()
    }

    pub fn project_key(&self) -> String {
        self.project_key.clone()
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }
}
