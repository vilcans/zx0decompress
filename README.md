# zx0decompress

A Rust library ([`zx0decompress`](https://crates.io/crates/zx0decompress))
and command-line application ([`zx0dec`](https://crates.io/crates/zx0dec))
to decompress files compressed with
Einar Saukas' [ZX0 compression format](https://github.com/einar-saukas/ZX0).

Normally, you use the ZX0 format to save space on 8 bit platforms like the ZX Spectrum.
You compress the data on a modern computer, and use a decompressor implemented in assembly language on the target platform.

For some use cases, like build tools and other utilities, it may still be useful to have a decompressor on your workstation. That's why I created this library.

## Usage as command-line application

See [zx0dec](cli).

## Usage as a library

See [zx0decompress](lib).

## Compression

To compress to ZX0, there is the [zx0 crate](https://crates.io/crates/zx0) by Emil Loer.

There is also the original [C code](https://github.com/einar-saukas/ZX0) by Einar Saukas.

Both of these repos contain a library and command-line application that compresses, and the C repo also has a decompressor.

## Credits

This crate made by [Martin Vilcans](https://www.librador.com).

Original ZX0 compression algorithm and C implementation (which served as a reference for the Rust implementation) by [Einar Saukas](https://github.com/einar-saukas).
