# Developer Documentation

## toml-test

To test if it passes [toml-test](https://github.com/toml-lang/toml-test), run the following.

```sh
# Please first install toml-test
$ go install github.com/toml-lang/toml-test/cmd/toml-test@latest

# decode test
$ toml-test cargo decode

# encode test
$ toml-test cargo encode -encode
```
