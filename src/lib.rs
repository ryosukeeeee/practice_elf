mod domain;

use crate::domain::elf_header::{EMachine, Elf64Hdr, Elf64Ident};
use crate::domain::elf_header::{ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3};
use nom::bytes::complete::tag;
use nom::number::{complete::*, Endianness};
use nom::IResult;

pub fn parse_elf64_header(input: &[u8]) -> IResult<&[u8], Elf64Hdr> {
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
        Elf64Hdr {
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
