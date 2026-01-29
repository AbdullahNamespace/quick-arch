use anyhow::{bail, Result};
use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub config: String,

    #[clap(short, long)]
    pub output: Option<String>,
}

impl Cli {
    pub fn validate_config_exists(&self) -> Result<()> {
        if !Path::new(&self.config).exists() {
            bail!(
                "Config file not found: '{}'. Please provide a valid path.",
                self.config
            );
        }
        Ok(())
    }
}
