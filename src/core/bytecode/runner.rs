use std::borrow::Cow;

use super::{
    Bytecode, CONCAT, Constant, LOAD_VARIABLE, PUSH_ATOM, PUSH_FALSE, PUSH_NULL, PUSH_STRING,
    PUSH_TRUE,
};
use crate::core::{Environment, Error, ErrorKind, value::Value};

pub(in crate::core) struct BytecodeRunner<'a> {
    bytecode: &'a Bytecode,
    environment: &'a Environment,
    pc: usize,
    stack: Vec<Value<'a>>,
}

impl<'a> BytecodeRunner<'a> {
    pub(in crate::core) fn new(bytecode: &'a Bytecode, environment: &'a Environment) -> Self {
        Self {
            bytecode,
            environment,
            pc: 0,
            stack: Vec::new(),
        }
    }

    pub(in crate::core) fn run(mut self) -> Result<Value<'a>, Error> {
        while let Some(&opcode) = self.bytecode.code.get(self.pc) {
            let offset = self.pc;
            self.pc += 1;
            match opcode {
                PUSH_STRING | PUSH_ATOM | LOAD_VARIABLE => {
                    let index = self.read_operand(offset)? as usize;
                    let value = self.bytecode.constants.get(index).ok_or_else(|| {
                        Self::invalid_bytecode("Constant index out of bounds", offset)
                            .with_metadata("index", index.to_string())
                    })?;
                    let value = match (opcode, value) {
                        (PUSH_STRING, Constant::String(value)) => {
                            Value::String(Cow::Borrowed(value))
                        }
                        (PUSH_ATOM, Constant::Atom(atom)) => Value::Atom(Cow::Borrowed(atom)),
                        (LOAD_VARIABLE, Constant::String(name)) => self
                            .environment
                            .get(name)
                            .map(Value::borrowed)
                            .unwrap_or(Value::Null),
                        _ => {
                            return Err(Self::invalid_bytecode(
                                "Constant type does not match opcode",
                                offset,
                            )
                            .with_metadata("index", index.to_string()));
                        }
                    };
                    self.stack.push(value);
                }
                PUSH_NULL => self.stack.push(Value::Null),
                PUSH_TRUE => self.stack.push(Value::Boolean(true)),
                PUSH_FALSE => self.stack.push(Value::Boolean(false)),
                CONCAT => {
                    let count = self.read_operand(offset)? as usize;
                    let start = self.stack.len().checked_sub(count).ok_or_else(|| {
                        Self::invalid_bytecode("Not enough values for concatenation", offset)
                            .with_metadata("count", count.to_string())
                            .with_metadata("stack_size", self.stack.len().to_string())
                    })?;
                    let mut result = String::new();
                    for value in self.stack.drain(start..) {
                        value.append_text(&mut result);
                    }
                    self.stack.push(Value::String(Cow::Owned(result)));
                }
                _ => {
                    return Err(Self::invalid_bytecode("Unknown opcode", offset)
                        .with_metadata("opcode", opcode.to_string()));
                }
            }
        }

        if self.stack.len() != 1 {
            return Err(
                Self::invalid_bytecode("Expected exactly one value on the stack", self.pc)
                    .with_metadata("stack_size", self.stack.len().to_string()),
            );
        }
        Ok(self.stack.pop().expect("one value is present"))
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
