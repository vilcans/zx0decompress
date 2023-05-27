# zx0dec

A command-line application for decompressing files compressed with
Einar Saukas' [ZX0 compression format](https://github.com/einar-saukas/ZX0).

Normally, you use the ZX0 format to save space on 8 bit platforms like the ZX Spectrum.
You compress the data on a modern computer, and use a decompressor implemented in assembly language on the target platform.

For some use cases, like build tools and other utilities, it may still be useful to have a decompressor on your workstation.
There is already a [command-line decompressor written in C](https://github.com/einar-saukas/ZX0)
but as I wanted a Rust library I also implemented an application while I was at it.

## Installation

You need a working Rust environment, which you can install with [Rustup](https://rustup.rs/).

The command-line application is named `zx0dec`. Install it with:

```sh
cargo install zx0dec
```

## Usage

You can get help with the command `zx0dec --help`:

```
Usage: zx0dec [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Compressed file to read from
  <OUTPUT>  File to write the decompressed data to

Options:
  -c, --classic                 Treat input as old (v1) file format
  -m, --max-output-size <SIZE>  Maximum number of decompressed bytes to write
  -h, --help                    Print help
```

## Usage as a library

See [zx0decompressor](https://github.com/vilcans/zx0decompress/tree/master/lib).

## Links

* [crates.io](https://crates.io/crates/zx0dec)
* [Repository](https://github.com/vilcans/zx0decompress/)
