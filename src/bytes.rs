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

pub trait To32Bit {
    fn to_32_bit(&self) -> [u8; 4];
}

pub trait To64Bit {
    fn to_64_bit(&self) -> [u8; 8];
}

pub trait To128Bit {
    fn to_128_bit(&self) -> [u8; 16];
}

pub mod _128bit {
    use super::*;
    use super::_32bit::*;
    use super::_64bit::*;

    pub trait ToStr {
        fn to_str(&self, signed: bool, bw: BitWidth) -> String;
    }

    impl ToStr for [u8; 16] {
        fn to_str(&self, signed: bool, bw: BitWidth) -> String {
            if signed {
                match bw {
                    BitWidth::_32BIT =>
                        format!("{}", self.to_32_bit().to_i32()),
                    BitWidth::_64BIT =>
                        format!("{}", self.to_64_bit().to_i64()),
                    BitWidth::_128BIT =>
                        format!("{}", self.to_i128()),
                }
            } else {
                match bw {
                    BitWidth::_32BIT =>
                        format!("{}", self.to_32_bit().to_u32()),
                    BitWidth::_64BIT =>
                        format!("{}", self.to_64_bit().to_u64()),
                    BitWidth::_128BIT =>
                        format!("{}", self.to_u128()),
                }
            }
        }
    }

    pub trait ToBinStr {
        fn to_bin_str(&self, bw: BitWidth) -> String;
    }

    impl ToBinStr for [u8; 16] {
        fn to_bin_str(&self, bw: BitWidth) -> String {
            match bw {
                BitWidth::_32BIT =>
                    format!("{:b}", self.to_32_bit().to_u32()),
                BitWidth::_64BIT =>
                    format!("{:b}", self.to_64_bit().to_u64()),
                BitWidth::_128BIT =>
                    format!("{:b}", self.to_u128()),
            }
        }
    }

    pub trait ToU128 {
        fn to_u128(&self) -> u128;
    }

    pub trait ToI128 {
        fn to_i128(&self) -> i128;
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

    impl ToI128 for [u8; 16] {
        fn to_i128(&self) -> i128 {
            i128::from_ne_bytes(*self)
        }
    }

    impl ToU128 for [u8; 16] {
        fn to_u128(&self) -> u128 {
            u128::from_ne_bytes(*self)
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

pub mod _32bit {
    use super::*;

    impl To128Bit for u32 {
        fn to_128_bit(&self) -> [u8; 16] {
            let mut bytes = [0u8; 16];
            self.to_ne_bytes().iter()
                .enumerate()
                .for_each(|(i, &b)| bytes[i] = b);
            bytes
        }
    }

    pub trait ToI32 {
        fn to_i32(&self) -> i32;
    }

    impl ToI32 for [u8; 4] {
        fn to_i32(&self) -> i32 {
            i32::from_ne_bytes(*self)
        }
    }

    pub trait ToU32 {
        fn to_u32(&self) -> u32;
    }

    impl ToU32 for [u8; 4] {
        fn to_u32(&self) -> u32 {
            u32::from_ne_bytes(*self)
        }
    }
}

pub mod _64bit {
    pub trait ToI64 {
        fn to_i64(&self) -> i64;
    }

    impl ToI64 for [u8; 8] {
        fn to_i64(&self) -> i64 {
            i64::from_ne_bytes(*self)
        }
    }

    pub trait ToU64 {
        fn to_u64(&self) -> u64;
    }

    impl ToU64 for [u8; 8] {
        fn to_u64(&self) -> u64 {
            u64::from_ne_bytes(*self)
        }
    }
}
