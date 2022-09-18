mod elf_64_program_header;
mod elf_segment_flags;
mod elf_segment_type;

pub use self::elf_64_program_header::*;
pub use self::elf_segment_flags::*;
pub use self::elf_segment_type::*;

// pub trait ElfProgramHeader {
//     fn try_from(buf: &[u8])
// }
