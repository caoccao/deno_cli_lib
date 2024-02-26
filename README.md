# Deno CLI Lib

Deno CLI Lib is Deno CLI as a Rust Library. It's designed for embedding Deno CLI into Rust applications or libraries.

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
cd deno_cli_lib/cli
deno run --allow-all patch_deno.ts
```

- Wait for the patch script to complete

## License

[APACHE LICENSE, VERSION 2.0](LICENSE)
