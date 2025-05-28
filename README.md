# SonarQube LLM comment generator

Questo tool è il risultato di un tirocinio della durata di 320 ore effettuato presso l'azienda Zucchetti S.p.A. L'obiettivo è quello 
di integrare l'intelligenza artificiale con l'analisi statica, in particolare con SonarQube.

## Prerequisiti

### SonarQube
- Server SonarQube in esecuzione 
- Token di accesso valido
  - [Guida alla generazione del token](https://docs.sonarqube.org/latest/user-guide/user-account/generating-and-using-tokens/)
- Progetto configurato in SonarQube
  - [Guida alla configurazione del progetto](https://docs.sonarqube.org/latest/setup/configure-project-analysis/)

### Ollama
- Server Ollama in esecuzione
  - [Installazione di Ollama](https://ollama.ai/download)
  - [Guida all'installazione con Docker](https://github.com/ollama/ollama/blob/main/docs/docker.md)
- Modello LLM configurato (es. llama2, mistral, ecc.)
  - [Lista dei modelli disponibili](https://ollama.ai/library)
  - [Guida all'utilizzo dei modelli](https://github.com/ollama/ollama/blob/main/docs/import.md)

### Rust
- Rust 1.70.0 o superiore
  - [Installazione di Rust](https://www.rust-lang.org/tools/install)
  - [Guida all'aggiornamento di Rust](https://doc.rust-lang.org/book/ch01-01-installation.html#updating-rust)
- Cargo (package manager di Rust)
  - Installato automaticamente con Rust
  - [Documentazione di Cargo](https://doc.rust-lang.org/cargo/)

## Installazione

1. Clona il repository:
```bash
git clone https://github.com/CHXSER/sqllmcg
cd sqllmcg
```

2. Installa le dipendenze:
```bash
cargo build
```

## Funzionamento

Il tool analizza automaticamente i problemi di sicurezza e affidabilità rilevati da SonarQube utilizzando un modello LLM (Large Language Model) tramite Ollama. Per ogni problema:

1. Recupera il contesto del codice
2. Analizza il problema utilizzando il modello LLM
3. Aggiunge un commento dettagliato in SonarQube
4. Marca i falsi positivi quando appropriato

## Utilizzo

```bash
cargo run -- --sonar-host [URL_SONARQUBE] --token [TOKEN] --project-key [PROJECT_KEY] --ollama-url [URL_OLLAMA] --model [MODEL_NAME]
```

### Parametri

- `-s` `--sonar-host`: (Opzionale) URL del server SonarQube (default = "http://localhost:9000")
- `--token`: Token di accesso SonarQube (Obbligatorio per la prima esecuzione)
- `-p` `--project-key`: (**Obbligatoria**) Chiave del progetto SonarQube
- `-o` `--ollama-url`: (Opzionale) URL del server Ollama, (default = "http://localhost:11434")
- `-m` `--model`: (Opzionale) Nome del modello LLM da utilizzare, (default = "deepseek-r1:14b")
- `-r` `--rules`: (Opzionale) Lista di regole note come falsi positivi

## Esempio

```bash
cargo r --release -- --sonar-host http://localhost:9000 --token abc123 --project-key my-project --ollama-url http://localhost:11434 --model llama2
```

## Funzionalità

- Analisi automatica dei problemi di sicurezza e affidabilità
- Identificazione dei falsi positivi
- Aggiunta di commenti dettagliati in SonarQube
- Tagging automatico dei falsi positivi
- Supporto per regole personalizzate di falsi positivi



