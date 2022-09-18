mod domain;

use domain::elf_program_header::{ElfSegmentFlags, ElfSegmentType};
use domain::elf_symbol::Elf64Symbol;
pub use domain::*;

use crate::domain::elf_header::{
    EMachine, Elf64Header, Elf64Ident, ElfHeader, ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3,
};
use crate::domain::elf_program_header::Elf64ProgramHeader;
use crate::domain::elf_section_header::Elf64SectionHeader;
use nom::bytes::complete::tag;
use nom::number::{complete::*, Endianness};
use nom::IResult;

pub fn parse_elf64_header(input: &[u8]) -> IResult<&[u8], Elf64Header> {
    let (input, _) = tag([ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3])(input)?;
    let (input, e_ident_4) = be_u8(input)?;
    let (input, e_ident_5) = be_u8(input)?;
    let (input, e_ident_6) = be_u8(input)?;
    let (input, e_ident_7) = be_u8(input)?;
    let (input, e_ident_8) = be_u8(input)?;
    let (input, e_ident_9) = be_u8(input)?;
    let (input, e_ident_10) = be_u8(input)?;
    let (input, e_ident_11) = be_u8(input)?;
    let (input, e_ident_12) = be_u8(input)?;
    let (input, e_ident_13) = be_u8(input)?;
    let (input, e_ident_14) = be_u8(input)?;
    let (input, e_ident_15) = be_u8(input)?;

    let endianness = match e_ident_5 {
        1 => Endianness::Little,
        2 => Endianness::Big,
        _ => panic!("invalid endianness"),
    };
    let (input, e_type) = u16(endianness)(input)?;
    let (input, e_machine) = u16(endianness)(input)?;
    let (input, e_version) = u32(endianness)(input)?;
    let (input, e_entry) = u64(endianness)(input)?;
    let (input, e_phoff) = u64(endianness)(input)?;
    let (input, e_shoff) = u64(endianness)(input)?;
    let (input, e_flags) = u32(endianness)(input)?;
    let (input, e_ehsize) = u16(endianness)(input)?;
    let (input, e_phentsize) = u16(endianness)(input)?;
    let (input, e_phnum) = u16(endianness)(input)?;
    let (input, e_shentsize) = u16(endianness)(input)?;
    let (input, e_shnum) = u16(endianness)(input)?;
    let (input, e_shstrndx) = u16(endianness)(input)?;

    let padding = [
        e_ident_9, e_ident_10, e_ident_11, e_ident_12, e_ident_13, e_ident_14, e_ident_15,
    ];
    let ident = Elf64Ident::try_new(
        e_ident_4, e_ident_5, e_ident_6, e_ident_7, e_ident_8, padding,
    )
    .map_err(|_e| nom::error::Error::<&[u8]>::new(input, nom::error::ErrorKind::Verify))
    .unwrap();

    let machine = EMachine::from_u16(e_machine).unwrap();

    Ok((
        input,
        Elf64Header {
            e_ident: ident,
            e_type,
            e_machine: machine,
            e_version,
            e_entry,
            e_phoff,
            e_shoff,
            e_flags,
            e_ehsize,
            e_phentsize,
            e_phnum,
            e_shentsize,
            e_shnum,
            e_shstrndx,
        },
    ))
}

pub fn parse_elf64_section_header(
    input: &[u8],
    endianness: Endianness,
) -> IResult<&[u8], Elf64SectionHeader> {
    let (input, sh_name) = u32(endianness)(input)?;
    let (input, sh_type) = u32(endianness)(input)?;
    let (input, sh_flags) = u64(endianness)(input)?;
    let (input, sh_addr) = u64(endianness)(input)?;
    let (input, sh_offset) = u64(endianness)(input)?;
    let (input, sh_size) = u64(endianness)(input)?;
    let (input, sh_link) = u32(endianness)(input)?;
    let (input, sh_info) = u32(endianness)(input)?;
    let (input, sh_addralign) = u64(endianness)(input)?;
    let (input, sh_entsize) = u64(endianness)(input)?;

    Ok((
        input,
        Elf64SectionHeader {
            sh_name,
            sh_type,
            sh_flags,
            sh_addr,
            sh_offset,
            sh_size,
            sh_link,
            sh_info,
            sh_addralign,
            sh_entsize,
        },
    ))
}

