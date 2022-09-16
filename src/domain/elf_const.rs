// typedef __u64	Elf64_Addr;
pub(crate) type Elf64Addr = u64;

// typedef __u16	Elf64_Half;
pub(crate) type Elf64Half = u16;

// typedef __s16	Elf64_SHalf;
#[allow(dead_code)]
pub(crate) type Elf64SHalf = i16;

// typedef __u64	Elf64_Off;
pub(crate) type Elf64Off = u64;

// typedef __s32	Elf64_Sword;
#[allow(dead_code)]
pub(crate) type Elf64SWord = i32;

// typedef __u32	Elf64_Word;
#[allow(dead_code)]
pub(crate) type Elf64Word = u32;

// typedef __u64	Elf64_Xword;
#[allow(dead_code)]
pub(crate) type Elf64XWord = u64;

// typedef __s64	Elf64_Sxword;
#[allow(dead_code)]
pub(crate) type Elf64SXWord = i64;

#[allow(dead_code)]
pub const EI_NIDENT: usize = 16;
