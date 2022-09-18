use crate::elf_header::ErrorKind;

#[repr(u8)]
pub enum SymbolVisibility {
    /// Default symbol visibility rules
    StvDEFAULT = 0,
    /// Processor specific hidden class
    StvINTERNAL = 1,
    /// Sym unavailable in other modules
    StvHIDDEN = 2,
    /// Not preemptible, not exported
    StvPROTECTED = 3,
}

impl TryFrom<u8> for SymbolVisibility {
    type Error = ErrorKind;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0x03 {
            0 => Ok(Self::StvDEFAULT),
            1 => Ok(Self::StvINTERNAL),
            2 => Ok(Self::StvHIDDEN),
            3 => Ok(Self::StvPROTECTED),
            _ => Err(ErrorKind::ConvertError),
        }
    }
}
