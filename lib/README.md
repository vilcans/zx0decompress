# zx0decompress

A Rust library to decompress files compressed with
Einar Saukas' [ZX0 compression format](https://github.com/einar-saukas/ZX0).

Normally, you use the ZX0 format to save space on 8 bit platforms like the ZX Spectrum.
You compress the data on a modern computer, and use a decompressor implemented in assembly language on the target platform.

For some use cases, like build tools and other utilities, it may still be useful to have a decompressor on your workstation. That's why I created this library.

I also implemented a command-line application called [`zx0dec`](https://crates.io/crates/zx0dec) based on this library.

## Usage

Add the crate to your project:

```
cargo add zx0decompress
```

Call `zx0decompress::decompress` with any object that implements [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html).

You may also want to specify settings other than the default. For that, call `zx0decompress::decompress_with_settings`.

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

## Testing

In [lib/tests](tests) there are compressed files (compressed with [zx0-rs](https://crates.io/crates/zx0)) and their corresponding uncompressed files. A test case verifies that `zx0decompress` decompresses correctly.

In the [fuzz](../fuzz) directory there are fuzz tests that can be run with [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz):

```
cargo fuzz run fuzz_decompress
```

## Links

* [crates.io](https://crates.io/crates/zx0decompress)
* [API documentation](https://docs.rs/zx0decompress/latest/zx0decompress/)
* [Repository](https://github.com/vilcans/zx0decompress/)
