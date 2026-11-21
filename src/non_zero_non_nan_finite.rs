/// A value that is not zero and not NaN and not infinity
#[mirl_macros::derive_all(
    read_only = true,
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false
)]
#[repr(transparent)]
pub struct NonZeroNonNanInfinite<
    T: mirl_extensions_math::IsNan + mirl_extensions_core::IsZero + mirl_extensions_math::IsInfinite,
> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: mirl_extensions_math::IsNan
        + mirl_extensions_core::IsZero
        + mirl_extensions_math::IsInfinite
        + [const] core::marker::Destruct,
> NonZeroNonNanInfinite<T>
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
    T: [const] mirl_extensions_math::IsNan
        + [const] mirl_extensions_core::IsZero
        + [const] mirl_extensions_math::IsInfinite
        + [const] core::marker::Destruct,
> NonZeroNonNanInfinite<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if (val.is_nan() || val.is_zero()) && !val.is_infinity() {
            None
        } else {
            Some(Self { _value: val })
        }
    }
}

impl<
    T: mirl_extensions_math::IsNan + mirl_extensions_core::IsZero + mirl_extensions_math::IsInfinite,
> NonZeroNonNanInfinite<T>
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
    T: mirl_extensions_math::IsNan + mirl_extensions_core::IsZero + mirl_extensions_math::IsInfinite,
> AsRef<T> for NonZeroNonNanInfinite<T>
{
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<
    T: mirl_extensions_math::IsNan + mirl_extensions_core::IsZero + mirl_extensions_math::IsInfinite,
> AsMut<T> for NonZeroNonNanInfinite<T>
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<
    T: mirl_extensions_math::IsNan + mirl_extensions_core::IsZero + mirl_extensions_math::IsInfinite,
> core::ops::Deref for NonZeroNonNanInfinite<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<
    T: mirl_extensions_math::IsNan + mirl_extensions_core::IsZero + mirl_extensions_math::IsInfinite,
> core::ops::DerefMut for NonZeroNonNanInfinite<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type NonZeroNonNanInfiniteF16 = NonZeroNonNanInfinite<f16>;
/// A f32 that cannot be nan
pub type NonZeroNonNanInfiniteF32 = NonZeroNonNanInfinite<f32>;
/// A f64 that cannot be nan
pub type NonZeroNonNanInfiniteF64 = NonZeroNonNanInfinite<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type NonZeroNonNanInfiniteF128 = NonZeroNonNanInfinite<f128>;

const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_core::Zero
        + mirl_extensions_math::IsNan
        + mirl_extensions_math::IsInfinite,
> core::cmp::Ord for NonZeroNonNanInfinite<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<
    T: [const] core::cmp::PartialEq
        + mirl_extensions_core::Zero
        + mirl_extensions_math::IsNan
        + mirl_extensions_math::IsInfinite
        + std::cmp::PartialOrd,
> core::cmp::Eq for NonZeroNonNanInfinite<T>
{
}

impl<
    T: mirl_extensions_core::Zero
        + mirl_extensions_math::IsInfinite
        + std::cmp::PartialOrd
        + mirl_extensions_math::IsNan
        + std::fmt::Debug,
> std::hash::Hash for NonZeroNonNanInfinite<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}
