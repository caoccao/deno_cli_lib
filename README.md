# Deno CLI Lib

Deno CLI Lib is Deno CLI as a Rust Library. It's designed for embedding Deno CLI into Rust applications or libraries.

Why do I need this project?

1. Deno doesn't expose Deno CLI as a Rust library.
2. There are some CLI features no available at `deno_runtime`, `deno_core`, `deno_ops`.

## Usage

### Patch Deno

Run a patch script that patches the Deno repository to expose Deno as a Rust library. Here are the steps.

- Clone Deno - The Deno repository is required to be cloned locally.

```sh
git clone https://github.com/denoland/deno.git
```

- Clone Deno CLI Lib - This repository is required to be cloned with Deno side by side.

```sh
git clone https://github.com/caoccao/deno_cli_lib.git
```

- Run the Patch Script - The patch script adds `lib.rs` to Deno CLI and update `Cargo.toml` by adding section `[lib]`.

```sh
cd deno_cli_lib
deno run --allow-all scripts/ts/patch_deno.ts
```

- Wait for the Patch Script to Complete - The patch takes less than 1s.

### Reference Deno

- Add the following dependency to your `Cargo.toml`.

```ini
[dependencies]
deno = { path = "/path/to/deno/cli" }
```

- Enjoy!

```rust
use deno::deno_core::*;
fn main() {
  println!("V8 version: {}", v8_version());
}

// V8 version: *.*.*.*
```

## License

[APACHE LICENSE, VERSION 2.0](LICENSE)
