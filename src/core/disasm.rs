use std::{fs, io::Write, path::Path};

use super::{Error, ErrorKind, bytecode::Bytecode, cache::inspect_header};

pub(crate) fn disassemble_file(path: &Path, output: &mut impl Write) -> Result<(), Error> {
    let result = (|| {
        let bytes = fs::read(path)
            .map_err(|err| Error::new(ErrorKind::IO, "Cannot read LBC file").with_source(err))?;
        let (mut text, payload) = inspect_header(&bytes)?;
        let bytecode = Bytecode::decode(payload)
            .ok_or_else(|| Error::new(ErrorKind::InvalidBytecode, "Cannot decode LBC payload"))?;
        text.push_str(&bytecode.disassemble()?);
        output
            .write_all(text.as_bytes())
            .and_then(|()| output.flush())
            .map_err(|err| Error::new(ErrorKind::IO, "Cannot write disassembly").with_source(err))
    })();
    result.map_err(|err| {
        err.with_context("disassembling LBC file")
            .with_metadata("path", path.display().to_string())
    })
}
