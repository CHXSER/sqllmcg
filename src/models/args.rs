use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// SonarQube host URL
    #[arg(short, long, default_value = "http://localhost:9000")]
    sonar_host: String,

    /// Ollama API URL
    #[arg(short, long, default_value = "http://localhost:11434")]
    ollama_url: String,

    /// SonarQube project key
    #[arg(short, long)]
    project_key: String,

    /// SonarQube token
    #[arg(long, default_value = "squ_94fcaf16d783848d75140f4159decfc483e753ca")]
    token: String,
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
}
