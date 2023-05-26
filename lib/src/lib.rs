//! Decompress data in zx0 format.
//!
//! ## Examples
//!
//! Decompress from file into a `Vec<u8>`:
//!
//! ```no_run
//! # fn test() -> Result<(), zx0decompress::DecompressError> {
//! let filename = "something.zx0";
//! let mut source = std::fs::File::open(filename)?;
//! let content = zx0decompress::decompress(&mut source)?;
//! # Ok(())
//! # }
//! ```
//!
//! Decompress from a byte slice into a `Vec<u8>`:
//!
//! ```
//! let source = [
//!     0x1fu8, 0x41, 0x42, 0x52, 0x41, 0x20, 0xf6, 0xab, 0x43, 0x44, 0xf5, 0xf2, 0x55, 0x58,
//! ];
//! let result = zx0decompress::decompress(&mut source.as_ref()).unwrap();
//! assert_eq!(&result, b"ABRA ABRACADABRA");
//! ```

mod error;

pub use error::DecompressError;
use std::io::Read;

/// Decompression settings
#[derive(Clone)]
pub struct Settings {
    /// Decompress classic file format (v1)
    pub classic_mode: bool,
    /// Limit the output to this number of bytes.
    pub max_output_size: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            classic_mode: false,
            max_output_size: usize::MAX,
        }
    }
}

struct Context<'a> {
    source: &'a mut dyn Read,
    settings: Settings,
    last_offset: usize,
    /// Current bit data. The most significant bit contains the next bit that will be read.
    /// After the bit data, there is a bit set to 1 which is used as an end marker.
    bit_value: u16,
}

impl<'a> Context<'a> {
    pub fn new(source: &'a mut dyn Read, settings: Settings) -> Self {
        Self {
            source,
            settings,
            last_offset: 1,
            bit_value: 0x8000,
        }
    }
    /// Executes the next step of the compression. Returns the next state.
    fn next_step(&mut self, state: State, output: &mut Vec<u8>) -> Result<State, DecompressError> {
        match state {
            State::CopyLiterals => {
                let length = self.read_interlaced_elias_gamma(false)?;
                for _ in 0..length {
                    let literal = self.read_byte()?;
                    output.push(literal);
                }
                if self.read_bit()? {
                    Ok(State::CopyFromNewOffset)
                } else {
                    Ok(State::CopyFromLastOffset)
                }
            }
            State::CopyFromLastOffset => {
                let length = self.read_interlaced_elias_gamma(false)?;
                self.write_bytes(self.last_offset, length, output)?;
                if self.read_bit()? {
                    Ok(State::CopyFromNewOffset)
                } else {
                    Ok(State::CopyLiterals)
                }
            }
            State::CopyFromNewOffset => {
                let high = self.read_interlaced_elias_gamma(!self.settings.classic_mode)?;
                if high == 256 {
                    return Ok(State::Done);
                }
                let second_byte = self.read_byte()?;
                let offset = (high << 7)
                    .checked_sub((second_byte >> 1) as usize)
                    .ok_or(DecompressError::InvalidOffset)?;
                self.last_offset = offset;

                // Make the lowest bit in second byte be the next bit to read
                self.bit_value = (self.bit_value >> 1) | ((second_byte as u16) << 15);

                let length = self
                    .read_interlaced_elias_gamma(false)?
                    .checked_add(1)
                    .ok_or(DecompressError::InvalidLength)?;
                self.write_bytes(offset, length, output)?;
                if self.read_bit()? {
                    Ok(State::CopyFromNewOffset)
                } else {
                    Ok(State::CopyLiterals)
                }
            }
            State::Done => Ok(State::Done),
        }
    }

    fn read_byte(&mut self) -> Result<u8, DecompressError> {
        let mut buf = [0u8];
        self.source.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_bit(&mut self) -> Result<bool, DecompressError> {
        if self.bit_value == 0x8000 {
            // All bits have been shifted out, only the end marker is left. Get the next byte:
            self.bit_value = ((self.read_byte()? as u16) << 8) | 0x80;
        }
        let bit = self.bit_value & 0x8000 != 0;
        self.bit_value <<= 1;
        Ok(bit)
    }

    fn read_interlaced_elias_gamma(&mut self, inverted: bool) -> Result<usize, DecompressError> {
        let mut value = 1;
        while !self.read_bit()? {
            value = (value << 1) | (self.read_bit()? ^ inverted) as usize;
        }
        Ok(value)
    }

    fn write_bytes(
        &self,
        offset: usize,
        length: usize,
        output: &mut Vec<u8>,
    ) -> Result<(), DecompressError> {
        if offset == 0 {
            return Err(DecompressError::InvalidOffset);
        }
        let Some(s) = output.len().checked_sub(offset) else {
            return Err(DecompressError::InvalidLength);
        };
        let length = length.min(self.settings.max_output_size - output.len());
        output.reserve(length);
        for i in 0..length {
            output.push(output[s + i]);
        }
        Ok(())
    }
}

#[derive(Debug)]
enum State {
    CopyLiterals,
    CopyFromLastOffset,
    CopyFromNewOffset,
    Done,
}

/// Decompress data using the default settings.
/// Reads data from the supplied `source` which is [`Read`] and return it as a `Vec`.
/// Any failures to read from `source` will be returned.
pub fn decompress(source: &mut dyn Read) -> Result<Vec<u8>, DecompressError> {
    decompress_with_settings(source, Settings::default())
}

/// Decompress data using the given settings.
/// Reads data from the supplied `source` which is [`Read`] and return it as a `Vec`.
/// Any failures to read from `source` will be returned.
pub fn decompress_with_settings(
    source: &mut dyn Read,
    settings: Settings,
) -> Result<Vec<u8>, DecompressError> {
    let mut context = Context::new(source, settings.clone());
    let mut output = Vec::new();
    let mut state = State::CopyLiterals;
    while output.len() < settings.max_output_size {
        state = context.next_step(state, &mut output)?;
        if let State::Done = state {
            break;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use super::*;

    #[test]
    fn decompress_to_expected() {
        let source = [
            0x1fu8, 0x41, 0x42, 0x52, 0x41, 0x20, 0xf6, 0xab, 0x43, 0x44, 0xf5, 0xf2, 0x55, 0x58,
        ];
        let result = decompress(&mut source.as_ref()).unwrap();
        assert_eq!(&result, b"ABRA ABRACADABRA");
    }

    #[test]
    fn empty_input() {
        let source: &[u8] = &[];
        let result = decompress(&mut source.as_ref());
        let Err(DecompressError::ReadFailure(e)) = result else {
            panic!("Expected read to fail, got {result:?}");
        };
        assert_eq!(e.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn invalid_input_offset_0() {
        let source = [
            149, 0, 0, 0, 255, 255, 255, 255, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85, 85,
            85, 85, 170, 0,
        ];
        let result = decompress(&mut source.as_ref());
        let Err(DecompressError::InvalidOffset) = result else {
            panic!("Expected InvalidOffset, got {result:?}");
        };
    }
}
