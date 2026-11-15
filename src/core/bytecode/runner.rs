use std::borrow::Cow;

use super::{Bytecode, CONCAT, PUSH_STRING};
use crate::core::{Error, ErrorKind};

pub(in crate::core) struct BytecodeRunner<'a> {
    bytecode: &'a Bytecode,
    pc: usize,
    stack: Vec<Cow<'a, str>>,
}

impl<'a> BytecodeRunner<'a> {
    pub(in crate::core) fn new(bytecode: &'a Bytecode) -> Self {
        Self {
            bytecode,
            pc: 0,
            stack: Vec::new(),
        }
    }

    pub(in crate::core) fn run(mut self) -> Result<String, Error> {
        while let Some(&opcode) = self.bytecode.code.get(self.pc) {
            let offset = self.pc;
            self.pc += 1;
            match opcode {
                PUSH_STRING => {
                    let index = self.read_operand(offset)? as usize;
                    let value = self.bytecode.constants.get(index).ok_or_else(|| {
                        Self::invalid_bytecode("String constant index out of bounds", offset)
                            .with_metadata("index", index.to_string())
                    })?;
                    self.stack.push(Cow::Borrowed(value.as_str()));
                }
                CONCAT => {
                    let count = self.read_operand(offset)? as usize;
                    let start = self.stack.len().checked_sub(count).ok_or_else(|| {
                        Self::invalid_bytecode("Not enough strings for concatenation", offset)
                            .with_metadata("count", count.to_string())
                            .with_metadata("stack_size", self.stack.len().to_string())
                    })?;
                    let mut result = String::new();
                    for value in self.stack.drain(start..) {
                        result.push_str(&value);
                    }
                    self.stack.push(Cow::Owned(result));
                }
                _ => {
                    return Err(Self::invalid_bytecode("Unknown opcode", offset)
                        .with_metadata("opcode", opcode.to_string()));
                }
            }
        }

        if self.stack.len() != 1 {
            return Err(Self::invalid_bytecode(
                "Expected exactly one string on the stack",
                self.pc,
            )
            .with_metadata("stack_size", self.stack.len().to_string()));
        }
        Ok(self
            .stack
            .pop()
            .expect("one string is present")
            .into_owned())
    }

    fn read_operand(&mut self, offset: usize) -> Result<u32, Error> {
        let operand = self.bytecode.code[self.pc..]
            .first_chunk::<4>()
            .ok_or_else(|| Self::invalid_bytecode("Truncated instruction operand", offset))?;
        self.pc += 4;
        Ok(u32::from_le_bytes(*operand))
    }

    fn invalid_bytecode(message: &str, offset: usize) -> Error {
        Error::new(ErrorKind::InvalidBytecode, message).with_metadata("offset", offset.to_string())
    }
}
