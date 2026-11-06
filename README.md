# Ziren Wasm Stark Verifier

WebAssembly bindings for the [Ziren](https://github.com/ProjectZKM/Ziren) verifier.

## Overview

This package is a fork of [ziren-wasm-verifier](https://github.com/ProjectZKM/ziren-wasm-verifier) by ProjectZKM. It has been modified and published to perform proof verification in-browser on Ethproofs. The WASM wrapper exposes the `zkm-verifier` crate's STARK, Groth16 and Plonk verifiers to web browsers and Node.js.

Version `0.2.x` verifies proofs generated with Ziren `v2.x`. Proofs from Ziren `v1.x` require `0.1.x`.

## Usage

### Installation

```bash
npm install @ethproofs/ziren-wasm-stark-verifier
```

### Bundlers (Vite, webpack, Next.js, ...)

The package is an ES module built with wasm-pack's `bundler` target. The WASM is loaded and initialized on import, so there is no `init()` to call.

```typescript
import { main, verify_stark } from '@ethproofs/ziren-wasm-stark-verifier';

main(); // Install the panic hook for readable errors

const isValid = verify_stark(proofBytes, vkBytes);
```

### Node.js

Node.js needs the `--experimental-wasm-modules` flag to import the package's `.wasm` file:

```bash
node --experimental-wasm-modules app.mjs
```

```javascript
import { main, verify_stark } from '@ethproofs/ziren-wasm-stark-verifier';

main();
const isValid = verify_stark(proofBytes, vkBytes);
```

The package is ESM-only; `require()` is not supported.

### API

All functions take `Uint8Array` arguments and return `true` if the proof is valid.

| Function | Description |
| --- | --- |
| `verify_stark(proof, vk)` | Verifies a compressed STARK proof against a bincode-serialized verifying key. Does not check public values. |
| `verify_stark_with_public_inputs(proof, publicInputs, vk)` | Same as `verify_stark`, and also checks that `publicInputs` match the values committed in the proof. |
| `verify_groth16(proof, publicInputs, vkHash)` | Verifies a Groth16 proof. `vkHash` is the program's `vk.bytes32()` hex string. |
| `verify_plonk(proof, publicInputs, vkHash)` | Verifies a Plonk proof. `vkHash` is the program's `vk.bytes32()` hex string. |

## Development

### Prerequisites

- A recent [Rust nightly](https://rust-lang.github.io/rustup/concepts/channels.html) toolchain (`rustup update nightly`). Ziren's dependencies regularly require recently stabilized features, so an outdated nightly will fail to compile.
- The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown --toolchain nightly`
- [wasm-pack](https://github.com/drager/wasm-pack)

### Building

```bash
npm install

# Build the published bundler package into pkg/
npm run build

# Build all targets (pkg/, pkg-node/, pkg-web/)
npm run build:all
```

### Node.js Test

```bash
npm run test:node
```

This verifies the STARK proof in `binaries/` and the fibonacci STARK, Groth16 and Plonk fixtures in `example/json/`. It exits with a non-zero code if any proof fails to verify.

### Browser Test

```bash
npm run test
```

This starts a local HTTP server at `http://localhost:8080` with a browser page for verifying a STARK proof and verification key, such as the files in `binaries/`.

**Note:** WASM must be served over HTTP, so the page cannot be opened directly from the filesystem.

### Regenerating Fixtures

`example/guest` is a fibonacci Ziren program, and `example/host` generates proofs for it into `example/binaries/` and `example/json/`. After upgrading Ziren, regenerate the fixtures from the `example/host` directory:

```bash
cargo run --release -- --mode stark --prove
cargo run --release -- --mode groth16 --prove
cargo run --release -- --mode plonk --prove
```

Without `--prove`, the host loads the existing proofs from `example/binaries/` and rewrites the JSON fixtures.

### Publishing

```bash
npm run build
npm publish
```