pub fn parse_elf64_program_header(
    input: &[u8],
    endianness: Endianness,
) -> IResult<&[u8], Elf64ProgramHeader> {
    let (input, p_type) = u32(endianness)(input)?;
    let (input, p_flags) = u32(endianness)(input)?;
    let (input, p_offset) = u64(endianness)(input)?;
    let (input, p_vaddr) = u64(endianness)(input)?;
    let (input, p_paddr) = u64(endianness)(input)?;
    let (input, p_filesz) = u64(endianness)(input)?;
    let (input, p_memsz) = u64(endianness)(input)?;
    let (input, p_align) = u64(endianness)(input)?;

    let p_type = ElfSegmentType::from_u32(p_type).unwrap();
    let p_flags = ElfSegmentFlags::from_u32(p_flags);
    Ok((
        input,
        Elf64ProgramHeader {
            p_type,
            p_flags,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_align,
        },
    ))
}

pub fn parse_elf64_symbol(input: &[u8], endianness: Endianness) -> IResult<&[u8], Elf64Symbol> {
    let (input, st_name) = u32(endianness)(input)?;
    let (input, st_info) = u8(input)?;
    let (input, st_other) = u8(input)?;
    let (input, st_shndx) = u16(endianness)(input)?;
    let (input, st_value) = u64(endianness)(input)?;
    let (input, st_size) = u64(endianness)(input)?;

    Ok((
        input,
        Elf64Symbol {
            st_name,
            st_info,
            st_other,
            st_shndx,
            st_value,
            st_size,
        },
    ))
}

pub fn show_section_names(
    header: &Elf64Header,
    str_tbl_sh: Elf64SectionHeader,
    buf: &[u8],
) -> Option<Elf64SectionHeader> {
    let mut output = None;
    for i in 0..(header.e_shnum) {
        let addr = (header.e_shoff + (header.e_shentsize as u64 * i as u64)) as usize;
        let (_, shdr) = parse_elf64_section_header(&buf[addr..], header.endian().unwrap()).unwrap();
        let name_addr = (str_tbl_sh.sh_offset + shdr.sh_name as u64) as usize;
        let (_, name) =
            nom::bytes::complete::take_till::<_, _, nom::error::Error<_>>(|c| c == 0x00)(
                &buf[name_addr..],
            )
            .unwrap();
        println!("name: {}", String::from_utf8_lossy(name));
        if name == b".strtab" {
            output = Some(shdr);
        }
    }
    output
}

pub fn show_symbol_names(header: &Elf64Header, str_tab: &Elf64SectionHeader, buf: &[u8]) {
    let endian = header.endian().unwrap();

    for i in 0..header.e_shnum as u64 {
        let addr = (header.e_shoff + (header.e_shentsize as u64 * i)) as usize;
        let (_, shdr) = parse_elf64_section_header(&buf[addr..], endian).unwrap();
        if shdr.sh_type != 0x02 {
            continue;
        }
        println!("sym: {:?}", &shdr);
        // find symbol table section header
        let sym = shdr;
        for j in 0..(sym.sh_size / sym.sh_entsize) {
            let addr = (sym.sh_offset + (sym.sh_entsize * j)) as usize;
            let (_, symp) = parse_elf64_symbol(&buf[addr..], endian).unwrap();

            if symp.st_name == 0 {
                continue;
            }
            let name = {
                let addr = (str_tab.sh_offset + symp.st_name as u64) as usize;
                let (_, name) = nom::bytes::complete::take_till::<_, _, nom::error::Error<_>>(
                    |c| c == 0x00,
                )(&buf[addr..])
                .unwrap();
                String::from_utf8_lossy(name)
            };
            println!("\t{} [{:?}]\t{}", j, symp.st_type().unwrap(), name);
        }
    }
}
