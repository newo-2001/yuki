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

macro_rules! impl_checked_add_signed {
    ($type:ty, $signed:ty) => {
        impl CheckedAddSigned for $type {
            type Signed = $signed;

            fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self> {
                <$type>::checked_add_signed(self, rhs)
            }
        }
    };

    ($type:ty) => {
        impl CheckedAddSigned for $type {
            type Signed = $type;

            fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self> {
                self.checked_add(rhs)
            }
        }
    }
}

impl_checked_add_signed!(u8, i8);
impl_checked_add_signed!(u16, i16);
impl_checked_add_signed!(u32, i32);
impl_checked_add_signed!(u64, i64);
impl_checked_add_signed!(u128, i128);
impl_checked_add_signed!(usize, isize);
impl_checked_add_signed!(i8);
impl_checked_add_signed!(i16);
impl_checked_add_signed!(i32);
impl_checked_add_signed!(i64);
impl_checked_add_signed!(i128);
impl_checked_add_signed!(isize);