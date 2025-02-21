# Serve Static

HTTP server that serves files from the specified directory.

Written in Rust.

## Demo files

```shell
cargo run -- -d html -p 8080
```

Then visit the following urls:
- http://127.0.0.1:8080
- http://127.0.0.1:8080/index.html
- http://127.0.0.1:8080/style.css
- http://127.0.0.1:8080/script.js

## Build release mode

```shell
cargo build --release
```

Then copy the executable from `target/release/serve-static` to your $PATH.

```shell
serve-static -c -d html -p 8080
```

## Usage

```text
Usage: serve-static [OPTIONS] --dir <DIR>

Options:
  -c, --cors
      --public       Expose to local network
  -d, --dir <DIR>
  -p, --port <PORT>  Local port [default: 3000]
  -h, --help         Print help
  -V, --version      Print version
```
