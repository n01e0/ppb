use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;

pub const DEFAULT_ANNOTATION_LABELS: [&str; 2] = ["TODO", "FIXME"];
pub const DEFAULT_TITLE_FORMAT: &str = "[Postpone] {label}: {line}";
pub const DEFAULT_BODY_FORMAT: &str = r#"
Postpone: {label}

{file}:{line_number}

```
{line}
```
"#;
pub const PPB_ISSUE_LABEL: &str = "postpone";

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    /// config file
    #[clap(short, long)]
    pub config: Option<String>,
    /// organization
    #[clap(long = "organization")]
    pub organization: Option<String>,
    /// repository
    #[clap(long = "repository")]
    pub repository: Option<String>,
    /// GitHub token
    #[clap(long = "token")]
    pub token: Option<String>,
    #[clap(long = "listup", short = 'l')]
    /// list up postpones
    pub listup: bool,
    /// annotation labels
    /// default: ["TODO", "FIXME"]
    #[clap(long="annotation-labels", value_parser, num_args = 1.., value_delimiter = ',')]
    pub annotation_labels: Option<Vec<String>>,
    /// title format
    /// you can use following variables
    /// {label} {file} {line_number} {line}
    /// default: "Postpone: {label} {file} {line_number}"
    #[clap(long="title-format", default_value=DEFAULT_TITLE_FORMAT)]
    pub title_format: Option<String>,
    /// body format
    /// you can use following variables
    /// {label} {file} {line_number} {line}
    /// default: "Postpone: {label}\n\n{file}:{line_number}\n\n```\n{line}\n```"
    #[clap(long="body-format", default_value=DEFAULT_BODY_FORMAT)]
    pub body_format: Option<String>,
    /// dry run
    /// will not create issues
    /// default: false
    #[clap(long="dry-run")]
    pub dry_run: bool,
    // TODO ignore files
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    organization: Option<String>,
    repository: Option<String>,
    token: Option<String>,
    annotation_labels: Option<Vec<String>>,
    title_format: Option<String>,
    body_format: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub organization: String,
    pub repository: String,
    pub token: String,
    pub annotation_labels: Vec<String>,
    pub title_format: String,
    pub body_format: String,
}

impl Config {
    pub fn new(args: &Args) -> Result<Self> {
        match args.config {
            Some(ref config_file) => {
                let config_file = std::fs::read_to_string(config_file).unwrap();
                let config_file: ConfigFile = serde_yaml::from_str(&config_file).unwrap();
                Ok(Config {
                    organization: config_file
                        .organization
                        .or(args.organization.clone())
                        .with_context(|| "organization must be set")?,
                    repository: config_file
                        .repository
                        .or(args.repository.clone())
                        .with_context(|| "repository must be set")?,
                    token: config_file.token.or(args.token.clone()).with_context(|| {
                        "token must be set"
                    })?,
                    annotation_labels: config_file.annotation_labels.or(args.annotation_labels.clone()).unwrap_or_else(|| {
                        DEFAULT_ANNOTATION_LABELS
                            .iter()
                            .map(|s| s.to_string())
                            .collect()
                    }),
                    title_format: config_file
                        .title_format
                        .or(args.title_format.clone())
                        .unwrap_or_else(|| DEFAULT_TITLE_FORMAT.to_string()),
                    body_format: config_file
                        .body_format
                        .or(args.title_format.clone())
                        .unwrap_or_else(|| DEFAULT_BODY_FORMAT.to_string()),
                })
            }
            None => Ok(Config {
                organization: args
                    .organization
                    .clone()
                    .with_context(|| "organization must be set")?,
                repository: args
                    .repository
                    .clone()
                    .with_context(|| "repository must be set")?,
                token: args.token.clone().with_context(|| "token must be set")?,
                annotation_labels: args.annotation_labels.clone().unwrap_or_else(|| {
                    DEFAULT_ANNOTATION_LABELS
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                }),
                title_format: args
                    .title_format
                    .clone()
                    .unwrap_or_else(|| DEFAULT_TITLE_FORMAT.to_string()),
                body_format: args
                    .body_format
                    .clone()
                    .unwrap_or_else(|| DEFAULT_BODY_FORMAT.to_string()),
            }),
        }
    }
}
