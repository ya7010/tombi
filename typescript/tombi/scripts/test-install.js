#!/usr/bin/env node

// このスクリプトは、実際のインストールプロセスをテストするためのものです
// 実行すると、インストールスクリプトが実行され、バイナリがダウンロードされます

const { execSync } = require('child_process');
const path = require('path');

console.log('🧪 インストールプロセスをテストしています...');

try {
  // インストールスクリプトを実行
  execSync('node scripts/install.js', {
    stdio: 'inherit',
    cwd: path.join(__dirname, '..')
  });

  console.log('\n✅ テスト完了。インストールは正常に完了しました！');
  console.log('次のコマンドでバイナリをテスト実行できます:');
  console.log('  bin/tombi --version');
} catch (error) {
  console.error('\n❌ テストに失敗しました');
  process.exit(1);
}
