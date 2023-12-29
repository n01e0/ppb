mod config;
mod github;
mod postpone;

use config::{Args, Config};
use postpone::Postpone;
use anyhow::{Result, Context};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()>  {
    let args = Args::parse();

    if args.listup {
        let pattern = format!("({})", args.annotation_labels.with_context(|| "listup needs annotation labels")?.join("|"));
        let postpones = Postpone::search(&pattern)?;
        for postpone in postpones {
            println!("{}:{} {}\n\t{}", postpone.file, postpone.line_number, postpone.label, postpone.line);
        }
        return Ok(());
    }
    if args.dry_run {
        let pattern = format!("({})", args.annotation_labels.with_context(|| "listup needs annotation labels")?.join("|"));
        let postpones = Postpone::search(&pattern)?
            .into_iter()
            .map(|postpone| postpone.to_issue(&args.title_format.clone().with_context(|| "dry_run needs title format")?, &args.body_format.clone().with_context(|| "dry_run needs body format")?))
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
    let issues = client.get_issues().await?.into_iter().filter(|issue| {
        issue.labels.iter().any(|label| label.name == "postpone")
    }).collect::<Vec<_>>();

    // TODO: bodyが一致してタイトルが異なる場合、タイトルを更新する
    let postpones = Postpone::search(&pattern)?
        .into_iter()
        .map(|postpone| postpone.to_issue(&config.title_format, &config.body_format))
        .filter_map(|issue| issue.ok())
        .filter(|(_, body)| !issues.iter().any(|issue| issue.body == Some(body.to_string())))
        .collect::<Vec<(String, String)>>();

    for (title, body) in postpones {
        client.create_issue(&title, &body).await?;
    }

    Ok(())
}
