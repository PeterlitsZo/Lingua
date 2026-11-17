use std::collections::HashMap;

use super::value::Value;

#[derive(Clone, Default)]
pub(crate) struct Environment {
    variables: HashMap<String, Value<'static>>,
}

impl Environment {
    pub(crate) fn insert(&mut self, name: String, value: Value<'static>) {
        self.variables.insert(name, value);
    }

    pub(super) fn get(&self, name: &str) -> Option<&Value<'static>> {
        self.variables.get(name)
    }
}
