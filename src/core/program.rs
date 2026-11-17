use super::{
    Environment, Error,
    bytecode::{Bytecode, BytecodeCompiler, BytecodeRunner},
    parser::Parser,
    timing::Timings,
    value::Value,
};

pub(crate) struct Program {
    bytecode: Bytecode,
}

impl Program {
    pub(super) fn encode(&self) -> Option<Vec<u8>> {
        self.bytecode.encode()
    }

    pub(super) fn decode(bytes: &[u8], timings: &mut Timings) -> Option<Self> {
        let bytecode = timings.measure("Cache decode", || Bytecode::decode(bytes))?;
        Some(Self { bytecode })
    }

    pub(crate) fn compile(source: &str, timings: &mut Timings) -> Result<Self, Error> {
        let ast = timings.measure("Parse", || Parser::new(source).parse())?;
        let bytecode = timings.measure("Compile", || BytecodeCompiler::new().compile(ast))?;
        Ok(Self { bytecode })
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
