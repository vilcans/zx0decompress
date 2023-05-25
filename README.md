# zx0decompress

A Rust library and command-line application to decompress files compressed with
Einar Saukas' [ZX0 compression format](https://github.com/einar-saukas/ZX0).

Normally, you use the ZX0 format to save space on 8 bit platforms like the ZX Spectrum.
You compress the data on a modern computer, and use a decompressor implemented in assembly language on the target platform.

For some use cases, like build tools and other utilities, it may still be useful to have a decompressor on your workstation. That's why I created this library.

## Usage as command-line application

The command-line application is named `zx0dec`.
Install it by cloning this repo, `cd` into it and run:

```sh
cargo install --path cli
```

Then you can get help with the command `zx0dec --help`:

```
Decompresses a file in ZX0 format

Usage: zx0dec <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Compressed file to read from
  <OUTPUT>  File to write the decompressed data to

Options:
  -h, --help  Print help
```

## Usage as a library

Just call `zx0decompress::decompress` with any object that implements [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html).

### Examples

Decompress from file into a `Vec<u8>`:

```rust
let mut source = std::fs::File::open(filename)?;
let content = zx0decompress::decompress(&mut source)?;
```

Decompress from a byte slice into a `Vec<u8>`:

```rust
let source = [
    0x1fu8, 0x41, 0x42, 0x52, 0x41, 0x20, 0xf6, 0xab, 0x43, 0x44, 0xf5, 0xf2, 0x55, 0x58,
];
let result = decompress(&mut source.as_ref()).unwrap();
assert_eq!(&result, b"ABRA ABRACADABRA");
```

## Compression

To compress to ZX0, there is the [zx0 crate](https://crates.io/crates/zx0) by Emil Loer.

There is also the original [C code](https://github.com/einar-saukas/ZX0) by Einar Saukas.

Both of these repos contain a library and command-line application that compresses, and the C repo also has a decompressor.

## Testing

In [lib/tests](lib/tests) there are compressed files (compressed with [zx0-rs](https://crates.io/crates/zx0)) and their corresponding uncompressed files. A test case verifies that `zx0decompress` decompresses correctly.
