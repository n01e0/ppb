# Postpone Bot

## Usage

```bash
postpone bot

Usage: ppb [OPTIONS]

Options:
  -c, --config <CONFIG>                        config file
      --org <ORGANIZATION>                     organization
      --repository <PROJECT>                      repository
      --token <TOKEN>                          GitHub token
      --annotation-labels <ANNOTATION_LABELS>  annotation labels default: ["TODO", "FIXME"]
  -h, --help                                   Print help
  -V, --version                                Print version
```

## Config

```yaml
organization: n01e0
repository: ppb
token: ghp_XXX
annotation_labels:
 - TODO
 - FIXME
 - BUG
 - HACK
 - "#\\[allow\\]"
 ```
