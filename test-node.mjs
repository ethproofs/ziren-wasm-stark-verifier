import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import { main, verify_stark } from './pkg-node/ziren_wasm_stark_verifier.js';

async function testVerification() {
  console.log('🧪 Testing Ziren WASM STARK Verifier\n');

  try {
    main();
    console.log('✅ WASM module initialized\n');

    // Load proof and verification key
    const proofPath = path.join(__dirname, 'binaries', 'zkm-eth-proof.bin');
    const vkPath = path.join(__dirname, 'binaries', 'zkm-vk.bin');

    console.log('\nLoading proof and verification key...');
    const proofBytes = fs.readFileSync(proofPath);
    const vkBytes = fs.readFileSync(vkPath);

    console.log(`  Proof size: ${proofBytes.length} bytes`);
    console.log(`  VK size: ${vkBytes.length} bytes`);

    // Test verification
    console.log('\n🔍 Verifying proof...');
    const start = performance.now();
    const result = verify_stark(proofBytes, vkBytes);
    const end = performance.now();
    console.log(`✅ Proof verification result: ${result}`);

    console.log('\n📊 Verification Summary:');
    console.log(`  STARK proof: ${result ? '✅ VALID' : '❌ INVALID'}`);
    console.log(`  Time taken: ${end - start} milliseconds`);
  } catch (error) {
    console.error('❌ Error during verification:', error.message);
    console.error('Stack trace:', error.stack);
    process.exit(1);
  }
}

testVerification();
