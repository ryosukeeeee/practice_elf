mod e_class;
mod e_machine;

pub use self::e_class::*;
pub use self::e_machine::*;

use crate::domain::elf_header::{elf_ident::EClass, ErrorKind, ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3};

#[derive(Debug, PartialEq)]
pub struct Elf64Ident {
    /// 0x7f, 'E', 'L', 'F'
    pub magic_number: [u8; 4],
    pub cpu_arch: EClass,
    pub endian: u8,
    pub elf_format_version: u8,
    pub abi: u8,
    pub abi_version: u8,
    pub padding: [u8; 7],
}

impl Elf64Ident {
    pub fn try_new(
        cpu_arch: u8,
        endian: u8,
        elf_format_version: u8,
        abi: u8,
        abi_version: u8,
        padding: [u8; 7],
    ) -> Result<Self, ErrorKind> {
        let magic_number = [ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3];
        let cpu_arch = EClass::from_u8(cpu_arch).ok_or(ErrorKind::ConvertError)?;

        Ok(Self {
            magic_number,
            cpu_arch,
            endian,
            elf_format_version,
            abi,
            abi_version,
            padding,
        })
    }
}
