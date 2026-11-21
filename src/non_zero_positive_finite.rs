/// A value that is not NaN
#[mirl_macros::derive_all(
    read_only = true,
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false
)]
#[repr(transparent)]
pub struct NonZeroPositiveFinite<
    T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsPositiveInfinity,
> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: mirl_extensions_core::IsPositive
        + mirl_extensions_math::IsPositiveInfinity
        + [const] core::marker::Destruct,
> NonZeroPositiveFinite<T>
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
        + [const] mirl_extensions_math::IsPositiveInfinity
        + [const] mirl_extensions_math::IsNan
        + [const] core::marker::Destruct,
> NonZeroPositiveFinite<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if val.is_positive() && !val.is_positive_infinity() && !val.is_nan() {
            Some(Self { _value: val })
        } else {
            None
        }
    }
}

impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsPositiveInfinity>
    NonZeroPositiveFinite<T>
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
impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsPositiveInfinity> AsRef<T>
    for NonZeroPositiveFinite<T>
{
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsPositiveInfinity> AsMut<T>
    for NonZeroPositiveFinite<T>
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsPositiveInfinity>
    core::ops::Deref for NonZeroPositiveFinite<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsPositiveInfinity>
    core::ops::DerefMut for NonZeroPositiveFinite<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type NonZeroPositiveFiniteF16 = NonZeroPositiveFinite<f16>;
/// A f32 that cannot be nan
pub type NonZeroPositiveFiniteF32 = NonZeroPositiveFinite<f32>;
/// A f64 that cannot be nan
pub type NonZeroPositiveFiniteF64 = NonZeroPositiveFinite<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type NonZeroPositiveFiniteF128 = NonZeroPositiveFinite<f128>;

const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity,
> core::cmp::Ord for NonZeroPositiveFinite<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<
    T: [const] core::cmp::PartialEq
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + std::cmp::PartialOrd,
> core::cmp::Eq for NonZeroPositiveFinite<T>
{
}

impl<
    T: mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + std::cmp::PartialOrd
        + std::fmt::Debug,
> std::hash::Hash for NonZeroPositiveFinite<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}
