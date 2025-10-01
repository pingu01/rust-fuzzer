use anyhow::anyhow;
use colored::Colorize;
use reqwest;
use std::{fs, path::PathBuf};

use crate::wordlist::{self, Wordlist};

pub struct Fuzzer {
    pub url: String,
    pub wordlist: Wordlist,
    client: reqwest::Client,
}

impl Fuzzer {
    pub fn new(url: &str, wordlist_path: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            url: url.to_string(),
            wordlist: Wordlist::new(wordlist_path)?,
            client: reqwest::Client::new(),
        })
    }

    async fn check_host(&self) -> anyhow::Result<()> {
        match self.client.get(&self.url).send().await {
            Ok(response) => {
                println!(
                    "{} Host is reachable (Status: {})",
                    "[+]".green().bold(),
                    response.status().to_string().cyan()
                );
                Ok(())
            }
            Err(e) => Err(anyhow!(format!(
                "{} Host is not reachable: {}",
                "[-]".red().bold(),
                e
            ))),
        }
    }

    pub async fn run_default(&self) {
        if let Err(e) = self.check_host().await {
            eprintln!("{}", e);
            return;
        }

        for entry in &self.wordlist.entries {
            println!("{}/{}", self.url, entry);
        }
    }
}
