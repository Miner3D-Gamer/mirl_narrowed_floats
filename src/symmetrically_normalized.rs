/// A value that is not NaN
#[mirl_macros::derive_all(
    bevy_reflect = false,
    ord = false,
    eq = false,
    hash = false,
    read_only = true
)]
#[repr(transparent)]
pub struct SymmetricallyNormalized<T: mirl_extensions_math::IsSymmetricallyNormalized> {
    /// The internal value
    pub _value: T,
}

const impl<
    T: [const] mirl_extensions_math::IsSymmetricallyNormalized
        + [const] std::default::Default
        + [const] std::marker::Destruct,
> Default for SymmetricallyNormalized<T>
{
    /// Please test this in debug mode as it may cause UB if untested in release mode
    fn default() -> Self {
        let default = Self::new(T::default());
        assert!(
            default.is_some(),
            "Default of T is not compatible with `SymmetricallyNormalized`"
        );
        unsafe { default.unwrap_unchecked() }
    }
}

const impl<T: mirl_extensions_math::IsSymmetricallyNormalized + [const] core::marker::Destruct> SymmetricallyNormalized<T> {
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
const impl<T: [const] mirl_extensions_math::IsSymmetricallyNormalized + [const] core::marker::Destruct>
    SymmetricallyNormalized<T>
{
    /// Create a new instance of this wrapper if the internal value isn't Nan
    pub fn new(val: T) -> Option<Self> {
        if val.is_symmetrically_normalized() {
            Some(Self { _value: val })
        } else {
            None
        }
    }
}

impl<T: mirl_extensions_math::IsSymmetricallyNormalized> SymmetricallyNormalized<T> {
    /// Take a ref of the wrapper to a ref of T
    pub const fn as_inner(&self) -> &T {
        &self._value
    }

    /// Take a mutable ref of the wrapper to a mutable ref of T
    pub const fn as_inner_mut(&mut self) -> &mut T {
        &mut self._value
    }
}
impl<T: mirl_extensions_math::IsSymmetricallyNormalized> AsRef<T> for SymmetricallyNormalized<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*std::ptr::from_ref::<Self>(self).cast::<T>() }
    }
}
impl<T: mirl_extensions_math::IsSymmetricallyNormalized> AsMut<T> for SymmetricallyNormalized<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *std::ptr::from_mut::<Self>(self).cast::<T>() }
    }
}

impl<T: mirl_extensions_math::IsSymmetricallyNormalized> core::ops::Deref for SymmetricallyNormalized<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
impl<T: mirl_extensions_math::IsSymmetricallyNormalized> core::ops::DerefMut for SymmetricallyNormalized<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}
#[cfg(feature = "f16")]
/// A f16 that cannot be nan
pub type SymmetricallyNormalizedF16 = SymmetricallyNormalized<f16>;
/// A f32 that cannot be nan
pub type SymmetricallyNormalizedF32 = SymmetricallyNormalized<f32>;
/// A f64 that cannot be nan
pub type SymmetricallyNormalizedF64 = SymmetricallyNormalized<f64>;
#[cfg(feature = "f128")]
/// A f128 that cannot be nan
pub type SymmetricallyNormalizedF128 = SymmetricallyNormalized<f128>;

const impl<
    T: mirl_extensions_math::IsSymmetricallyNormalized
        + [const] core::cmp::PartialEq
        + [const] core::cmp::PartialOrd,
> core::cmp::Ord for SymmetricallyNormalized<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self._value.partial_cmp(&other._value).unwrap_unchecked() }
    }
}

const impl<T: mirl_extensions_math::IsSymmetricallyNormalized + [const] core::cmp::PartialEq> core::cmp::Eq
    for SymmetricallyNormalized<T>
{
}

impl<T: mirl_extensions_math::IsSymmetricallyNormalized + std::cmp::PartialOrd + std::fmt::Debug> std::hash::Hash
    for SymmetricallyNormalized<T>
{
    // TODO: This cannot be the best way
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_str(&format!("{:?}", self._value));
    }
}

// impl<
//     T: mirl_extensions_core::Zero + mirl_extensions_math::GetPositiveInfinity + std::cmp::PartialOrd,
// > core::convert::Into<T> for SymmetricallyNormalized<T>
// {
//     fn into(self) -> T {
//         self._value
//     }
// }
