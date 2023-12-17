mod config;
mod github;
mod postpone;

use config::{Args, Config};
use postpone::Postpone;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()>  {
    let args = Args::parse();
    let config = Config::new(&args)?;
    let pattern = format!("({})", config.annotation_labels.join("|"));

    let client = github::GitHub::new(config)?;
    // Created by ppb, existing Issue
    let issues = client.get_issues().await?.into_iter().filter(|issue| {
        issue.labels.iter().any(|label| label.name == "postpone")
    }).collect::<Vec<_>>();

    let postpones = Postpone::search(&pattern)?
        .into_iter()
        .map(|postpone| postpone.to_issue())
        .filter(|(_, body)| !issues.iter().any(|issue| issue.body== Some(body.to_string())))
        .collect::<Vec<(String, String)>>();

    for (title, body) in postpones {
        client.create_issue(&title, &body).await?;
    }

    Ok(())
}
