use anyhow::{Context, Result};
use grep::{
    matcher::Matcher,
    regex::RegexMatcher,
    searcher::{sinks::UTF8, SearcherBuilder, BinaryDetection},
};
use ignore::Walk;
use std::collections::HashMap;
use strfmt::strfmt;

#[derive(Debug)]
pub struct Postpone {
    pub file: String,
    pub line_number: u64,
    pub line: String,
    pub label: String,
}

impl Postpone {
    pub fn search(pattern: &str, ignore_file: &[String]) -> Result<Vec<Self>> {
        let matcher = RegexMatcher::new_line_matcher(pattern)?;
        let mut result = Vec::new();

        // TODO: layonとか使って並列化したい
        Walk::new(".")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .filter(|e| !e.path().to_str().unwrap().starts_with(".git"))
            .filter(|e| {
                !ignore_file
                    .iter()
                    .any(|ignore| e.path().to_str().unwrap().contains(ignore))
            })
            .into_iter()
            .try_for_each(|entry| {
                let path = entry.path();
                let path = path.to_str().unwrap();

                let mut searcher = SearcherBuilder::new()
                    .line_number(true)
                    .binary_detection(BinaryDetection::quit(b'\x00'))
                    .build();
                searcher
                    .search_path(
                        &matcher,
                        path,
                        UTF8(|line_number, line| {
                            let mut label = String::new();
                            if let Ok(Some(mat)) = matcher.find(line.as_bytes()) {
                                let (start, end) = (mat.start(), mat.end());
                                label = line[start..end].to_string();
                                result.push(Postpone {
                                    file: path.to_string(),
                                    line_number,
                                    line: line[end..].to_string(),
                                    label,
                                })
                            } else {
                                result.push(Postpone {
                                    file: path.to_string(),
                                    line_number,
                                    line: line.to_string(),
                                    label,
                                })
                            }
                            Ok(true)
                        }),
                    )
                    .with_context(|| format!("failed to search {}", path))
            })?;

        Ok(result)
    }

    pub fn to_issue(&self, title_format: &str, body_format: &str) -> Result<(String, String)> {
        // TODO: permanent linkをつけたい
        let mut vars = HashMap::new();
        vars.insert("file".to_string(), self.file.clone());
        vars.insert("line_number".to_string(), self.line_number.to_string());
        vars.insert("label".to_string(), self.label.to_string());
        vars.insert("line".to_string(), self.line.trim().to_string());
        let title = strfmt(&title_format, &vars)?;
        let body = strfmt(&body_format, &vars)?;
        Ok((title, body))
    }
}
