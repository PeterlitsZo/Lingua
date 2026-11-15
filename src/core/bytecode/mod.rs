mod compiler;
mod runner;

pub(super) use self::{compiler::BytecodeCompiler, runner::BytecodeRunner};

const PUSH_STRING: u8 = 0x01;
const CONCAT: u8 = 0x02;

pub(super) struct Bytecode {
    constants: Vec<String>,
    code: Vec<u8>,
}
