use num_traits::Signed;

/// Trait for adding a signed variant of a number to another
/// whilst checking for underflows
pub trait CheckedAddSigned where
    Self: Sized,
    Self::Signed: Signed
{
    type Signed;

    fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self>;
}

pub trait AbsDiff where
    Self: Sized
{
    type Unsigned;

    fn abs_diff(self, other: Self) -> Self::Unsigned;
}

macro_rules! impl_num_traits {
    ($unsigned:ty, $signed:ty) => {
        impl CheckedAddSigned for $signed {
            type Signed = $signed;

            fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self> {
                self.checked_add(rhs)
            }
        }

        impl CheckedAddSigned for $unsigned {
            type Signed = $signed;

            fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self> {
                <$unsigned>::checked_add_signed(self, rhs)
            }
        }

        impl AbsDiff for $signed {
            type Unsigned = $unsigned;

            fn abs_diff(self, rhs: Self) -> Self::Unsigned {
                self.abs_diff(rhs)
            }
        }

        impl AbsDiff for $unsigned {
            type Unsigned = $unsigned;

            fn abs_diff(self, rhs: Self) -> Self::Unsigned {
                self.abs_diff(rhs)
            }
        }
    }
}

impl_num_traits!(u8, i8);
impl_num_traits!(u16, i16);
impl_num_traits!(u32, i32);
impl_num_traits!(u64, i64);
impl_num_traits!(u128, i128);
impl_num_traits!(usize, isize);