<div align="center">
    <img src="https://raw.githubusercontent.com/tombi-toml/tombi/refs/heads/main/docs/images/tombi.svg" alt="Logo">
</div>

[![Marketplace Version](https://vsmarketplacebadges.dev/version/yassun7010.tombi.png?label=VS%20Code%20Marketplace&logo=visual-studio-code "Current Release")](https://marketplace.visualstudio.com/items?itemName=yassun7010.tombi)
[![CI VSCode Extensions](https://github.com/tombi-toml/tombi/actions/workflows/ci_vscode.yml/badge.svg)](https://github.com/tombi-toml/tombi/actions/workflows/ci_vscode.yml)
[![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)

Tombi (鳶) is a toolkit for TOML; providing a formatter/linter and language server.

## Differences from Taplo

[Taplo](https://github.com/tamasfe/taplo) is already famous as a TOML Language Server.
However, I rewrote a new Language Server for the following purposes.

- Support for TOML v1.1.0
- Formatter inspired by Python's [Black](https://github.com/psf/black) (Support for [magic trailing comma](https://black.readthedocs.io/en/stable/the_black_code_style/current_style.html#the-magic-trailing-comma))

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

## NOTE
### TOML v1.1.0
[TOML v1.1.0](https://github.com/toml-lang/toml/issues/928) has not been released.

The formatter of Tombi will show its true power in `v1.1.0`, but be careful.

Many existing systems ([Cargo.toml](https://doc.rust-lang.org/cargo/reference/manifest.html), [pyproject.toml](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/)) are `v1.0.0`, and the default TOML version of Tombi is also `v1.0.0`.

Tombi will provide support for migrating from `v1.0.0` to `v1.1.0`,
but it is not yet fully functional.

### toml-test
[toml-test](https://github.com/toml-lang/toml-test) is almost supported. Once all tests pass, it will be included in CI for monitoring.

### JSON Schema
[TOML itself may add schema specifications](https://github.com/toml-lang/toml/issues/792),
but like Taplo, Tombi is also trying to add validation functions to the linter that support [JSON Schema](https://json-schema.org/).

Currently, we are considering adding special information called `x-tombi-*` to JSON Schema.

- `x-tombi-toml-version`: Required to automatically determine which system supports `v1.1.0` (implemented)
- `x-tombi-table-key-order-by`: For automatically sorting tables like `[dependencies]` (not implemented)

We need to request additional modifications to the [JSON Schema Store](https://www.schemastore.org/json/),
but Tombi has not yet gained market share, so it will be much later.
