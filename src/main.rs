use practice_elf::parse_elf64_header;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("hello")?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    let header = parse_elf64_header(&buf);

    match header {
        Ok(v) => {
            println!("Succeed to parse: {:?}", v.1);
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
