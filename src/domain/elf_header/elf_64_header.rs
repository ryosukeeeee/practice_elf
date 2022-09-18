use crate::domain::{
    elf_const::*,
    elf_header::{elf_ident::EMachine, EEndian, Elf64Ident, ElfHeader},
};
use nom::number::Endianness;

#[derive(Clone, Debug, PartialEq)]
pub struct Elf64Header {
    pub e_ident: Elf64Ident,
    /// file type
    pub e_type: Elf64Half,
    /// machine architecture
    pub e_machine: EMachine,
    pub e_version: Elf64Word,
    /// Entry point virtual address
    pub e_entry: Elf64Addr,
    /// Program header table file offset
    pub e_phoff: Elf64Off,
    /// Section header table file offset
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

impl ElfHeader for Elf64Header {
    fn endian(&self) -> Option<Endianness> {
        match self.e_ident.endian {
            EEndian::ElfDataNone => None,
            EEndian::ElfData2LSB => Some(Endianness::Little),
            EEndian::ElfData2MSB => Some(Endianness::Big),
        }
    }
}
