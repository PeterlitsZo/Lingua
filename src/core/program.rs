use super::{
    Environment, Error,
    bytecode::{Bytecode, BytecodeCompiler, BytecodeRunner},
    parser::Parser,
    value::Value,
};

pub(crate) struct Program {
    bytecode: Bytecode,
}

impl Program {
    pub(super) fn encode(&self) -> Option<Vec<u8>> {
        self.bytecode.encode()
    }

    pub(super) fn decode(bytes: &[u8]) -> Option<Self> {
        Some(Self {
            bytecode: Bytecode::decode(bytes)?,
        })
    }

    pub(crate) fn compile(source: &str) -> Result<Self, Error> {
        let ast = Parser::new(source).parse()?;
        Ok(Self {
            bytecode: BytecodeCompiler::new().compile(ast)?,
        })
    }

    pub(crate) fn compile_literal(source: &str) -> Result<Self, Error> {
        let ast = Parser::new(source).parse_literal()?;
        Ok(Self {
            bytecode: BytecodeCompiler::new().compile(ast)?,
        })
    }

    pub(crate) fn run<'a>(&'a self, environment: &'a Environment) -> Result<Value<'a>, Error> {
        BytecodeRunner::new(&self.bytecode, environment).run()
    }
}
