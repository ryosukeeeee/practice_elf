#[derive(Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum EMachine {
    EmNone = 0,
    EmM32 = 1,
    EmSparc = 2,
    Em386 = 3,
    Em68K = 4,
    Em88K = 5,
    Em486 = 6, /* Perhaps disused */
    Em860 = 7,
    EmMips = 8, /* MIPS R3000 (officially, big-endian only) */
    /* Next two are historical and binaries and
    modules of these types will be rejected by
    Linux.  */
    // EmMipsRs3Le = 10, /* MIPS R3000 little-endian */
    // EmMipsRs4Be = 10, /* MIPS R4000 big-endian */
    EmParisc = 15,       /* HPPA */
    EmSparc32Plus = 18,  /* Sun's "v8plus" */
    EmPpc = 20,          /* PowerPC */
    EmPpc64 = 21,        /* PowerPC64 */
    EmSpu = 23,          /* Cell BE SPU */
    EmArm = 40,          /* ARM 32 bit */
    EmSh = 42,           /* SuperH */
    EmSparcv9 = 43,      /* SPARC v9 64-bit */
    EmH8_300 = 46,       /* Renesas H8/300 */
    EmIa64 = 50,         /* HP/Intel IA-64 */
    EmX86_64 = 62,       /* AMD x86-64 */
    EmS390 = 22,         /* IBM S/390 */
    EmCris = 76,         /* Axis Communications 32-bit embedded processor */
    EmM32R = 88,         /* Renesas M32R */
    EmMn10300 = 89,      /* Panasonic/MEI MN10300, AM33 */
    EmOpenrisc = 92,     /* OpenRISC 32-bit embedded processor */
    EmBlackfin = 106,    /* ADI Blackfin Processor */
    EmAlteraNios2 = 113, /* Altera Nios II soft-core processor */
    EmTiC6000 = 140,     /* TI C6X DSPs */
    EmAarch64 = 183,     /* ARM 64 bit */
    EmTilepro = 188,     /* Tilera TILEPro */
    EmMicroblaze = 189,  /* Xilinx MicroBlaze */
    EmTilegx = 191,      /* Tilera TILE-Gx */
    EmBpf = 247,         /* Linux BPF - in-kernel virtual machine */
    EmFrv = 0x5441,      /* Fujitsu FR-V */

    /*
     * This is an interim value that we will use until the committee comes
     * up with a final number.
     */
    EmAlpha = 0x9026,

    /* Bogus old m32r magic number, used by old tools. */
    EmCygnusM32R = 0x9041,
    /* This is the old interim value for S/390 architecture */
    EmS390Old = 0xA390,
    /* Also Panasonic/MEI MN10300, AM33 */
    EmCygnusMn10300 = 0xbeef,
}

impl EMachine {
    pub fn from_u16(n: u16) -> Option<EMachine> {
        match n {
            0 => Some(EMachine::EmNone),
            1 => Some(EMachine::EmM32),
            2 => Some(EMachine::Em386),
            4 => Some(EMachine::Em68K),
            5 => Some(EMachine::Em88K),
            6 => Some(EMachine::Em486),
            7 => Some(EMachine::Em860),
            8 => Some(EMachine::EmMips),
            15 => Some(EMachine::EmParisc),
            18 => Some(EMachine::EmSparc32Plus),
            20 => Some(EMachine::EmPpc),
            21 => Some(EMachine::EmPpc64),
            23 => Some(EMachine::EmSpu),
            40 => Some(EMachine::EmArm),
            42 => Some(EMachine::EmSh),
            43 => Some(EMachine::EmSparcv9),
            46 => Some(EMachine::EmH8_300),
            50 => Some(EMachine::EmIa64),
            62 => Some(EMachine::EmX86_64),
            22 => Some(EMachine::EmS390),
            76 => Some(EMachine::EmCris),
            88 => Some(EMachine::EmM32R),
            89 => Some(EMachine::EmMn10300),
            92 => Some(EMachine::EmOpenrisc),
            106 => Some(EMachine::EmBlackfin),
            113 => Some(EMachine::EmAlteraNios2),
            140 => Some(EMachine::EmTiC6000),
            183 => Some(EMachine::EmAarch64),
            188 => Some(EMachine::EmTilepro),
            189 => Some(EMachine::EmMicroblaze),
            191 => Some(EMachine::EmTilegx),
            247 => Some(EMachine::EmBpf),
            0x5441 => Some(EMachine::EmFrv),
            0x9026 => Some(EMachine::EmAlpha),
            0x9041 => Some(EMachine::EmCygnusM32R),
            0xA390 => Some(EMachine::EmS390Old),
            0xbeef => Some(EMachine::EmCygnusMn10300),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u16() {
        let p = EMachine::from_u16(113);

        assert_eq!(p, Some(EMachine::EmAlteraNios2));
    }
}
