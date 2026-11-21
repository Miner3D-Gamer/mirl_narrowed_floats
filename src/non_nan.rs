/// A value that is not NaN
#[mirl_macros::derive_all(
    read_only = true,
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false
)]
#[repr(transparent)]
pub struct NonNan<T: mirl_extensions_math::IsNan> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: [const] mirl_extensions_math::IsNan
        + [const] std::default::Default
        + [const] std::marker::Destruct,
> Default for NonNan<T>
{
    /// Please test this in debug mode as it may cause UB if untested in release mode
    fn default() -> Self {
        let default = Self::new(T::default());
        assert!(
            default.is_some(),
            "Default of T is not compatible with `NonNan`"
        );
        unsafe { default.unwrap_unchecked() }
    }
}

const impl<T: mirl_extensions_math::IsNan + [const] core::marker::Destruct> NonNan<T> {
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
const impl<T: [const] mirl_extensions_math::IsNan + [const] core::marker::Destruct> NonNan<T> {
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if val.is_nan() {
            None
        } else {
            Some(Self { _value: val })
        }
    }
}

impl<T: mirl_extensions_math::IsNan> NonNan<T> {
    /// Take a ref of the wrapper to a ref of T
    pub const fn as_inner(&self) -> &T {
        &self._value
    }

    /// Take a mutable ref of the wrapper to a mutable ref of T
    pub const fn as_inner_mut(&mut self) -> &mut T {
        &mut self._value
    }
}
impl<T: mirl_extensions_math::IsNan> AsRef<T> for NonNan<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<T: mirl_extensions_math::IsNan> AsMut<T> for NonNan<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<T: mirl_extensions_math::IsNan> core::ops::Deref for NonNan<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: mirl_extensions_math::IsNan> core::ops::DerefMut for NonNan<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type NonNanF16 = NonNan<f16>;
/// A f32 that cannot be nan
pub type NonNanF32 = NonNan<f32>;
/// A f64 that cannot be nan
pub type NonNanF64 = NonNan<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type NonNanF128 = NonNan<f128>;

const impl<
    T: [const] core::cmp::PartialOrd
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + mirl_extensions_math::IsNan,
> core::cmp::Ord for NonNan<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<
    T: [const] core::cmp::PartialEq
        + mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + std::cmp::PartialOrd
        + mirl_extensions_math::IsNan,
> core::cmp::Eq for NonNan<T>
{
}

impl<
    T: mirl_extensions_core::Zero
        + mirl_extensions_math::GetPositiveInfinity
        + std::cmp::PartialOrd
        + std::fmt::Debug
        + mirl_extensions_math::IsNan,
> std::hash::Hash for NonNan<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}
