# @tombi/cli

🦅 tombiのTOMLツールキットのRustバイナリインストーラー

## 概要

このパッケージは、Rust製のtombiバイナリをnpmを通じてインストールするためのものです。インストール時に自動的にプラットフォームに適したバイナリがダウンロードされます。

## インストール

```
npm install -g @tombi/cli
```

インストール後、`tombi`コマンドがグローバルに使用可能になります。

## 使い方

### フォーマット

TOMLファイルをフォーマットします：

```
tombi format path/to/file.toml
```

ファイルを直接編集するには `-i` オプションを使用します：

```
tombi format -i path/to/file.toml
```

### リント

TOMLファイルをリントします：

```
tombi lint path/to/file.toml
```

可能な場合は問題を自動修正するには `--fix` オプションを使用します：

```
tombi lint --fix path/to/file.toml
```

## サポートされているプラットフォーム

- macOS (x86_64, aarch64)
- Linux (x86_64)
- Windows (x86_64)

## ライセンス

MIT 
