use std::{borrow::Cow, fs};

use super::{
    Bytecode, CONCAT, Constant, DUP, EQUAL, INCLUDE, JUMP, JUMP_IF_FALSE, LOAD_VARIABLE, POP,
    PUSH_ATOM, PUSH_FALSE, PUSH_NULL, PUSH_STRING, PUSH_TRUE,
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
                INCLUDE => {
                    let Value::String(path) = self.pop_value(offset)? else {
                        return Err(Error::new(
                            ErrorKind::Runtime,
                            "include expects a string path",
                        )
                        .with_metadata("offset", offset.to_string()));
                    };
                    let contents = fs::read_to_string(path.as_ref()).map_err(|err| {
                        Error::new(ErrorKind::IO, "Cannot read included file as UTF-8")
                            .with_context("include")
                            .with_metadata("path", path.as_ref())
                            .with_source(err)
                    })?;
                    self.stack.push(Value::String(Cow::Owned(contents)));
                }
                DUP => {
                    let value = self
                        .stack
                        .last()
                        .ok_or_else(|| {
                            Self::invalid_bytecode("Cannot duplicate an empty stack", offset)
                        })?
                        .clone();
                    self.stack.push(value);
                }
                POP => {
                    self.pop_value(offset)?;
                }
                EQUAL => {
                    let right = self.pop_value(offset)?;
                    let left = self.pop_value(offset)?;
                    self.stack.push(Value::Boolean(left == right));
                }
                JUMP | JUMP_IF_FALSE => {
                    let target = self.read_operand(offset)? as usize;
                    if target < self.pc || target > self.bytecode.code.len() {
                        return Err(
                            Self::invalid_bytecode("Invalid forward jump target", offset)
                                .with_metadata("target", target.to_string()),
                        );
                    }
                    let should_jump = if opcode == JUMP_IF_FALSE {
                        match self.pop_value(offset)? {
                            Value::Boolean(value) => !value,
                            _ => {
                                return Err(Self::invalid_bytecode(
                                    "Expected a boolean jump condition",
                                    offset,
                                ));
                            }
                        }
                    } else {
                        true
                    };
                    if should_jump {
                        self.pc = target;
                    }
                }
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

    fn pop_value(&mut self, offset: usize) -> Result<Value<'a>, Error> {
        self.stack
            .pop()
            .ok_or_else(|| Self::invalid_bytecode("Stack underflow", offset))
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
