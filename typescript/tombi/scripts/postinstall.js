#!/usr/bin/env node

const axios = require('axios');
const fs = require('fs');
const path = require('path');
const tar = require('tar');
const os = require('os');
const { execSync } = require('child_process');

// Configuration for downloading binary from GitHub releases
const REPO_URL = 'https://github.com/tombi-toml/tombi';
const VERSION = '0.1.0'; // Release version
const BINARY_NAME = 'tombi';
const BIN_PATH = path.join(__dirname, '..', 'bin');

async function main() {
  try {
    // Determine platform and architecture
    const platform = getPlatform();
    const arch = getArch();

    console.log(`🦅 Installing tombi v${VERSION} for ${platform}-${arch}...`);

    // Build download URL
    const downloadUrl = getDownloadUrl(platform, arch);

    // Create bin directory
    if (!fs.existsSync(BIN_PATH)) {
      fs.mkdirSync(BIN_PATH, { recursive: true });
    }

    // Download and extract tarball
    await downloadAndExtract(downloadUrl);

    // Add execute permission to binary
    const binaryPath = path.join(BIN_PATH, BINARY_NAME);
    fs.chmodSync(binaryPath, 0o755);

    console.log(`✅ Installation of tombi v${VERSION} completed!`);
    console.log(`Binary location: ${binaryPath}`);

  } catch (error) {
    console.error('❌ An error occurred during installation:');
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
      throw new Error(`Unsupported platform: ${platform}`);
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
      throw new Error(`Unsupported architecture: ${arch}`);
  }
}

function getDownloadUrl(platform, arch) {
  // Build download URL from GitHub releases
  // Example: https://github.com/tombi-toml/tombi/releases/download/v0.1.0/tombi-v0.1.0-x86_64-apple-darwin.tar.gz

  let targetTriple;

  if (platform === 'macos') {
    targetTriple = `${arch}-apple-darwin`;
  } else if (platform === 'linux') {
    targetTriple = `${arch}-unknown-linux-gnu`;
  } else if (platform === 'windows') {
    targetTriple = `${arch}-pc-windows-msvc`;
  } else {
    throw new Error(`Unsupported platform: ${platform}`);
  }

  return `${REPO_URL}/releases/download/v${VERSION}/tombi-v${VERSION}-${targetTriple}.tar.gz`;
}

async function downloadAndExtract(url) {
  const tempFile = path.join(os.tmpdir(), `tombi-${VERSION}.tar.gz`);

  try {
    // Download
    console.log(`📦 Downloading binary from ${url}...`);
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

    // Extract
    console.log('📂 Extracting binary...');
    await tar.extract({
      file: tempFile,
      cwd: BIN_PATH
    });

  } catch (error) {
    throw new Error(`Failed to download or extract: ${error.message}`);
  } finally {
    // Clean up temporary file
    if (fs.existsSync(tempFile)) {
      fs.unlinkSync(tempFile);
    }
  }
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
