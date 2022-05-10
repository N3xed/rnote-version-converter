# Rnote v0.4 files converter

A simple CLI application to convert [Rnote](https://github.com/flxzt/rnote) `v0.4` files
to `v0.5` files.


## Install

- [Install Rust](https://www.rust-lang.org/tools/install).
- Run `cargo install --git https://github.com/N3xed/rnote-version-converter`

## Usage

```shell
rnote-version-converter

USAGE:
    rnote-version-converter [OPTIONS] <FILE> [DEST_FILE]

ARGS:
    <FILE>         The file to convert
    <DEST_FILE>    The destination file or `<file>-upgraded.rnote` per default

OPTIONS:
    -h, --help                         Print help information
    -i, --input-type <INPUT_TYPE>      [default: v0-4] [possible values: v0-4, v0-5]
    -o, --output-type <OUTPUT_TYPE>    [default: latest] [possible values: json, pretty-json, v0-4,
                                       v0-5, latest]
```

So to convert a Rnote v4.0 file run 
```bash
$ rnote-version-converter <file> [<converted-file>]
```

`converted-file` is optional and set to `<file>-upgraded.rnote` per default.