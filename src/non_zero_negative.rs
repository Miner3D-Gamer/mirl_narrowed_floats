/// A value that is not NaN
#[mirl_macros::derive_all(
    read_only = true,
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false
)]
#[repr(transparent)]
pub struct NonZeroNegative<T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan + [const] core::marker::Destruct,
> NonZeroNegative<T>
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
        + [const] mirl_extensions_math::IsNan
        + [const] core::marker::Destruct,
> NonZeroNegative<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if val.is_negative() && !val.is_nan() {
            Some(Self { _value: val })
        } else {
            None
        }
    }
}

impl<T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan> NonZeroNegative<T> {
    /// Take a ref of the wrapper to a ref of T
    pub const fn as_inner(&self) -> &T {
        &self._value
    }

    /// Take a mutable ref of the wrapper to a mutable ref of T
    pub const fn as_inner_mut(&mut self) -> &mut T {
        &mut self._value
    }
}
impl<T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan> AsRef<T>
    for NonZeroNegative<T>
{
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan> AsMut<T>
    for NonZeroNegative<T>
{
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan> core::ops::Deref
    for NonZeroNegative<T>
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: mirl_extensions_core::IsNegative + mirl_extensions_math::IsNan> core::ops::DerefMut
    for NonZeroNegative<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type NonZeroNegativeF16 = NonZeroNegative<f16>;
/// A f32 that cannot be nan
pub type NonZeroNegativeF32 = NonZeroNegative<f32>;
/// A f64 that cannot be nan
pub type NonZeroNegativeF64 = NonZeroNegative<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type NonZeroNegativeF128 = NonZeroNegative<f128>;

const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_math::GetNegativeInfinity
        + mirl_extensions_math::IsNan
        + mirl_extensions_core::Zero,
> core::cmp::Ord for NonZeroNegative<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<
    T: [const] core::cmp::PartialEq
        + mirl_extensions_math::GetNegativeInfinity
        + mirl_extensions_math::IsNan
        + std::cmp::PartialOrd
        + mirl_extensions_core::Zero,
> core::cmp::Eq for NonZeroNegative<T>
{
}

impl<
    T: mirl_extensions_math::GetNegativeInfinity
        + mirl_extensions_math::IsNan
        + std::cmp::PartialOrd
        + std::fmt::Debug
        + mirl_extensions_core::Zero,
> std::hash::Hash for NonZeroNegative<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}
