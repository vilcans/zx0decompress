use std::io::Read;

struct Context<'a> {
    source: &'a mut dyn Read,
    backtrack: bool,
    last_offset: usize,
    last_byte: u8,
    bit_mask: u8,
    bit_value: u8,
}

impl<'a> Context<'a> {
    pub fn new(source: &'a mut dyn Read) -> Self {
        Self {
            source,
            backtrack: false,
            last_offset: 1,
            last_byte: 0,
            bit_mask: 0,
            bit_value: 0,
        }
    }
    pub fn next_step(&mut self, state: State, output: &mut Vec<u8>) -> std::io::Result<State> {
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
                self.write_bytes(self.last_offset, length, output);
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
                let sub = self.read_byte()? >> 1;
                self.last_offset = (high << 7) - sub as usize;
                self.backtrack = true;
                let length = self.read_interlaced_elias_gamma(false)? + 1;
                self.write_bytes(self.last_offset, length, output);
                if self.read_bit()? {
                    Ok(State::CopyFromNewOffset)
                } else {
                    Ok(State::CopyLiterals)
                }
            }
            State::Done => Ok(State::Done),
        }
    }

    fn read_byte(&mut self) -> std::io::Result<u8> {
        let mut buf = [0u8];
        self.source.read_exact(&mut buf)?;
        self.last_byte = buf[0];
        Ok(self.last_byte)
    }

    fn read_bit(&mut self) -> std::io::Result<bool> {
        if self.backtrack {
            self.backtrack = false;
            let bit = self.last_byte & 1 != 0;
            Ok(bit)
        } else {
            self.bit_mask >>= 1;
            if self.bit_mask == 0 {
                self.bit_mask = 0x80;
                self.bit_value = self.read_byte()?;
            }
            let bit = (self.bit_value & self.bit_mask) != 0;
            Ok(bit)
        }
    }
    fn read_interlaced_elias_gamma(&mut self, inverted: bool) -> std::io::Result<usize> {
        let mut value = 1;
        while !self.read_bit()? {
            value = (value << 1) | (self.read_bit()? ^ inverted) as usize;
        }
        Ok(value)
    }
    fn write_bytes(&mut self, offset: usize, length: usize, output: &mut Vec<u8>) {
        for _ in 0..length {
            let b = output[output.len() - offset];
            output.push(b);
        }
    }
}

#[derive(Debug)]
enum State {
    CopyLiterals,
    CopyFromLastOffset,
    CopyFromNewOffset,
    Done,
}

pub fn decompress(source: &mut dyn Read) -> std::io::Result<Vec<u8>> {
    let mut context = Context::new(source);
    let mut output = Vec::new();
    let mut state = State::CopyLiterals;
    loop {
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
