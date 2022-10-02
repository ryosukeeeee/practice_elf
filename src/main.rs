use clap::Parser;
use practice_elf::*;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    if cli.name.is_empty() {
        println!("no file passed");
        return Ok(());
    }
    let name = &cli.name[0];

    let mut file = File::open(name)?;
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
