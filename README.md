# Ziren Wasm Stark Verifier

WebAssembly bindings for the Ziren STARK verifier.

## Overview

This package is a fork of [ziren-wasm-verifier](https://github.com/ProjectZKM/ziren-wasm-verifier) by ProjectZKM. It has been modified and published to perform proof verification in-browser on Ethproofs. The WASM wrapper exposes `verify_proof` from `zkm-verifier`'s `StarkVerifier`, allowing STARK proof verification in web browsers and Node.js environments.

## Usage

### Installation

```bash
npm install @ethproofs/ziren-wasm-stark-verifier
```

### React Integration

```typescript
import init, { main, verify_stark } from '@ethproofs/ziren-wasm-stark-verifier';

await init(); // Initialize WASM (if needed)
main(); // Initialize panic hook

// Verify a proof
const isValid = verify_stark(proofBytes, vkBytes);
```

For complete React integration examples, see [REACT_INTEGRATION.md](./REACT_INTEGRATION.md).

### Node.js Usage

```javascript
const { main, verify_stark } = require('@ethproofs/ziren-wasm-stark-verifier');

// The Node.js version initializes automatically

main(); // Initialize panic hook
const result = verify_stark(proofBytes, vkBytes);
```

## Testing

### Installation

```bash
npm install
```

### Prerequisites

- [Rust (Nightly)](https://pico-docs.brevis.network/getting-started/installation#rust-toolchain)
- [wasm-pack](https://github.com/drager/wasm-pack)

### Building

```bash
# Build for all targets
npm run build:all
```

### Node.js Example

```bash
npm run test:node
```

This runs the Node.js example that loads proof and verification key files from the filesystem and verifies them using the KoalaBear field configuration.

### Browser Example

```bash
npm run test
```

This starts a local HTTP server at `http://localhost:8080` with a browser example that demonstrates:

- Loading the WASM module in a browser environment
- File upload interface for proof and verification key files
- Interactive STARK proof verification
- Performance metrics and detailed logging
- Error handling and user feedback

The browser example provides a complete UI for testing the WASM verifier with drag-and-drop file selection and real-time verification results.

**Note:** The browser example requires files to be served over HTTP due to WASM CORS restrictions. The included server script handles this automatically.
