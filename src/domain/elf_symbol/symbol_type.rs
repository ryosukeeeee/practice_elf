use crate::elf_header::ErrorKind;

#[derive(Debug)]
pub enum SymbolType {
    /// Symbol type is unspecified
    SttNotype = 0,
    /// Symbol is a data object
    SttObject = 1,
    /// Symbol is a code object
    SttFunc = 2,
    /// Symbol associated with a section
    SttSection = 3,
    /// Symbol's name is file name
    SttFile = 4,
    /// Symbol is a common data object
    SttCommon = 5,
    /// Symbol is thread-local data object
    SttTls = 6,
    /// Number of defined types.  
    SttNum = 7,
    /// Start of OS-specific
    SttLoos = 10,
    /// Symbol is indirect code object
    // SttGnuIfunc = 10,
    /// End of OS-specific
    SttHios = 12,
    /// Start of processor-specific
    SttLoproc = 13,
    /// End of processor-specific
    SttHiproc = 15,
}

impl TryFrom<u8> for SymbolType {
    type Error = ErrorKind;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0xf {
            0 => Ok(Self::SttNotype),
            1 => Ok(Self::SttObject),
            2 => Ok(Self::SttFunc),
            3 => Ok(Self::SttSection),
            4 => Ok(Self::SttFile),
            5 => Ok(Self::SttCommon),
            6 => Ok(Self::SttTls),
            7 => Ok(Self::SttNum),
            10 => Ok(Self::SttLoos),
            12 => Ok(Self::SttHios),
            13 => Ok(Self::SttLoproc),
            15 => Ok(Self::SttHiproc),
            _ => Err(ErrorKind::ConvertError),
        }
    }
}
