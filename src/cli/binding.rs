use std::str::FromStr;

use crate::core::{Environment, Parser, Program, Value};

#[derive(Clone)]
pub(super) struct VariableAssignment {
    pub(super) name: String,
    pub(super) value: Value<'static>,
}

impl FromStr for VariableAssignment {
    type Err = String;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let (name, literal) = source.split_once('=').ok_or("Expected NAME=VALUE")?;
        let name = Parser::new(name)
            .parse_identifier()
            .map_err(|err| format!("Invalid variable name: {err}"))?;
        let program = Program::compile_literal(literal)
            .map_err(|err| format!("Invalid variable value: {err}"))?;
        let value = program
            .run(&Environment::default())
            .map_err(|err| format!("Cannot evaluate variable value: {err}"))?
            .into_owned();
        Ok(Self { name, value })
    }
}
