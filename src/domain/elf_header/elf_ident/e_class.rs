#[derive(Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum EClass {
    Elfclassnone = 0x00, /* Invalid class */
    Elfclass32 = 0x01,   /* 32-bit objects */
    Elfclass64 = 0x02,   /* 64-bit objects */
    Elfclassnum = 0x03,
}

impl EClass {
    pub fn from_u8(n: u8) -> Option<EClass> {
        match n {
            0 => Some(EClass::Elfclassnone),
            1 => Some(EClass::Elfclass32),
            2 => Some(EClass::Elfclass64),
            3 => Some(EClass::Elfclassnum),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u8() {
        let e_class = EClass::from_u8(2);
        assert_eq!(e_class, Some(EClass::Elfclass64));
    }
}
