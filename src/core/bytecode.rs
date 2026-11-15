use super::{
    Error, ErrorKind,
    ast::{Ast, AstNode},
};

const PUSH_STRING: u8 = 0x01;

pub(super) struct Bytecode {
    constants: Vec<String>,
    code: Vec<u8>,
}

pub(super) struct BytecodeCompiler {
    ast: Ast,
}

impl BytecodeCompiler {
    pub(super) fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub(super) fn compile(self) -> Bytecode {
        match self.ast.root {
            AstNode::StringLiteral(value) => {
                let mut code = vec![PUSH_STRING];
                code.extend_from_slice(&0_u32.to_le_bytes());
                Bytecode {
                    constants: vec![value],
                    code,
                }
            }
        }
    }
}

pub(super) struct BytecodeRunner<'a> {
    bytecode: &'a Bytecode,
    pc: usize,
    stack: Vec<&'a String>,
}

impl<'a> BytecodeRunner<'a> {
    pub(super) fn new(bytecode: &'a Bytecode) -> Self {
        Self {
            bytecode,
            pc: 0,
            stack: Vec::new(),
        }
    }

    pub(super) fn run(mut self) -> Result<String, Error> {
        while let Some(&opcode) = self.bytecode.code.get(self.pc) {
            let offset = self.pc;
            self.pc += 1;
            match opcode {
                PUSH_STRING => {
                    let operand = self.bytecode.code[self.pc..]
                        .first_chunk::<4>()
                        .ok_or_else(|| {
                            Self::invalid_bytecode("Truncated string constant index", offset)
                        })?;
                    let index = u32::from_le_bytes(*operand) as usize;
                    self.pc += 4;
                    let value = self.bytecode.constants.get(index).ok_or_else(|| {
                        Self::invalid_bytecode("String constant index out of bounds", offset)
                            .with_metadata("index", index.to_string())
                    })?;
                    self.stack.push(value);
                }
                _ => {
                    return Err(Self::invalid_bytecode("Unknown opcode", offset)
                        .with_metadata("opcode", opcode.to_string()));
                }
            }
        }

        match self.stack.as_slice() {
            [value] => Ok((*value).clone()),
            _ => Err(
                Self::invalid_bytecode("Expected exactly one string on the stack", self.pc)
                    .with_metadata("stack_size", self.stack.len().to_string()),
            ),
        }
    }

    fn invalid_bytecode(message: &str, offset: usize) -> Error {
        Error::new(ErrorKind::InvalidBytecode, message).with_metadata("offset", offset.to_string())
    }
}
