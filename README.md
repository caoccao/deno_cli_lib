# Deno CLI Lib

Deno CLI Lib is Deno CLI as a Rust Library. It's designed for embedding Deno CLI into Rust applications or libraries.

Why do I need this project?

1. Deno doesn't expose Deno CLI as a Rust library.
2. There are some CLI features no available at `deno_runtime`, `deno_core`, `deno_ops`.

## Usage

### Patch Deno

- Clone Deno

```sh
git clone https://github.com/denoland/deno.git
```

- Clone Deno CLI Lib

```sh
git clone https://github.com/caoccao/deno_cli_lib.git
```

- Run the patch script

```sh
cd deno_cli_lib
deno run --allow-all scripts/ts/patch_deno.ts
```

- Wait for the patch script to complete

## License

[APACHE LICENSE, VERSION 2.0](LICENSE)
