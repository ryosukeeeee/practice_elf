use std::convert::TryFrom;

use crate::elf_const::{Elf64Addr, Elf64Section, Elf64Word, Elf64XWord};
use crate::elf_header::ErrorKind;

use super::SymbolType;

#[derive(Debug, PartialEq)]
pub struct Elf64Symbol {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64Section,
    pub st_value: Elf64Addr,
    pub st_size: Elf64XWord,
}

impl Elf64Symbol {
    pub fn st_type(&self) -> Result<SymbolType, ErrorKind> {
        SymbolType::try_from(self.st_info)
    }
}
