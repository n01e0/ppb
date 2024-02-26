mod config;
mod github;
mod postpone;

use anyhow::{Context, Result};
use clap::Parser;
use config::{Args, Config, ConfigFile};
use postpone::Postpone;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.dry_run {
        let (target_dir, pattern, body_format, title_format) = match args.config {
            Some(config) => {
                let config: ConfigFile = serde_yaml::from_str(&std::fs::read_to_string(config)?)?;
                (
                    config.target_dir.with_context(|| "dry_run needs target dir")?,
                    format!("({})", config.annotation_labels.with_context(|| "dry_run needs annotation labels")?.join("|")),
                    config.body_format.with_context(|| "dry_run needs body format")?,
                    config.title_format.with_context(|| "dry_run needs title format")?,
                )
            }
            None => (
                args.target_dir
                    .clone(),
                format!(
                    "({})",
                    args.annotation_labels
                        .with_context(|| "dry_run needs annotation labels")?
                        .join("|")
                ),
                args.body_format
                    .clone()
                    .with_context(|| "dry_run needs body format")?,
                args.title_format
                    .clone()
                    .with_context(|| "dry_run needs title format")?,
            ),
        };
        let postpones = Postpone::search(&target_dir, &pattern, &args.ignore_file.unwrap_or(Vec::new()))?
            .into_iter()
            .map(|postpone| postpone.to_issue(&title_format, &body_format))
            .filter_map(|issue| issue.ok())
            .collect::<Vec<(String, String)>>();

        for (title, body) in postpones {
            println!("title: {}\nbody: {}\n", title, body);
        }
        return Ok(());
    }

    let config = Config::new(&args)?;
    let pattern = format!("({})", config.annotation_labels.join("|"));
    let client = github::GitHub::new(&config)?;
    // Created by ppb, existing Issue
    let issues = client
        .get_issues()
        .await?
        .into_iter()
        .filter(|issue| issue.labels.iter().any(|label| label.name == "postpone"))
        .collect::<Vec<_>>();

    let postpones = Postpone::search(&config.target_dir, &pattern, &config.ignore_file)?
        .into_iter()
        .map(|postpone| postpone.to_issue(&config.title_format, &config.body_format))
        .filter_map(|issue| issue.ok())
        .filter(|(title, _)| !issues.iter().any(|issue| issue.title == title.to_string()))
        .collect::<Vec<(String, String)>>();

    for (title, body) in postpones {
        client.create_issue(&title, &body).await?;
    }

    Ok(())
}
