use crate::elf_header::ErrorKind;

#[repr(u8)]
pub enum SymbolBinding {
    /// Local symbol
    StbLocal = 0,
    /// Global symbol
    StbGlobal = 1,
    /// Weak symbol
    StbWeak = 2,
    /// Number of defined types.
    StbNum = 3,
    /// Start of OS-specific
    StbLoos = 10,
    /// Unique symbol.
    // StbGnuUnique = 10,
    /// End of OS-specific
    StbHios = 12,
    /// Start of processor-specific
    StbLoproc = 13,
    /// End of processor-specific
    StbHiproc = 15,
}

impl TryFrom<u8> for SymbolBinding {
    type Error = ErrorKind;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value >> 4 {
            0 => Ok(Self::StbLocal),
            1 => Ok(Self::StbGlobal),
            2 => Ok(Self::StbWeak),
            3 => Ok(Self::StbNum),
            10 => Ok(Self::StbLoos),
            12 => Ok(Self::StbHios),
            13 => Ok(Self::StbLoproc),
            15 => Ok(Self::StbHiproc),
            _ => Err(ErrorKind::ConvertError),
        }
    }
}
