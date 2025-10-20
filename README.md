# Totp Auth

Secure multi-seed TOTP generator and verifier with WebAssembly support.

## Description

`totp-auth` is a Rust library for generating and verifying Time-based One-Time Password (TOTP) tokens using multiple seeds. It combines tokens from up to 6 seeds into a single hyphen-separated string for enhanced security. The library uses HMAC-SHA1 for cryptographic operations and supports configurable time windows and allowances for clock drifts during verification.

This library is suitable for authentication systems, APIs, and web applications. It includes optional WebAssembly (WASM) support via `wasm-bindgen`, allowing seamless integration into browser environments.

## Features

- **Multi-Seed TOTP Generation**: Combine tokens from multiple seeds into one formatted string (e.g., `123456-789012-...`).
- **Verification with Drift Allowance**: Verify tokens while accounting for time offsets (e.g., ±1 or more windows).
- **Unix Timestamp Utility**: Retrieve the current Unix time for time-based calculations.
- **WASM Compatibility**: Export functions for use in JavaScript/web environments.
- **Lightweight Dependencies**: Relies on `hmac` and `sha1` for core crypto, with `wasm-bindgen` as an optional feature.
- **Example Code**: Includes a simple verification example.

## Installation

```toml
[dependencies]
totp-auth = "1"
```

### Building for WebAssembly

To build the WASM module:

1. Install `wasm-pack` if not already installed: `cargo install wasm-pack`.
2. Run: `wasm-pack build --target web --features wasm`.
3. This generates a `pkg/` directory with the WASM binary and JavaScript bindings.

## Usage

### In Rust

Import the library and use the core functions:

```rust
use totp_auth::{current_unix_time, generate_combined_token, verify_combined_token};

fn main() {
    let seeds = ["seed1", "seed2", "seed3", "seed4", "seed5", "seed6"];
    let time = current_unix_time();
    let window = 30; // Time window in seconds (default for TOTP is 30s)

    // Generate a combined token
    let token = generate_combined_token(seeds, time, window);
    println!("Generated Token: {}", token);

    // Verify the token (allowing ±1 window for clock drift)
    let is_valid = verify_combined_token(seeds, time, &token, window, 2, "s");
    println!("Verification Result: {}", is_valid);
}
```

- `generate_combined_token(seeds: [&str; 6], time: u64, window: u64) -> String`: Generates a hyphen-separated token string.
- `verify_combined_token(seeds: [&str; 6], time: u64, token: &str, window: u64, allowed_windows: u32, unit: &str) -> bool`: Verifies the token, with `allowed_windows` specifying drift tolerance (e.g., 2 for ±1 window). `unit` is currently "s" for seconds.
- `current_unix_time() -> u64`: Returns the current Unix timestamp in seconds.

### In WebAssembly (JavaScript)

After building with `wasm-pack`, import the module in your JavaScript code:

```javascript
import init, { wasm_generate_combined_token, wasm_verify_combined_token } from './pkg/totp_auth.js';

async function run() {
    await init();

    const seeds = ["seed1", "seed2", "seed3", "seed4", "seed5", "seed6"];
    const time = Math.floor(Date.now() / 1000); // Current Unix time
    const window = 30;

    // Generate token
    const token = wasm_generate_combined_token(seeds, time, window);
    console.log("Generated Token:", token);

    // Verify token
    const isValid = wasm_verify_combined_token(seeds, time, token, window, 2, "s");
    console.log("Verification Result:", isValid);
}

run();
```

Note: The WASM functions (`wasm_generate_combined_token` and `wasm_verify_combined_token`) accept `Vec<String>` for seeds and other parameters matching the Rust API.

## Examples

The repository includes an example in `examples/verify.rs`:

```rust
use totp_auth::{current_unix_time, generate_combined_token, verify_combined_token};

fn main() {
    let seeds = ["a", "b", "c", "d", "e", "f"];
    let time = current_unix_time();
    let window = 15;
    let token = generate_combined_token(seeds, time, window);
    println!("Generated: {}", token);
    let ok = verify_combined_token(seeds, time, &token, window, 2, "s");
    println!("Verified: {}", ok);
}
```

To run the example: `cargo run --example verify`.

## Directory Structure

- `src/`: Core library code.
  - `lib.rs`: Entry point and WASM exports.
  - `totp.rs`: TOTP generation and verification logic.
- `examples/`: Usage demonstrations.
- `pkg/`: Generated WASM artifacts (after building).
- `target/`: Build outputs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or pull request on [GitHub](https://github.com/canmi21/totp-auth).
