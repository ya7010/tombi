#!/usr/bin/env node

// This script is for testing the actual installation process
// When executed, it runs the installation script and downloads the binary

const { execSync } = require('child_process');
const path = require('path');

console.log('🧪 Testing installation process...');

try {
  // Execute installation script
  execSync('node scripts/install.js', {
    stdio: 'inherit',
    cwd: path.join(__dirname, '..')
  });

  console.log('\n✅ Test completed. Installation completed successfully!');
  console.log('You can test the binary with the following command:');
  console.log('  bin/tombi --version');
} catch (error) {
  console.error('\n❌ Test failed');
  process.exit(1);
}
