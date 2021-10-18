#[derive(PartialEq, Copy, Clone)]
pub enum BitWidth {
    _32BIT,
    _64BIT,
    _128BIT,
}

impl Default for BitWidth {
    fn default() -> BitWidth {
        BitWidth::_32BIT
    }
}

impl BitWidth {
    pub fn to_num_bytes(&self) -> usize {
        match self {
            BitWidth::_32BIT => 4,
            BitWidth::_64BIT => 8,
            BitWidth::_128BIT => 16,
        }
    }
}

trait To32Bit {
    fn to_32_bit(&self) -> [u8; 4];
}

trait To64Bit {
    fn to_64_bit(&self) -> [u8; 8];
}

pub trait To128Bit {
    fn to_128_bit(&self) -> [u8; 16];
}

pub union Int32or64or128 {
    pub _u32: u32,
    pub _u64: u64,
    pub _u128: u128
}

pub mod _128bit {
    use super::*;

    pub trait ToStr {
        fn to_str(&self, signed: bool, bw: BitWidth) -> String;
    }

    impl ToStr for [u8; 16] {
        fn to_str(&self, signed: bool, bw: BitWidth) -> String {
            let u = Int32or64or128 { _u128: u128::from_ne_bytes(*self) };

            if signed {
                match bw {
                    BitWidth::_32BIT =>
                        format!("{}", unsafe { u._u32 } as i32),
                    BitWidth::_64BIT =>
                        format!("{}", unsafe { u._u64 } as i64),
                    BitWidth::_128BIT =>
                        format!("{}", unsafe { u._u128 } as i128),
                }
            } else {
                match bw {
                    BitWidth::_32BIT =>
                        format!("{}", unsafe { u._u32 }),
                    BitWidth::_64BIT =>
                        format!("{}", unsafe { u._u64 }),
                    BitWidth::_128BIT =>
                        format!("{}", unsafe { u._u128 }),
                }
            }
        }
    }

    pub trait ToBinStr {
        fn to_bin_str(&self, bw: BitWidth) -> String;
    }

    impl ToBinStr for [u8; 16] {
        fn to_bin_str(&self, bw: BitWidth) -> String {
            let u = Int32or64or128 { _u128: u128::from_ne_bytes(*self) };
            match bw {
                BitWidth::_32BIT =>
                    format!("{:b}", unsafe { u._u32 }),
                BitWidth::_64BIT =>
                    format!("{:b}", unsafe { u._u64 }),
                BitWidth::_128BIT =>
                    format!("{:b}", unsafe { u._u128 }),
            }
        }
    }

    impl To32Bit for &[u8; 16] {
        fn to_32_bit(&self) -> [u8; 4] {
            let mut bytes = [0u8; 4];
            self.iter()
                .take(4)
                .enumerate()
                .for_each(|(i, &b)| bytes[i] = b);
            bytes
        }
    }

    impl To64Bit for &[u8; 16] {
        fn to_64_bit(&self) -> [u8; 8] {
            let mut bytes = [0u8; 8];
            self.iter()
                .take(8)
                .enumerate()
                .for_each(|(i, &b)| bytes[i] = b);
            bytes
        }
    }

    pub trait ParseTo128Bit {
        fn parse_to_128bit(&self, signed: bool, bw: BitWidth) -> Result<[u8; 16], std::num::ParseIntError>;
    }

    impl ParseTo128Bit for &str {
        fn parse_to_128bit(&self, signed: bool, bw: BitWidth) -> Result<[u8; 16], std::num::ParseIntError> {
            let mut bytes = [0u8; 16];
            if signed {
                match bw {
                    BitWidth::_32BIT =>
                        i32::from_str_radix(self, 10)?
                            .to_ne_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, &b)| bytes[i] = b ),
                     BitWidth::_64BIT =>
                        i64::from_str_radix(self, 10)?
                            .to_ne_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, &b)| bytes[i] = b ),
                      BitWidth::_128BIT =>
                        i128::from_str_radix(self, 10)?
                            .to_ne_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, &b)| bytes[i] = b ),
                }
            } else {
                match bw {
                    BitWidth::_32BIT =>
                        u32::from_str_radix(self, 10)?
                            .to_ne_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, &b)| bytes[i] = b ),
                     BitWidth::_64BIT =>
                        u64::from_str_radix(self, 10)?
                            .to_ne_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, &b)| bytes[i] = b ),
                      BitWidth::_128BIT =>
                        u128::from_str_radix(self, 10)?
                            .to_ne_bytes()
                            .iter()
                            .enumerate()
                            .for_each(|(i, &b)| bytes[i] = b ),
                }
            }
            Ok(bytes)
        }
    }
}
