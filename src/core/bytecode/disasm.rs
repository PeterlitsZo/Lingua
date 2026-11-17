use std::fmt::Write;

use super::{
    Bytecode, CONCAT, Constant, DUP, EQUAL, INCLUDE, JUMP, JUMP_IF_FALSE, LOAD_VARIABLE,
    MATCH_CONST, MATCH_FALSE, MATCH_NULL, MATCH_TRUE, POP, PUSH_ATOM, PUSH_FALSE, PUSH_NULL,
    PUSH_STRING, PUSH_TRUE,
};
use crate::core::{Error, ErrorKind};

impl Bytecode {
    pub(in crate::core) fn disassemble(&self) -> Result<String, Error> {
        let mut output = format!("\nConstants ({})\n", self.constants.len());
        for (index, constant) in self.constants.iter().enumerate() {
            let (kind, text) = constant.display_parts();
            writeln!(output, "  #{index}  {kind}  {text:?}").expect("write to string");
        }
        writeln!(output, "\nInstructions ({} bytes)", self.code.len()).expect("write to string");
        let mut pc = 0;
        while pc < self.code.len() {
            let offset = pc;
            let opcode = self.code[pc];
            pc += 1;
            let (name, has_operand) = match opcode {
                PUSH_STRING => ("PUSH_STRING", true),
                CONCAT => ("CONCAT", true),
                PUSH_ATOM => ("PUSH_ATOM", true),
                PUSH_NULL => ("PUSH_NULL", false),
                PUSH_TRUE => ("PUSH_TRUE", false),
                PUSH_FALSE => ("PUSH_FALSE", false),
                LOAD_VARIABLE => ("LOAD_VARIABLE", true),
                DUP => ("DUP", false),
                EQUAL => ("EQUAL", false),
                JUMP_IF_FALSE => ("JUMP_IF_FALSE", true),
                POP => ("POP", false),
                JUMP => ("JUMP", true),
                INCLUDE => ("INCLUDE", false),
                MATCH_CONST => ("MATCH_CONST", true),
                MATCH_TRUE => ("MATCH_TRUE", true),
                MATCH_FALSE => ("MATCH_FALSE", true),
                MATCH_NULL => ("MATCH_NULL", true),
                _ => {
                    return Err(invalid("Unknown opcode", offset)
                        .with_metadata("opcode", format!("0x{opcode:02X}")));
                }
            };
            write!(output, "  {offset:04}  {name}").expect("write to string");
            if has_operand {
                let bytes = self.code[pc..]
                    .first_chunk::<4>()
                    .ok_or_else(|| invalid("Truncated instruction operand", offset))?;
                let operand = u32::from_le_bytes(*bytes);
                pc += 4;
                write!(output, "{:width$}", "", width = 16 - name.len()).expect("write to string");
                match opcode {
                    PUSH_STRING | PUSH_ATOM | LOAD_VARIABLE | MATCH_CONST => {
                        write!(output, "#{operand}").expect("write to string");
                        if opcode == MATCH_CONST {
                            let bytes = self.code[pc..]
                                .first_chunk::<4>()
                                .ok_or_else(|| invalid("Truncated instruction operand", offset))?;
                            let target = u32::from_le_bytes(*bytes);
                            pc += 4;
                            write!(output, ", {target:04}").expect("write to string");
                        }
                        output.push_str("    ; ");
                        match self.constants.get(operand as usize) {
                            Some(constant) => {
                                let (kind, text) = constant.display_parts();
                                write!(output, "{kind} {text:?}").expect("write to string");
                            }
                            None => output.push_str("invalid constant index"),
                        }
                    }
                    JUMP | JUMP_IF_FALSE | MATCH_TRUE | MATCH_FALSE | MATCH_NULL => {
                        write!(output, "{operand:04}").expect("write to string")
                    }
                    _ => write!(output, "{operand}").expect("write to string"),
                }
            }
            output.push('\n');
        }
        Ok(output)
    }
}

impl Constant {
    fn display_parts(&self) -> (&'static str, &str) {
        match self {
            Self::String(text) => ("String", text),
            Self::Atom(atom) => ("Atom", atom.name()),
        }
    }
}

fn invalid(message: &str, offset: usize) -> Error {
    Error::new(ErrorKind::InvalidBytecode, message).with_metadata("offset", offset.to_string())
}
