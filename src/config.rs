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
    #[clap(long="org")]
    pub organization: Option<String>,
    /// project
    #[clap(long="project")]
    pub project: Option<String>,
    /// token
    #[clap(long="token")]
    pub token: Option<String>,
    /// annotation labels
    #[clap(long="annotation-labels")]
    pub annotation_labels: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    organization: Option<String>,
    project: Option<String>,
    token: String,
    annotation_labels: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Config {
    pub organization: String,
    pub project: String,
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
                    project: config_file.project.with_context(|| "project must be set")?,
                    token: config_file.token,
                    annotation_labels: config_file.annotation_labels.unwrap_or_else(|| DEFAULT_ANNOTATION_LABELS.iter().map(|s| s.to_string()).collect()),
                })
            }
            None => {
                Ok(Config {
                    organization: args.organization.clone().with_context(|| "organization must be set")?,
                    project: args.project.clone().with_context(|| "project must be set")?,
                    token: args.token.clone().with_context(|| "token must be set")?,
                    annotation_labels: args.annotation_labels.clone().unwrap_or_else(|| DEFAULT_ANNOTATION_LABELS.iter().map(|s| s.to_string()).collect()),
                })
            }
        }
    }
}
