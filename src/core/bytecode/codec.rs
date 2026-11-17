use super::{Bytecode, Constant};
use crate::core::atom::Atom;

impl Bytecode {
    pub(in crate::core) fn encode(&self) -> Option<Vec<u8>> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&u32::try_from(self.constants.len()).ok()?.to_le_bytes());
        for constant in &self.constants {
            let (tag, value) = match constant {
                Constant::String(value) => (0, value.as_str()),
                Constant::Atom(atom) => (1, atom.name()),
            };
            bytes.push(tag);
            bytes.extend_from_slice(&u32::try_from(value.len()).ok()?.to_le_bytes());
            bytes.extend_from_slice(value.as_bytes());
        }
        bytes.extend_from_slice(&u32::try_from(self.code.len()).ok()?.to_le_bytes());
        bytes.extend_from_slice(&self.code);
        Some(bytes)
    }

    pub(in crate::core) fn decode(mut bytes: &[u8]) -> Option<Self> {
        let count = read_u32(&mut bytes)? as usize;
        if count > bytes.len() / 5 {
            return None;
        }
        let mut constants = Vec::new();
        for _ in 0..count {
            let (&tag, rest) = bytes.split_first()?;
            bytes = rest;
            let len = read_u32(&mut bytes)? as usize;
            let value = std::str::from_utf8(bytes.get(..len)?).ok()?;
            bytes = &bytes[len..];
            constants.push(match tag {
                0 => Constant::String(value.to_owned()),
                1 => Constant::Atom(Atom::intern(value).ok()?),
                _ => return None,
            });
        }
        let len = read_u32(&mut bytes)? as usize;
        if bytes.len() != len {
            return None;
        }
        Some(Self {
            constants,
            code: bytes.to_vec(),
        })
    }
}

fn read_u32(bytes: &mut &[u8]) -> Option<u32> {
    let value = u32::from_le_bytes(bytes.get(..4)?.try_into().ok()?);
    *bytes = &bytes[4..];
    Some(value)
}
