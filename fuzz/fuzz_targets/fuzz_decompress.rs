#![no_main]

use libfuzzer_sys::fuzz_target;
use zx0decompress::{self, DecompressError, Settings};

fuzz_target!(|data: &[u8]| {
    /// Without a maximum size, the fuzzer can easily generate content that cause out of memory errors,
    const MAX_OUTPUT_SIZE: usize = 431; // just a number, but prime as it may be harder to get right
    let settings = Settings {
        max_output_size: MAX_OUTPUT_SIZE,
        ..Default::default()
    };
    match zx0decompress::decompress_with_settings(&mut data.as_ref(), settings) {
        Ok(data) => {
            if data.len() > MAX_OUTPUT_SIZE {
                panic!(
                    "Decompressed data is {} bytes, but max_output_size was set to {}",
                    data.len(),
                    MAX_OUTPUT_SIZE
                );
            }
        }
        Err(DecompressError::InvalidLength) => {}
        Err(DecompressError::InvalidOffset) => {}
        Err(DecompressError::TruncatedInput) => {}
        Err(e) => {
            panic!("Failed: {e:?}");
        }
    }
});
