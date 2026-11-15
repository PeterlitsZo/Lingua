use super::{Bytecode, CONCAT, Constant, PUSH_ATOM, PUSH_STRING};
use crate::core::{
    Error, ErrorKind,
    ast::{Ast, AstNode},
};

pub(in crate::core) struct BytecodeCompiler {
    bytecode: Bytecode,
}

impl BytecodeCompiler {
    pub(in crate::core) fn new() -> Self {
        Self {
            bytecode: Bytecode {
                constants: Vec::new(),
                code: Vec::new(),
            },
        }
    }

    pub(in crate::core) fn compile(mut self, ast: Ast) -> Result<Bytecode, Error> {
        self.compile_node(ast.root)?;
        Ok(self.bytecode)
    }

    fn compile_node(&mut self, node: AstNode) -> Result<(), Error> {
        match node {
            AstNode::StringLiteral(value) => {
                self.emit_constant(PUSH_STRING, Constant::String(value))?;
            }
            AstNode::AtomLiteral(atom) => {
                self.emit_constant(PUSH_ATOM, Constant::Atom(atom))?;
            }
            AstNode::Concat(nodes) => {
                let count = u32::try_from(nodes.len()).map_err(|err| {
                    Error::new(ErrorKind::Compile, "Too many concatenation operands")
                        .with_source(err)
                })?;
                for node in nodes {
                    self.compile_node(node)?;
                }
                self.emit(CONCAT, count);
            }
        }
        Ok(())
    }

    fn emit_constant(&mut self, opcode: u8, constant: Constant) -> Result<(), Error> {
        let index = u32::try_from(self.bytecode.constants.len())
            .map_err(|err| Error::new(ErrorKind::Compile, "Too many constants").with_source(err))?;
        self.bytecode.constants.push(constant);
        self.emit(opcode, index);
        Ok(())
    }

    fn emit(&mut self, opcode: u8, operand: u32) {
        self.bytecode.code.push(opcode);
        self.bytecode.code.extend_from_slice(&operand.to_le_bytes());
    }
}
