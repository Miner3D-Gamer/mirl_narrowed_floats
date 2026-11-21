use crate::PositiveNonNan;

/// A value that is not NaN
#[mirl_macros::derive_all(
    read_only = true,
    bevy_reflect = false,
    // partial_ord = false,
    ord = false,
    eq = false,
    hash = false
)]
#[repr(transparent)]
pub struct NonZeroPositive<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan> {
    /// The internal value
    pub _value: T,
}
#[cfg(feature = "serde")]
impl<
    'de,
    T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan + serde::Deserialize<'de>,
> serde::Deserialize<'de> for NonZeroPositive<T>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let t = T::deserialize(deserializer)?;

        Self::new(t).map_or_else(|| Err(D::Error::custom("T is not a valid value.")), Ok)
    }
}

const impl<
    T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan + [const] core::marker::Destruct,
> NonZeroPositive<T>
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
        + [const] mirl_extensions_math::IsNan
        + [const] core::marker::Destruct,
> NonZeroPositive<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if val.is_positive() && !val.is_nan() {
            Some(Self { _value: val })
        } else {
            None
        }
    }
}

impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan> NonZeroPositive<T> {
    /// Take a ref of the wrapper to a ref of T
    pub const fn as_inner(&self) -> &T {
        &self._value
    }

    /// Take a mutable ref of the wrapper to a mutable ref of T
    pub const fn as_inner_mut(&mut self) -> &mut T {
        &mut self._value
    }
}
impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan> AsRef<T>
    for NonZeroPositive<T>
{
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan> AsMut<T>
    for NonZeroPositive<T>
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan> core::ops::Deref
    for NonZeroPositive<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan> core::ops::DerefMut
    for NonZeroPositive<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type NonZeroPositiveF16 = NonZeroPositive<f16>;
/// A f32 that cannot be nan
pub type NonZeroPositiveF32 = NonZeroPositive<f32>;
/// A f64 that cannot be nan
pub type NonZeroPositiveF64 = NonZeroPositive<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type NonZeroPositiveF128 = NonZeroPositive<f128>;

// const impl<
//     T: [const] core::cmp::PartialOrd
//         + mirl_extensions_math::GetPositiveInfinity
//         + mirl_extensions_math::IsNan
//         + mirl_extensions_core::Zero,
// > core::cmp::PartialOrd for NonZeroPositive<T>
// {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() })
//     }
// }
const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + mirl_extensions_math::IsNan,
> core::cmp::Ord for NonZeroPositive<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

// const impl<
//     T: [const] core::cmp::PartialEq
//         + mirl_extensions_core::Zero
//         + mirl_extensions_math::IsNan,
// > core::cmp::PartialEq for NonZeroPositive<T>
// {
//     fn eq(&self, other: &Self) -> bool {
//         self._value == other._value
//     }
// }
const impl<
    T: [const] core::cmp::PartialEq + mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan,
> core::cmp::Eq for NonZeroPositive<T>
{
}

impl<T: mirl_extensions_core::IsPositive + std::fmt::Debug + mirl_extensions_math::IsNan>
    std::hash::Hash for NonZeroPositive<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}
impl<
    T: mirl_extensions_core::IsPositive
        + mirl_extensions_math::IsNan
        + std::cmp::PartialEq
        + mirl_extensions_core::Zero,
> AsRef<PositiveNonNan<T>> for NonZeroPositive<T>
{
    fn as_ref(&self) -> &PositiveNonNan<T> {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<crate::PositiveNonNan<T>>() }
    }
}
