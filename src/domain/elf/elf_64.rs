use std::sync::Mutex;

use crate::{
    elf_header::{Elf64Header, ElfHeader},
    elf_program_header::Elf64ProgramHeader,
    elf_section_header::Elf64SectionHeader,
    parse_elf64_symbol,
};

pub struct Elf64 {
    header: Elf64Header,
    section_headers: Vec<Elf64SectionHeader>,
    #[allow(dead_code)]
    program_headers: Vec<Elf64ProgramHeader>,
    // TODO: can empty or multiple? confirm spec
    /// symbol name table
    str: Mutex<Option<Elf64SectionHeader>>,
    /// section name table
    shstr: Mutex<Option<Elf64SectionHeader>>,
    sym: Mutex<Option<Elf64SectionHeader>>,
    // TODO: remove this
    // copy of ELF64 binary data
    buf: Vec<u8>,
}

impl Elf64 {
    pub fn new(
        header: Elf64Header,
        section_headers: Vec<Elf64SectionHeader>,
        program_headers: Vec<Elf64ProgramHeader>,
        shstr: Option<Elf64SectionHeader>,
        buf: Vec<u8>,
    ) -> Self {
        Self {
            header,
            section_headers,
            program_headers,
            str: Mutex::new(None),
            shstr: Mutex::new(shstr),
            sym: Mutex::new(None),
            buf,
        }
    }

    pub fn header(&self) -> Elf64Header {
        self.header.clone()
    }

    pub fn str(&self) -> Option<Elf64SectionHeader> {
        let str = self.str.lock().unwrap();
        if let Some(str) = &*str {
            return Some(str.clone());
        }

        let mut shstr = self.shstr.lock().unwrap();
        if let Some(_shstr) = &*shstr {
            for shdr in &self.section_headers {
                let sname_addr = (_shstr.sh_offset + shdr.sh_name as u64) as usize;
                let (_, sname) = nom::bytes::complete::take_till::<_, _, nom::error::Error<_>>(
                    |c| c == 0x00,
                )(&self.buf[sname_addr..])
                .unwrap();
                if sname == b".strtab" {
                    *shstr = Some(shdr.clone());
                    return Some(shdr.clone());
                }
            }
        }
        None
    }

    pub fn sym(&self) -> Option<Elf64SectionHeader> {
        let mut sym = self.sym.lock().unwrap();
        if let Some(sym) = &*sym {
            return Some(sym.clone());
        }

        for shdr in &self.section_headers {
            if shdr.sh_type == 0x02 {
                *sym = Some(shdr.clone());
                return Some(shdr.clone());
            }
        }
        None
    }

    //
    // show_*
    //

    pub fn show_section_names(&self) {
        // TODO: unwrap()
        let shstr = self.shstr.lock().unwrap();
        let shstr = (*shstr).as_ref().unwrap();

        for shdr in &self.section_headers {
            let name_addr = (shstr.sh_offset + shdr.sh_name as u64) as usize;
            let (_, name) =
                nom::bytes::complete::take_till::<_, _, nom::error::Error<_>>(|c| c == 0x00)(
                    &self.buf[name_addr..],
                )
                .unwrap();
            println!("name: {}", String::from_utf8_lossy(name));
        }
    }

    pub fn show_symbol_names(&self) {
        let endian = self.header().endian().unwrap();
        let sym = self.sym().unwrap();
        let str = self.str().unwrap();

        for i in 0..(sym.sh_size / sym.sh_entsize) {
            let addr = (sym.sh_offset + (sym.sh_entsize * i)) as usize;
            let (_, symp) = parse_elf64_symbol(&self.buf[addr..], endian).unwrap();

            if symp.st_name == 0 {
                continue;
            }

            let name = {
                let addr = (str.sh_offset + symp.st_name as u64) as usize;
                let (_, name) = nom::bytes::complete::take_till::<_, _, nom::error::Error<_>>(
                    |c| c == 0x00,
                )(&self.buf[addr..])
                .unwrap();
                String::from_utf8_lossy(name)
            };
            println!("\t{} [{:?}]\t{}", i, symp.st_type().unwrap(), name);
        }
    }
}
