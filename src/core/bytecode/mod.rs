mod codec;
mod compiler;
mod disasm;
mod runner;

use super::atom::Atom;

pub(super) use self::{compiler::BytecodeCompiler, runner::BytecodeRunner};

const PUSH_STRING: u8 = 0x01;
const CONCAT: u8 = 0x02;
const PUSH_ATOM: u8 = 0x03;
const PUSH_NULL: u8 = 0x04;
const PUSH_TRUE: u8 = 0x05;
const PUSH_FALSE: u8 = 0x06;
const LOAD_VARIABLE: u8 = 0x07;
const DUP: u8 = 0x08;
const EQUAL: u8 = 0x09;
const JUMP_IF_FALSE: u8 = 0x0A;
const POP: u8 = 0x0B;
const JUMP: u8 = 0x0C;
const INCLUDE: u8 = 0x0D;
const MATCH_CONST: u8 = 0x0E;
const MATCH_TRUE: u8 = 0x0F;
const MATCH_FALSE: u8 = 0x10;
const MATCH_NULL: u8 = 0x11;

pub(super) struct Bytecode {
    constants: Vec<Constant>,
    code: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Constant {
    String(String),
    Atom(Atom),
}
