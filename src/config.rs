use clap::Parser;
use serde::Deserialize;
use anyhow::{Result, Context};

pub const DEFAULT_ANNOTATION_LABELS: [&str; 2] = ["TODO", "FIXME"];
pub const PPB_ISSUE_LABEL: &str = "postpone";

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    /// config file
    #[clap(short, long)]
    pub config: Option<String>,
    /// organization
    #[clap(long="organization")]
    pub organization: Option<String>,
    /// repository
    #[clap(long="repository")]
    pub repository: Option<String>,
    /// GitHub token
    #[clap(long="token")]
    pub token: Option<String>,
    /// annotation labels
    /// default: ["TODO", "FIXME"]
    #[clap(long="annotation-labels", value_parser, num_args = 1.., value_delimiter = ',')]
    pub annotation_labels: Option<Vec<String>>,
    // TODO ignore files
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    organization: Option<String>,
    repository: Option<String>,
    token: String,
    annotation_labels: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Config {
    pub organization: String,
    pub repository: String,
    pub token: String,
    pub annotation_labels: Vec<String>,
}

impl Config {
    pub fn new(args: &Args) -> Result<Self> {
        match args.config {
            Some(ref config_file) => {
                let config_file = std::fs::read_to_string(config_file).unwrap();
                let config_file: ConfigFile = serde_yaml::from_str(&config_file).unwrap();
                Ok(Config {
                    organization: config_file.organization.with_context(|| "organization must be set")?,
                    repository: config_file.repository.with_context(|| "repository must be set")?,
                    token: config_file.token,
                    annotation_labels: config_file.annotation_labels.unwrap_or_else(|| DEFAULT_ANNOTATION_LABELS.iter().map(|s| s.to_string()).collect()),
                })
            }
            None => {
                Ok(Config {
                    organization: args.organization.clone().with_context(|| "organization must be set")?,
                    repository: args.repository.clone().with_context(|| "repository must be set")?,
                    token: args.token.clone().with_context(|| "token must be set")?,
                    annotation_labels: args.annotation_labels.clone().unwrap_or_else(|| DEFAULT_ANNOTATION_LABELS.iter().map(|s| s.to_string()).collect()),
                })
            }
        }
    }
}
