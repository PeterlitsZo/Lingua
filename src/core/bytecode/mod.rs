mod compiler;
mod runner;

use super::atom::Atom;

pub(super) use self::{compiler::BytecodeCompiler, runner::BytecodeRunner};

const PUSH_STRING: u8 = 0x01;
const CONCAT: u8 = 0x02;
const PUSH_ATOM: u8 = 0x03;

pub(super) struct Bytecode {
    constants: Vec<Constant>,
    code: Vec<u8>,
}

enum Constant {
    String(String),
    Atom(Atom),
}
