/// A value that is not NaN
#[mirl_macros::derive_all(
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false,
    read_only = true
)]
#[repr(transparent)]
pub struct NegativeNonNan<
    T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan + mirl_extensions_core::IsZero,
> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: [const] mirl_extensions_core::IsNegative
        + [const] mirl_extensions_math::IsNan
        + [const] mirl_extensions_core::IsZero
        + [const] std::default::Default
        + [const] std::marker::Destruct,
> Default for NegativeNonNan<T>
{
    /// Please test this in debug mode as it may cause UB if untested in release mode
    fn default() -> Self {
        let default = Self::new(T::default());
        assert!(
            default.is_some(),
            "Default of T is not compatible with `NegativeNonNan`"
        );
        unsafe { default.unwrap_unchecked() }
    }
}
const impl<
    T: mirl_extensions_core::IsNegative
        + mirl_extensions_math::IsNan
        + mirl_extensions_core::IsZero
        + [const] core::marker::Destruct,
> NegativeNonNan<T>
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
    T: [const] mirl_extensions_core::IsNegative
        + [const] mirl_extensions_core::IsZero
        + [const] mirl_extensions_math::IsNan
        + [const] core::marker::Destruct,
> NegativeNonNan<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if (val.is_negative() || val.is_zero()) && !val.is_nan() {
            Some(Self { _value: val })
        } else {
            None
        }
    }
}

impl<
    T: mirl_extensions_core::IsNegative + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> NegativeNonNan<T>
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
    T: mirl_extensions_core::IsNegative + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> AsRef<T> for NegativeNonNan<T>
{
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<
    T: mirl_extensions_core::IsNegative + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> AsMut<T> for NegativeNonNan<T>
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<
    T: mirl_extensions_core::IsNegative + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> core::ops::Deref for NegativeNonNan<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<
    T: mirl_extensions_core::IsNegative + mirl_extensions_core::IsZero + mirl_extensions_math::IsNan,
> core::ops::DerefMut for NegativeNonNan<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type NegativeNonNanF16 = NegativeNonNan<f16>;
/// A f32 that cannot be nan
pub type NegativeNonNanF32 = NegativeNonNan<f32>;
/// A f64 that cannot be nan
pub type NegativeNonNanF64 = NegativeNonNan<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type NegativeNonNanF128 = NegativeNonNan<f128>;

const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_core::Zero
        + mirl_extensions_math::IsNan
        + mirl_extensions_math::GetNegativeInfinity,
> core::cmp::Ord for NegativeNonNan<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<
    T: [const] core::cmp::PartialEq
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetNegativeInfinity
        + mirl_extensions_math::IsNan
        + std::cmp::PartialOrd,
> core::cmp::Eq for NegativeNonNan<T>
{
}

impl<
    T: mirl_extensions_core::Zero
        + mirl_extensions_math::GetNegativeInfinity
        + std::cmp::PartialOrd
        + mirl_extensions_math::IsNan
        + std::fmt::Debug,
> std::hash::Hash for NegativeNonNan<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}

// impl<
//     T: mirl_extensions_core::Zero + mirl_extensions_math::GetNegativeInfinity + std::cmp::PartialOrd,
// > core::convert::Into<T> for NegativeNonNan<T>
// {
//     fn into(self) -> T {
//         self._value
//     }
// }
