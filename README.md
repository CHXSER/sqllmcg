# SonarQube LLM comment generator

Questo tool è il risultato di un tirocinio della durata di 320 ore effettuato presso l'azienda Zucchetti S.p.A. L'obiettivo è quello 
di integrare l'intelligenza artificiale con l'analisi statica, in particolare con SonarQube.

## Prerequisiti

### SonarQube
- Server SonarQube in esecuzione 
  - [Guida all'intallazione di SonarQube](https://docs.sonarsource.com/sonarqube-server/latest/setup-and-upgrade/overview/)
- *User* Token di accesso valido
- Progetto configurato con almeno una issue
### Ollama
- Server Ollama in esecuzione
  - [Installazione di Ollama](https://ollama.ai/download)
- Modello LLM configurato (consigliato gemma3:12b)
  - [Lista dei modelli disponibili](https://ollama.ai/library)

### Rust
- Rust 1.70.0 o superiore
  - [Installazione di Rust](https://www.rust-lang.org/tools/install)

## Installazione

1. Clona il repository:
```bash
git clone https://github.com/CHXSER/sqllmcg.git
cd sqllmcg
```

2. Installa le dipendenze:
```bash
cargo build --release
```

## Funzionamento

Il tool analizza automaticamente i problemi di sicurezza e affidabilità rilevati da SonarQube utilizzando un modello LLM (Large Language Model) tramite Ollama. Per ogni problema:

1. Recupera il contesto del codice (5 righe prima e dopo della vulnerabilità)
2. Analizza il problema utilizzando il modello LLM
3. Aggiunge un commento dettagliato in SonarQube che contiene:
  - Se la issue è un falso positivo
  - Descrizione dettagliata della issue
  - Una possibile soluzione alla issue

## Utilizzo

```bash
cargo run --release -- -p VulnerableApp
```

### Parametri

- `-s` `--sonar-host`: (Opzionale) URL del server SonarQube (default = "http://localhost:9000")
- `--token`: Token di accesso SonarQube (Obbligatorio per la prima esecuzione)
- `-p` `--project-key`: (**Obbligatoria**) Chiave del progetto SonarQube
- `-o` `--ollama-url`: (Opzionale) URL del server Ollama, (default = "http://localhost:11434")
- `-m` `--model`: (Opzionale) Nome del modello LLM da utilizzare, (default = "deepseek-r1:14b")
- `-r` `--rules`: (Opzionale) Lista di regole da segnalare come falsi positivi

## Esempio

```bash
cargo r --release -- --sonar-host http://localhost:9000 --token abc123 --project-key my-project --ollama-url http://localhost:11434 --model deepseek-r1:14b
```