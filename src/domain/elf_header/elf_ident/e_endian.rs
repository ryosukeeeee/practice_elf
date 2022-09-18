#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum EEndian {
    ElfDataNone = 0,
    /// little endian
    ElfData2LSB = 1,
    /// big endian
    ElfData2MSB = 2,
}

impl EEndian {
    pub fn from_u8(n: u8) -> Option<EEndian> {
        match n {
            0 => Some(EEndian::ElfDataNone),
            1 => Some(EEndian::ElfData2LSB),
            2 => Some(EEndian::ElfData2MSB),
            _ => None,
        }
    }
}
