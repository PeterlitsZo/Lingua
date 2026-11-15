use super::{
    Bytecode, CONCAT, Constant, DUP, EQUAL, JUMP, JUMP_IF_FALSE, LOAD_VARIABLE, POP, PUSH_ATOM,
    PUSH_FALSE, PUSH_NULL, PUSH_STRING, PUSH_TRUE,
};
use crate::core::{
    Error, ErrorKind,
    ast::{Ast, AstNode, MatchArm, MatchPattern},
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
            AstNode::NullLiteral => self.bytecode.code.push(PUSH_NULL),
            AstNode::BooleanLiteral(value) => {
                self.bytecode
                    .code
                    .push(if value { PUSH_TRUE } else { PUSH_FALSE });
            }
            AstNode::Variable(name) => {
                self.emit_constant(LOAD_VARIABLE, Constant::String(name))?;
            }
            AstNode::Match { value, arms } => self.compile_match(*value, arms)?,
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

    fn compile_match(&mut self, value: AstNode, arms: Vec<MatchArm>) -> Result<(), Error> {
        self.compile_node(value)?;
        let mut exits = Vec::new();
        let mut has_default = false;
        for arm in arms {
            let failure = if matches!(arm.pattern, MatchPattern::Wildcard) {
                has_default = true;
                None
            } else {
                self.bytecode.code.push(DUP);
                match arm.pattern {
                    MatchPattern::String(value) => {
                        self.emit_constant(PUSH_STRING, Constant::String(value))?
                    }
                    MatchPattern::Atom(atom) => {
                        self.emit_constant(PUSH_ATOM, Constant::Atom(atom))?
                    }
                    MatchPattern::Boolean(value) => {
                        self.bytecode
                            .code
                            .push(if value { PUSH_TRUE } else { PUSH_FALSE })
                    }
                    MatchPattern::Null => self.bytecode.code.push(PUSH_NULL),
                    MatchPattern::Wildcard => unreachable!(),
                }
                self.bytecode.code.push(EQUAL);
                Some(self.emit_jump(JUMP_IF_FALSE))
            };
            self.bytecode.code.push(POP);
            self.compile_node(arm.value)?;
            exits.push(self.emit_jump(JUMP));
            if let Some(failure) = failure {
                self.patch_jump(failure)?;
            }
        }
        if !has_default {
            self.bytecode.code.push(POP);
            self.bytecode.code.push(PUSH_NULL);
        }
        for exit in exits {
            self.patch_jump(exit)?;
        }
        Ok(())
    }

    fn emit_jump(&mut self, opcode: u8) -> usize {
        let operand = self.bytecode.code.len() + 1;
        self.emit(opcode, 0);
        operand
    }

    fn patch_jump(&mut self, operand: usize) -> Result<(), Error> {
        let target = u32::try_from(self.bytecode.code.len()).map_err(|err| {
            Error::new(ErrorKind::Compile, "Bytecode jump target is too large").with_source(err)
        })?;
        self.bytecode.code[operand..operand + 4].copy_from_slice(&target.to_le_bytes());
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
