# Postpone Bot

## Usage

```bash
postpone bot

Usage: ppb [OPTIONS]

Options:
  -c, --config <CONFIG>
          config file
      --organization <ORGANIZATION>
          organization
      --repository <REPOSITORY>
          repository
      --token <TOKEN>
          GitHub token
  -l, --listup
          list up postpones
      --annotation-labels <ANNOTATION_LABELS>...
          annotation labels default: ["TODO", "FIXME"]
      --title-format <TITLE_FORMAT>
          title format you can use following variables {label} {file} {line_number} {line} default: "Postpone: {label} {file} {line_number}" [default: "[Postpone] {label}: {line}"]
      --body-format <BODY_FORMAT>
          body format you can use following variables {label} {file} {line_number} {line} default: "Postpone: {label}\n\n{file}:{line_number}\n\n```\n{line}\n```" [default: "\nPostpone: {label}\n\n{file}:{line_number}\n\n```\n{line}\n```\n"]
      --dry-run
          dry run will not create issues default: false
  -h, --help
          Print help
  -V, --version
          Print version
```

## Config

```yaml
organization: n01e0
repository: ppb
token: ghp_XXX
annotation_labels:
 - "TODO:* "
 - "FIXME:* "
 - "BUG:* "
 - "HACK:* "
 - "#\\[allow\\([a-zA-Z_].+\\)\\]"
title_format: "{label}: {file} {line}"
body_format: "{line}"
 ```

## Actions

```yaml
name: Postpone Bot
on:
  push:
    branches:
      - main

permissions:
  contents: read
  issues: write

jobs:
  ppb:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: n01e0/ppb@release
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          config: "postpone.yml"
```
