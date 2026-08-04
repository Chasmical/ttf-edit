use crate::types::impl_fmt_from_getter;

macro_rules! impl_fixed_point_number {
    (
        $(#[$outer:meta])*
        $vis:vis struct $Name:ident(
            $int:ty as [u8; $bytes:literal];
            $integer_bits:literal | $fraction_bits:literal as $fp:ty
        );
    ) => {
        $(#[$outer])*
        #[derive(Copy, Hash)]
        #[derive_const(Clone, PartialEq, Eq)]
        #[repr(transparent)]
        $vis struct $Name([u8; $bytes]);

        const _: () = {
            assert!(size_of::<$int>() == $bytes);
            assert!($integer_bits + $fraction_bits == <$int>::BITS);
        };

        impl $Name {
            const STEP: $fp = 1.0 / (1 << $fraction_bits) as $fp;
            const MIN: $fp = -(1 << $integer_bits) as $fp;
            const MAX: $fp = ((1 << $integer_bits) - 1) as $fp;

            pub const fn new(num: $fp) -> Option<Self> {
                if matches!(num, Self::MIN..Self::MAX) {
                    Some(unsafe { Self::new_unchecked(num) })
                } else {
                    None
                }
            }
            pub const unsafe fn new_unchecked(num: $fp) -> Self {
                debug_assert!(matches!(num, Self::MIN..Self::MAX));
                Self(((num / Self::STEP).round() as $int).to_be_bytes())
            }

            pub const fn from_be_bytes(bytes: [u8; $bytes]) -> Self {
                Self(bytes)
            }
            pub const fn to_be_bytes(self) -> [u8; $bytes] {
                self.0
            }
            pub const fn as_be_bytes(&self) -> &[u8; $bytes] {
                &self.0
            }

            pub const fn to_frac_num(&self) -> $int {
                <$int>::from_be_bytes(self.0)
            }
            pub const fn get(&self) -> $fp {
                self.to_frac_num() as $fp * Self::STEP
            }
        }

        impl_fmt_from_getter! { Debug, Display, LowerExp, UpperExp for $Name }

        const impl PartialOrd for $Name {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        const impl Ord for $Name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.to_frac_num().cmp(&other.to_frac_num())
            }
        }

        const impl PartialEq<$fp> for $Name {
            fn eq(&self, other: &$fp) -> bool {
                self.get().eq(other)
            }
        }
        const impl PartialOrd<$fp> for $Name {
            fn partial_cmp(&self, other: &$fp) -> Option<std::cmp::Ordering> {
                self.get().partial_cmp(other)
            }
        }

        impl std::str::FromStr for $Name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$fp>::from_str(s).or(Err(())).and_then($Name::try_from)
            }
        }
        const impl TryFrom<$fp> for $Name {
            type Error = ();
            fn try_from(value: $fp) -> Result<$Name, Self::Error> {
                Self::new(value).ok_or(())
            }
        }
    }
}

impl_fixed_point_number! {
    pub struct Fixed(i32 as [u8; 4]; 16|16 as f64);
}
impl_fixed_point_number! {
    pub struct F2DOT14(i16 as [u8; 2]; 2|14 as f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed() {
        let nums: [(u32, f64); _] = [
            (0x7FFF_FFFF, 32767.999985),
            (0x7FFF_FF00, 32767.996094),
            (0x7FFF_2000, 32767.125000),
            (0x7FFF_0000, 32767.000000),
            (0x0040_0100, 64.003906),
            (0x0040_0000, 64.000000),
            (0x0001_0000, 1.000000),
            (0x0000_0001, 0.000015),
            (0x0000_0000, 0.000000),
            (0xFFFF_0000, -1.000000),
            (0xFFBF_FF00, -64.003906),
            (0x8000_0000, -32768.000000),
        ];

        for (raw, fp) in nums {
            let real = Fixed::new(fp).unwrap().to_frac_num() as u32;
            assert_eq!(real, raw, "{real:#X} != {raw:#X} ({fp})");

            let real = Fixed::new(fp).unwrap().get();
            let diff = (real - fp).abs();
            assert!(diff <= 0.1 * Fixed::STEP, "{real} != {fp} (Δ={diff})");
        }
    }

    #[test]
    fn f2dot14() {
        let nums: [(u16, f32); _] = [
            (0x7FFF, 1.999939),
            (0x7000, 1.750000),
            (0x0085, 0.008118),
            (0x0002, 0.000122),
            (0x0001, 0.000061),
            (0x0000, 0.000000),
            (0xFFFF, -0.000061),
            (0xFFFE, -0.000122),
            (0xFF7B, -0.008118),
            (0x8000, -2.000000),
        ];

        for (raw, fp) in nums {
            let real = F2DOT14::new(fp).unwrap().to_frac_num() as u16;
            assert_eq!(real, raw, "{real:#X} != {raw:#X} ({fp})");

            let real = F2DOT14::new(fp).unwrap().get();
            let diff = (real - fp).abs();
            assert!(diff <= 0.1 * F2DOT14::STEP, "{real} != {fp} (Δ={diff})");
        }
    }
}
