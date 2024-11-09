![Logo](./docs/images/tombi.svg)

Tombi is a toolkit for TOML; providing a formatter/linter and LSP server.

## TODO
### Features
- [ ] コメントの完全なサポート
    - [x] トップブロックのコメントサポート
    - [ ] Array のコメントサポート
    - [ ] Inline Table のコメントサポート
- [ ] inf/nan のキーワード化
- [ ] diagnostics のエラーメッセージの範囲の改善
- [ ] Document のサポート。
- [ ] linter のサポート
- [ ] リリース
- [ ] JSON Schema のサポート
- [ ] syntax tree 側での行・列情報のサポート

### Bugs
- [ ] Local Date 型が誤って IntegerDec としてパースされる
- [ ] Keys に float や int を使った場合、誤ってパースされる
