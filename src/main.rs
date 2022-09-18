use practice_elf::{elf_header::ElfHeader, *};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("elfsample.o")?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let (_, header) = parse_elf64_header(&buf).unwrap();
    println!("header: {:?}", &header);

    // show_section_names
    let endian = header.endian().unwrap();
    let shstr = (header.e_shoff + (header.e_shentsize as u64 * header.e_shstrndx as u64)) as usize;
    println!("shsrt: {}", &shstr);
    let (_, section_str) = parse_elf64_section_header(&buf[shstr..], endian).unwrap();

    let strtab = show_section_names(&header, section_str, &buf).unwrap();

    // parse program header table
    let first_program_header_address = header.e_phoff;
    for i in 0..header.e_phnum {
        let addr = (first_program_header_address + (header.e_phentsize as u64 * i as u64)) as usize;
        let (_, program_header) = parse_elf64_program_header(&buf[addr..], endian).unwrap();

        println!("program header: {:?}", &program_header);
    }

    show_symbol_names(&header, &strtab, &buf);
    Ok(())
}
