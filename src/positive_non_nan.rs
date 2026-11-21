use crate::NonZeroPositive;

/// A value that is not NaN
#[mirl_macros::derive_all(
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false,
    read_only = true
)]
#[repr(transparent)]
pub struct PositiveNonNan<
    T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan + mirl_extensions_core::IsZero,
> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: [const] mirl_extensions_core::IsPositive
        + [const] mirl_extensions_math::IsNan
        + [const] mirl_extensions_core::IsZero
        + [const] std::default::Default
        + [const] std::marker::Destruct,
> Default for PositiveNonNan<T>
{
    /// Please test this in debug mode as it may cause UB if untested in release mode
    fn default() -> Self {
        let default = Self::new(T::default());
        assert!(
            default.is_some(),
            "Default of T is not compatible with `PositiveNonNan`"
        );
        unsafe { default.unwrap_unchecked() }
    }
}

const impl<
    T: mirl_extensions_core::IsPositive
        + mirl_extensions_math::IsNan
        + mirl_extensions_core::IsZero
        + [const] core::marker::Destruct,
> PositiveNonNan<T>
{
    /// Turn the wrapper of T into T
    pub fn into_inner(self) -> T {
        self._value
    }
    #[deprecated(
        note = "Please use `into_inner` instead. This function only exists for compatibility purposes.",
        since = "0.0.0-alpha"
    )]
    /// Turn the wrapper of T into T
    pub fn get(self) -> T {
        self._value
    }
}
const impl<
    T: [const] mirl_extensions_core::IsPositive
        + [const] mirl_extensions_core::IsZero
        + [const] mirl_extensions_math::IsNan
        + [const] core::marker::Destruct,
> PositiveNonNan<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if (val.is_positive() || val.is_zero()) && !val.is_nan() {
            Some(Self { _value: val })
        } else {
            None
        }
    }
}

impl<
    T: mirl_extensions_core::IsPositive + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> PositiveNonNan<T>
{
    /// Take a ref of the wrapper to a ref of T
    pub const fn as_inner(&self) -> &T {
        &self._value
    }

    /// Take a mutable ref of the wrapper to a mutable ref of T
    pub const fn as_inner_mut(&mut self) -> &mut T {
        &mut self._value
    }
}
impl<
    T: mirl_extensions_core::IsPositive + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> AsRef<T> for PositiveNonNan<T>
{
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<
    T: mirl_extensions_core::IsPositive + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> AsMut<T> for PositiveNonNan<T>
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<
    T: mirl_extensions_core::IsPositive + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> core::ops::Deref for PositiveNonNan<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<
    T: mirl_extensions_core::IsPositive + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> core::ops::DerefMut for PositiveNonNan<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type PositiveNonNanF16 = PositiveNonNan<f16>;
/// A f32 that cannot be nan
pub type PositiveNonNanF32 = PositiveNonNan<f32>;
/// A f64 that cannot be nan
pub type PositiveNonNanF64 = PositiveNonNan<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type PositiveNonNanF128 = PositiveNonNan<f128>;

const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_core::Zero
        + mirl_extensions_math::IsNan
        + mirl_extensions_math::GetPositiveInfinity,
> core::cmp::Ord for PositiveNonNan<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<
    T: [const] core::cmp::PartialEq
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + mirl_extensions_math::IsNan
        + std::cmp::PartialOrd,
> core::cmp::Eq for PositiveNonNan<T>
{
}

impl<
    T: mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + std::cmp::PartialOrd
        + mirl_extensions_math::IsNan
        + std::fmt::Debug,
> std::hash::Hash for PositiveNonNan<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}

// impl<
//     T: mirl_extensions_core::Zero + mirl_extensions_math::GetPositiveInfinity + std::cmp::PartialOrd,
// > core::convert::Into<T> for PositiveNonNan<T>
// {
//     fn into(self) -> T {
//         self._value
//     }
// }

impl<
    T: mirl_extensions_core::IsPositive
        + mirl_extensions_math::IsNan
        + std::cmp::PartialEq
        + mirl_extensions_core::Zero
        + Sized,
> From<NonZeroPositive<T>> for PositiveNonNan<T>
{
    fn from(value: NonZeroPositive<T>) -> Self {
        Self {
            _value: value._value,
        }
    }
}
