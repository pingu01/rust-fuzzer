use colored::Colorize;
use reqwest;
use std::fs;

pub struct Fuzzer {
    pub url: String,
    pub wordlist_path: String,
}

impl Fuzzer {
    pub fn new(url: &str, wordlist_path: &str) -> Self {
        Self {
            url: url.to_string(),
            wordlist_path: wordlist_path.to_string(),
        }
    }

    async fn check_host(&self) -> Result<(), String> {
        let client = reqwest::Client::new();

        match client.get(&self.url).send().await {
            Ok(response) => {
                println!(
                    "{} Host is reachable (Status: {})",
                    "[+]".green().bold(),
                    response.status().to_string().cyan()
                );
                Ok(())
            }
            Err(e) => Err(format!(
                "{} Host is not reachable: {}",
                "[-]".red().bold(),
                e
            )),
        }
    }

    pub async fn run_default(&self) {
        if let Err(e) = self.check_host().await {
            eprintln!("{}", e);
            return;
        }
    }
}
