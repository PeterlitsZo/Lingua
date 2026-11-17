use std::collections::HashMap;

use super::{
    Bytecode, CONCAT, Constant, INCLUDE, JUMP, LOAD_VARIABLE, MATCH_CONST, MATCH_FALSE, MATCH_NULL,
    MATCH_TRUE, POP, PUSH_ATOM, PUSH_FALSE, PUSH_NULL, PUSH_STRING, PUSH_TRUE,
};
use crate::core::{
    Error, ErrorKind,
    ast::{Ast, AstNode, MatchArm, MatchPattern},
};

pub(in crate::core) struct BytecodeCompiler {
    bytecode: Bytecode,
    constant_indices: HashMap<Constant, u32>,
}

impl BytecodeCompiler {
    pub(in crate::core) fn new() -> Self {
        Self {
            constant_indices: HashMap::new(),
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
            AstNode::Include(path) => {
                self.compile_node(*path)?;
                self.bytecode.code.push(INCLUDE);
            }
            AstNode::Match { value, arms } => self.compile_match(*value, arms)?,
            AstNode::Concat(nodes) => self.compile_concat(nodes)?,
        }
        Ok(())
    }

    fn compile_concat(&mut self, nodes: Vec<AstNode>) -> Result<(), Error> {
        let mut merged = Vec::new();
        for node in nodes {
            match (merged.last_mut(), node) {
                (Some(AstNode::StringLiteral(previous)), AstNode::StringLiteral(next)) => {
                    previous.push_str(&next);
                }
                (_, node) => merged.push(node),
            }
        }
        if merged.is_empty() {
            return self.compile_node(AstNode::StringLiteral(String::new()));
        }
        let needs_concat = merged.len() != 1
            || !matches!(
                merged[0],
                AstNode::StringLiteral(_) | AstNode::Include(_) | AstNode::Concat(_)
            );
        let count = u32::try_from(merged.len()).map_err(|err| {
            Error::new(ErrorKind::Compile, "Too many concatenation operands").with_source(err)
        })?;
        for node in merged {
            self.compile_node(node)?;
        }
        if needs_concat {
            self.emit(CONCAT, count);
        }
        Ok(())
    }

    fn compile_match(&mut self, value: AstNode, arms: Vec<MatchArm>) -> Result<(), Error> {
        self.compile_node(value)?;
        let mut exits = Vec::new();
        let mut has_default = false;
        for arm in arms {
            let failure = match arm.pattern {
                MatchPattern::Wildcard => {
                    has_default = true;
                    self.bytecode.code.push(POP);
                    None
                }
                pattern => {
                    match pattern {
                        MatchPattern::String(value) => {
                            self.emit_constant(MATCH_CONST, Constant::String(value))?;
                        }
                        MatchPattern::Atom(atom) => {
                            self.emit_constant(MATCH_CONST, Constant::Atom(atom))?;
                        }
                        MatchPattern::Boolean(value) => {
                            self.bytecode
                                .code
                                .push(if value { MATCH_TRUE } else { MATCH_FALSE })
                        }
                        MatchPattern::Null => self.bytecode.code.push(MATCH_NULL),
                        MatchPattern::Wildcard => unreachable!(),
                    }
                    let target = self.bytecode.code.len();
                    self.bytecode.code.extend_from_slice(&0u32.to_le_bytes());
                    Some(target)
                }
            };
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
        let index = self.register_constant(constant)?;
        self.emit(opcode, index);
        Ok(())
    }

    fn register_constant(&mut self, constant: Constant) -> Result<u32, Error> {
        if let Some(&index) = self.constant_indices.get(&constant) {
            return Ok(index);
        }
        let index = u32::try_from(self.bytecode.constants.len())
            .map_err(|err| Error::new(ErrorKind::Compile, "Too many constants").with_source(err))?;
        self.constant_indices.insert(constant.clone(), index);
        self.bytecode.constants.push(constant);
        Ok(index)
    }

    fn emit(&mut self, opcode: u8, operand: u32) {
        self.bytecode.code.push(opcode);
        self.bytecode.code.extend_from_slice(&operand.to_le_bytes());
    }
}
