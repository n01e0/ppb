use crate::config::{self, Config};
use anyhow::{anyhow, Context, Result};
use octocrab::{models::issues::Issue, params, Octocrab, OctocrabBuilder};

#[derive(Debug)]
pub struct GitHub<'a> {
    config: &'a Config,
    client: Octocrab,
}

impl<'a> GitHub<'a> {
    pub fn new(config: &'a Config) -> Result<Self> {
        let client = OctocrabBuilder::new()
            .personal_token(config.token.to_string())
            .build()
            .context("Failed to create GitHub client")?;
        Ok(Self { config, client })
    }

    pub async fn get_issues(&self) -> Result<Vec<Issue>> {
        let page = self
            .client
            .issues(&self.config.organization, &self.config.repository)
            .list()
            .labels(&vec![String::from(config::PPB_ISSUE_LABEL)])
            .state(params::State::All)
            .per_page(100)
            .send()
            .await?;

        self.client
            .all_pages::<Issue>(page)
            .await
            .with_context(|| anyhow!("Failed to get issues from GitHub"))
    }

    pub async fn create_issue(&self, title: &str, body: &str) -> Result<Issue> {
        self.client
            .issues(&self.config.organization, &self.config.repository)
            .create(title)
            .body(body)
            .labels(Some(vec![String::from(config::PPB_ISSUE_LABEL)]))
            .send()
            .await
            .with_context(|| anyhow!("Failed to create issue on GitHub"))
    }
}
