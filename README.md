![Logo](./docs/images/tombi.svg)

[![Marketplace Version](https://vsmarketplacebadges.dev/version/yassun7010.tombi.png?label=VS%20Code%20Marketplace&logo=visual-studio-code "Current Release")](https://marketplace.visualstudio.com/items?itemName=yassun7010.tombi)
[![CI VSCode Extensions](https://github.com/tombi-toml/tombi/actions/workflows/ci_vscode.yml/badge.svg)](https://github.com/tombi-toml/tombi/actions/workflows/ci_vscode.yml)
[![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)

Tombi (鳶) is a toolkit for TOML; providing a formatter/linter and language server.

## Differences from Taplo

[Taplo](https://github.com/tamasfe/taplo) is already famous as a TOML Language Server.
However, I rewrote a new Language Server for the following purposes.

- Support for TOML v1.1.0
- Formatter inspired by Python's [Black](https://github.com/psf/black) (Support for magic comma)

## Usage
### Formatting
```sh
tombi format
```

### Linting
```sh
tombi lint
```

## Config file
The documentation site has not been built yet,
but you can control the behavior with a configuration file called
[tombi.toml](https://github.com/tombi-toml/tombi/blob/main/tombi.toml).

## TODO
The formatter/linter is almost complete, but support for JsonSchema is incomplete.
