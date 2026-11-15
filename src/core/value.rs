use std::borrow::Cow;

use super::atom::Atom;

pub(super) enum Value<'a> {
    String(Cow<'a, str>),
    Atom(&'a Atom),
}

impl Value<'_> {
    pub(super) fn append_text(&self, output: &mut String) {
        match self {
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
