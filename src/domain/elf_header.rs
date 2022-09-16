mod elf_64_header;
mod elf_ident;
mod elf_magic;

pub use self::elf_64_header::*;
pub use self::elf_ident::*;
pub use self::elf_magic::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("my fuga error")]
    ConvertError,
}
