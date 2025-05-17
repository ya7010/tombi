#!/usr/bin/env node

const axios = require('axios');
const fs = require('fs');
const path = require('path');
const tar = require('tar');
const os = require('os');
const { execSync } = require('child_process');

// GitHubリリースからバイナリをダウンロードする設定
const REPO_URL = 'https://github.com/tombi-toml/tombi';
const VERSION = '0.1.0'; // リリースバージョン
const BINARY_NAME = 'tombi';
const BIN_PATH = path.join(__dirname, '..', 'bin');

async function main() {
  try {
    // プラットフォームとアーキテクチャを判定
    const platform = getPlatform();
    const arch = getArch();

    console.log(`🦅 tombi v${VERSION} for ${platform}-${arch} をインストールしています...`);

    // ダウンロードURLを構築
    const downloadUrl = getDownloadUrl(platform, arch);

    // binディレクトリを作成
    if (!fs.existsSync(BIN_PATH)) {
      fs.mkdirSync(BIN_PATH, { recursive: true });
    }

    // tarballをダウンロードして展開
    await downloadAndExtract(downloadUrl);

    // バイナリに実行権限を付与
    const binaryPath = path.join(BIN_PATH, BINARY_NAME);
    fs.chmodSync(binaryPath, 0o755);

    console.log(`✅ tombi v${VERSION} のインストールが完了しました！`);
    console.log(`バイナリの場所: ${binaryPath}`);

  } catch (error) {
    console.error('❌ インストール中にエラーが発生しました:');
    console.error(error.message || error);
    process.exit(1);
  }
}

function getPlatform() {
  const platform = os.platform();

  switch (platform) {
    case 'darwin':
      return 'macos';
    case 'win32':
      return 'windows';
    case 'linux':
      return 'linux';
    default:
      throw new Error(`サポートされていないプラットフォーム: ${platform}`);
  }
}

function getArch() {
  const arch = os.arch();

  switch (arch) {
    case 'x64':
      return 'x86_64';
    case 'arm64':
      return 'aarch64';
    default:
      throw new Error(`サポートされていないアーキテクチャ: ${arch}`);
  }
}

function getDownloadUrl(platform, arch) {
  // GitHubリリースからダウンロードするURLを構築
  // 例: https://github.com/tombi-toml/tombi/releases/download/v0.1.0/tombi-v0.1.0-x86_64-apple-darwin.tar.gz

  let targetTriple;

  if (platform === 'macos') {
    targetTriple = `${arch}-apple-darwin`;
  } else if (platform === 'linux') {
    targetTriple = `${arch}-unknown-linux-gnu`;
  } else if (platform === 'windows') {
    targetTriple = `${arch}-pc-windows-msvc`;
  } else {
    throw new Error(`サポートされていないプラットフォーム: ${platform}`);
  }

  return `${REPO_URL}/releases/download/v${VERSION}/tombi-v${VERSION}-${targetTriple}.tar.gz`;
}

async function downloadAndExtract(url) {
  const tempFile = path.join(os.tmpdir(), `tombi-${VERSION}.tar.gz`);

  try {
    // ダウンロード
    console.log(`📦 ${url} からバイナリをダウンロードしています...`);
    const response = await axios({
      method: 'get',
      url: url,
      responseType: 'stream'
    });

    const writer = fs.createWriteStream(tempFile);
    response.data.pipe(writer);

    await new Promise((resolve, reject) => {
      writer.on('finish', resolve);
      writer.on('error', reject);
    });

    // 展開
    console.log('📂 バイナリを展開しています...');
    await tar.extract({
      file: tempFile,
      cwd: BIN_PATH
    });

  } catch (error) {
    throw new Error(`ダウンロードまたは展開に失敗しました: ${error.message}`);
  } finally {
    // 一時ファイルの削除
    if (fs.existsSync(tempFile)) {
      fs.unlinkSync(tempFile);
    }
  }
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
