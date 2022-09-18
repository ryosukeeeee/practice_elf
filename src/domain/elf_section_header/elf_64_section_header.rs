use crate::elf_const::{Elf64Addr, Elf64Off, Elf64Word, Elf64XWord};

#[derive(Debug, PartialEq)]
pub struct Elf64SectionHeader {
    /// Section name, index in string tbl
    pub sh_name: Elf64Word,
    /// Type of section
    pub sh_type: Elf64Word,
    /// Miscellaneous section attributes
    pub sh_flags: Elf64XWord,
    /// Section virtual addr at execution
    pub sh_addr: Elf64Addr,
    /// Section file offset
    pub sh_offset: Elf64Off,
    /// Size of section in bytes
    pub sh_size: Elf64XWord,
    /// Index of another section
    pub sh_link: Elf64Word,
    /// Additional section information
    pub sh_info: Elf64Word,
    /// Section alignment
    pub sh_addralign: Elf64XWord,
    /// Entry size if section holds table
    pub sh_entsize: Elf64XWord,
}
