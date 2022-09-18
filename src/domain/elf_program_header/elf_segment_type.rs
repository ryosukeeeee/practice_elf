#[derive(PartialEq)]
#[repr(u32)]
pub enum ElfSegmentType {
    PtNull = 0,                 /* Program header table entry unused */
    PtLoad = 1,                 /* Loadable program segment */
    PtDynamic = 2,              /* Dynamic linking information */
    PtInterp = 3,               /* Program interpreter */
    PtNote = 4,                 /* Auxiliary information */
    PtShlib = 5,                /* Reserved */
    PtPhdr = 6,                 /* Entry for header table itself */
    PtTls = 7,                  /* Thread-local storage segment */
    PtNum = 8,                  /* Number of defined types */
    PtLoos = 0x60000000,        /* Start of OS-specific */
    PtGnuEhFrame = 0x6474e550,  /* GCC .eh_frame_hdr segment */
    PtGnuStack = 0x6474e551,    /* Indicates stack executability */
    PtGnuRelro = 0x6474e552,    /* Read-only after relocation */
    PtGnuProperty = 0x6474e553, /* GNU property */
    // PtLosunw = 0x6ffffffa,
    // PtSunwbss = 0x6ffffffa,   /* Sun Specific segment */
    PtSunwstack = 0x6ffffffb, /* Stack segment */
    // PtHisunw = 0x6fffffff,
    // PtHios = 0x6fffffff,   /* End of OS-specific */
    PtLoproc = 0x70000000, /* Start of processor-specific */
    PtHiproc = 0x7fffffff, /* End of processor-specific */
}

impl ElfSegmentType {
    pub fn from_u32(value: u32) -> Option<ElfSegmentType> {
        match value {
            0 => Some(ElfSegmentType::PtNull),
            1 => Some(ElfSegmentType::PtLoad),
            2 => Some(ElfSegmentType::PtDynamic),
            3 => Some(ElfSegmentType::PtInterp),
            4 => Some(ElfSegmentType::PtNote),
            5 => Some(ElfSegmentType::PtShlib),
            6 => Some(ElfSegmentType::PtPhdr),
            7 => Some(ElfSegmentType::PtTls),
            8 => Some(ElfSegmentType::PtNum),
            0x60000000 => Some(ElfSegmentType::PtLoos),
            0x6474e550 => Some(ElfSegmentType::PtGnuEhFrame),
            0x6474e551 => Some(ElfSegmentType::PtGnuStack),
            0x6474e552 => Some(ElfSegmentType::PtGnuRelro),
            0x6474e553 => Some(ElfSegmentType::PtGnuProperty),
            // 0x6ffffffa => {Some(ElfSegmentType::PtSunwstack)},
            // 0x6ffffffa => {Some(ElfSegmentType::PtLoproc)},
            0x6ffffffb => Some(ElfSegmentType::PtSunwstack),
            // 0x6fffffff => {Some(ElfSegmentType::)}
            // 0x6fffffff => {Some(ElfSegmentType::)}
            0x70000000 => Some(ElfSegmentType::PtLoproc),
            0x7fffffff => Some(ElfSegmentType::PtHiproc),
            _ => None,
        }
    }
}

impl std::fmt::Debug for ElfSegmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PtNull => write!(f, "NULL"),
            Self::PtLoad => write!(f, "LOAD"),
            Self::PtDynamic => write!(f, "DYNAMIC"),
            Self::PtInterp => write!(f, "INTERP"),
            Self::PtNote => write!(f, "NOTE"),
            Self::PtShlib => write!(f, "SHLIB"),
            Self::PtPhdr => write!(f, "PHDR"),
            Self::PtTls => write!(f, "TLS"),
            Self::PtNum => write!(f, "NUM"),
            Self::PtLoos => write!(f, "LOOS"),
            Self::PtGnuEhFrame => write!(f, "GNU_EH_FRAME"),
            Self::PtGnuStack => write!(f, "GNU_STACK"),
            Self::PtGnuRelro => write!(f, "GNU_RELRO"),
            Self::PtGnuProperty => write!(f, "GNU_PROPERTY"),
            Self::PtSunwstack => write!(f, "SUN_WSTACK"),
            Self::PtLoproc => write!(f, "LOPROC"),
            Self::PtHiproc => write!(f, "HIPROC"),
        }
    }
}
