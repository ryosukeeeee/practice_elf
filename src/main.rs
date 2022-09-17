use practice_elf::{elf_header::ElfHeader, *};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("hello")?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let (_, header) = parse_elf64_header(&buf).unwrap();
    println!("header: {:?}", &header);

    let endian = header.endian().unwrap();
    let shstr = (header.e_shoff + (header.e_shentsize as u64 * header.e_shstrndx as u64)) as usize;
    println!("shsrt: {}", &shstr);
    let (_, section_str) = parse_elf64_section_header(&buf[shstr..], endian).unwrap();

    show_section_names(header, section_str, &buf);

    Ok(())
}
