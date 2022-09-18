use crate::domain::elf_const::*;
use crate::domain::elf_program_header::{ElfSegmentFlags, ElfSegmentType};

#[derive(Debug, PartialEq)]
pub struct Elf64ProgramHeader {
    pub p_type: ElfSegmentType,
    pub p_flags: Vec<ElfSegmentFlags>,
    /// Segment file offset
    pub p_offset: Elf64Off,
    /// Segment virtual address
    pub p_vaddr: Elf64Addr,
    /// Segment physical address
    pub p_paddr: Elf64Addr,
    /// Segment size in file
    pub p_filesz: Elf64XWord,
    /// Segment size in memory
    pub p_memsz: Elf64XWord,
    /// Segment alignment, file & memory
    pub p_align: Elf64XWord,
}
