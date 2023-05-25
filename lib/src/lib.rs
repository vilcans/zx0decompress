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

struct Context<'a> {
    source: &'a mut dyn Read,
    max_output_size: usize,
    last_offset: usize,
    /// The number of valid bits in `bit_value`.
    bit_count: u8,
    /// Current bit data. The most significant bit contains the next bit that will be read.
    bit_value: u16,
}

impl<'a> Context<'a> {
    pub fn new(source: &'a mut dyn Read) -> Self {
        Self {
            source,
            max_output_size: 0x20000,
            last_offset: 1,
            bit_count: 0,
            bit_value: 0,
        }
    }
    /// Executes the next step of the compression. Returns the next state.
    fn next_step(&mut self, state: State, output: &mut Vec<u8>) -> Result<State, DecompressError> {
        let classic_mode = false;
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
                let high = self.read_interlaced_elias_gamma(!classic_mode)?;
                if high == 256 {
                    return Ok(State::Done);
                }
                let second_byte = self.read_byte()?;
                self.last_offset = (high << 7) - (second_byte >> 1) as usize;

                // Make the lowest bit in second byte be the next bit to read
                self.bit_value = (self.bit_value >> 1) | ((second_byte as u16) << 15);
                self.bit_count += 1;

                let length = self.read_interlaced_elias_gamma(false)? + 1;
                self.write_bytes(self.last_offset, length, output)?;
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
        if self.bit_count == 0 {
            self.bit_value = (self.read_byte()? as u16) << 8;
            self.bit_count = 8;
        }
        let bit = self.bit_value & 0x8000 != 0;
        self.bit_value <<= 1;
        self.bit_count -= 1;
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
        let Some(s) = output.len().checked_sub(offset) else {
            return Err(DecompressError::InvalidLength);
        };

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

/// Reads data from the supplied `source` which is [`Read`] and return it as a `Vec`.
/// Any failures to read from `source` will be returned.
pub fn decompress(source: &mut dyn Read) -> Result<Vec<u8>, DecompressError> {
    let mut context = Context::new(source);
    let mut output = Vec::new();
    let mut state = State::CopyLiterals;
    while output.len() < context.max_output_size {
        state = context.next_step(state, &mut output)?;
        if let State::Done = state {
            break;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_to_expected() {
        let source = [
            0x1fu8, 0x41, 0x42, 0x52, 0x41, 0x20, 0xf6, 0xab, 0x43, 0x44, 0xf5, 0xf2, 0x55, 0x58,
        ];
        let result = decompress(&mut source.as_ref()).unwrap();
        assert_eq!(&result, b"ABRA ABRACADABRA");
    }
}
