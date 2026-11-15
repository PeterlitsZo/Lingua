use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    IO,
    InvalidSource,
    InvalidTarget,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::IO => "IO Error",
            Self::InvalidSource => "Invalid Source",
            Self::InvalidTarget => "Invalid Target",
        })
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String,
    pub contexts: Vec<String>,
    pub metadatas: HashMap<String, String>,
    pub source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: impl Into<String>) -> Self {
        Self {
            kind,
            msg: msg.into(),
            contexts: Vec::new(),
            metadatas: HashMap::new(),
            source: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.contexts.push(context.into());
        self
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadatas.insert(key.into(), value.into());
        self
    }

    pub fn with_source(mut self, source: impl StdError + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] ", self.kind)?;
        for context in self.contexts.iter().rev() {
            write!(f, "{context}: ")?;
        }
        write!(f, "{}", self.msg)?;
        if !self.metadatas.is_empty() {
            write!(f, " (")?;
            let mut entries: Vec<_> = self.metadatas.iter().collect();
            entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
            for (index, (key, value)) in entries.into_iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{key}={value}")?;
            }
            write!(f, ")")?;
        }
        if let Some(source) = &self.source {
            write!(f, ", Source: {source}")?;
        }
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_deref().map(|source| source as &dyn StdError)
    }
}
