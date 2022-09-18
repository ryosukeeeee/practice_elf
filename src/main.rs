use practice_elf::*;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("prepare_elf64/sample/elfsamp.o")?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let elf64 = parse_elf64(buf.as_slice()).unwrap();

    println!("section names");
    elf64.show_section_names();

    println!();
    println!("symbol names");
    elf64.show_symbol_names();

    println!();
    println!("relocations");
    elf64.show_relocations();

    Ok(())
}
