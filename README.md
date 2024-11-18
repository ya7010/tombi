![Logo](./docs/images/tombi.svg)

Tombi (鳶) is a toolkit for TOML; providing a formatter/linter and Language server.

## TODO
### Milestone 1
- [x] コメントの完全なサポート
    - [x] トップブロックのコメントサポート
    - [x] Array のコメントサポート
    - [x] Inline Table のコメントサポート
- [x] lexer の自作
- [ ] diagnostics のエラーメッセージの範囲の改善
- [ ] Document のサポート。
- [ ] linter のサポート
- [ ] リリース

### Milestone 2
- [ ] JSON Schema のサポート

### Milestone 3
- [ ] Linter の機能強化
- [ ] Formatter のオプションサポート

### Refactoring
- [x] syntax tree 側での行・列情報のサポート

### Bugs
- [x] Local Date 型が誤って IntegerDec としてパースされる
- [ ] Keys に float や int を使った場合、誤ってパースされる
    - [ ] 3.14 を keys に使った場合、3 と 14 の key としてパースされる
    - [ ] 3 を keys に使った場合、3 の key としてパースされる
    - [ ] inf, nan を keys に使った場合、key としてパースされる
