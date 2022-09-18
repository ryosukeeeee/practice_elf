use crate::elf_const::*;

#[derive(Debug, Clone)]
pub struct Elf64Rela {
    /// Address
    pub r_offset: Elf64Addr,
    /// Relocation type and symbol index
    pub r_info: Elf64XWord,
    /// Addend
    pub r_addend: Elf64SXWord,
}

impl Elf64Rela {
    pub fn elf_r_sym(&self) -> u64 {
        self.r_info >> 32
    }
}
