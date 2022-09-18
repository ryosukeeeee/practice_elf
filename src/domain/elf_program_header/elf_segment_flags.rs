#[derive(PartialEq)]
#[repr(u32)]
pub enum ElfSegmentFlags {
    /// Segment is executable
    PfX = 1 << 0,
    /// Segment is writable
    PfW = 1 << 1,
    /// Segment is readable
    PfR = 1 << 2,
    /// OS-specific
    PfMaskos,
    /// Processor-specific
    PfMaskproc,
}

impl ElfSegmentFlags {
    pub fn from_u32(value: u32) -> Vec<ElfSegmentFlags> {
        let mut output = vec![];
        if (value & (ElfSegmentFlags::PfR as u32)) != 0 {
            output.push(ElfSegmentFlags::PfR);
        }
        if (value & (ElfSegmentFlags::PfW as u32)) != 0 {
            output.push(ElfSegmentFlags::PfW);
        }
        if (value & (ElfSegmentFlags::PfX as u32)) != 0 {
            output.push(ElfSegmentFlags::PfX);
        }
        output
    }
}

impl std::fmt::Debug for ElfSegmentFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PfX => write!(f, "X"),
            Self::PfW => write!(f, "W"),
            Self::PfR => write!(f, "R"),
            Self::PfMaskos => write!(f, "PfMaskos"),
            Self::PfMaskproc => write!(f, "PfMaskproc"),
        }
    }
}
