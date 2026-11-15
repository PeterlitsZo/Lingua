use std::borrow::Cow;

use super::atom::Atom;

#[derive(Clone)]
pub(crate) enum Value<'a> {
    String(Cow<'a, str>),
    Atom(Cow<'a, Atom>),
    Null,
    Boolean(bool),
}

impl Value<'_> {
    pub(crate) fn into_owned(self) -> Value<'static> {
        match self {
            Self::String(value) => Value::String(Cow::Owned(value.into_owned())),
            Self::Atom(atom) => Value::Atom(Cow::Owned(atom.into_owned())),
            Self::Null => Value::Null,
            Self::Boolean(value) => Value::Boolean(value),
        }
    }

    pub(super) fn borrowed(&self) -> Value<'_> {
        match self {
            Self::String(value) => Value::String(Cow::Borrowed(value)),
            Self::Atom(atom) => Value::Atom(Cow::Borrowed(atom)),
            Self::Null => Value::Null,
            Self::Boolean(value) => Value::Boolean(*value),
        }
    }

    pub(super) fn append_text(&self, output: &mut String) {
        match self {
            Self::Null => output.push_str("null"),
            Self::Boolean(value) => output.push_str(if *value { "true" } else { "false" }),
            Self::String(value) => output.push_str(value),
            Self::Atom(atom) => {
                output.push(':');
                output.push_str(&atom.name);
            }
        }
    }

    pub(super) fn into_text(self) -> String {
        if let Self::String(value) = self {
            return value.into_owned();
        }
        let mut output = String::new();
        self.append_text(&mut output);
        output
    }
}
