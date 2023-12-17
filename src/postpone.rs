use grep::{regex::RegexMatcher, searcher::{Searcher, sinks::UTF8}};
use ignore::Walk;
use anyhow::{Result, Context};

#[derive(Debug)]
pub struct Postpone {
    pub file: String,
    pub line_number: u64,
    pub matched: String,
}

impl Postpone {
    pub fn search(pattern: &str) -> Result<Vec<Self>> {
        let matcher = RegexMatcher::new_line_matcher(pattern)?;
        let mut result = Vec::new();

        // TODO: layonとか使って並列化したい
        Walk::new(".")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .filter(|e| !e.path().to_str().unwrap().starts_with(".git"))
            .into_iter()
            .try_for_each(|entry| {
                let path = entry.path();
                let path = path.to_str().unwrap();

                Searcher::new().search_path(
                    &matcher,
                    path,
                    UTF8(|line_number, line| {
                        result.push(Postpone{
                            file: path.to_string(),
                            line_number,
                            matched: line.to_string()
                        });
                        Ok(true)
                    })
                ).with_context(|| format!("failed to search {}", path))
            })?;

        Ok(result)
    }

    pub fn to_issue(&self) -> (String, String) {
        // TODO: フォーマットを変えられるようにする
        // TODO: タイトルがファイル名と行数じゃわかりづらい
        // TODO: permanent linkをつけたい
        (format!("{}:{}", self.file, self.line_number), self.matched.clone())
    }
}
