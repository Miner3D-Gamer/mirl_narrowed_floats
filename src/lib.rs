//! A crate to help with limiting the behavior of numbers
//!
// This crate is closer to `mirl/extensions` than `mirl/math` but oh well
#![feature(derive_const)]
#![feature(const_cmp)]
#![feature(const_clone)]
#![feature(const_trait_impl)]
#![feature(const_destruct)]
#![cfg_attr(feature = "f128", feature(f128))]
#![cfg_attr(feature = "f16", feature(f16))]
#![allow(clippy::derive_ord_xor_partial_ord)]
#![feature(hasher_prefixfree_extras)]
#![feature(const_default)]
// TODO: Rewrite this lib using a proc macro for maximal code deduplication

// TODO²: 
// Impl custom deserialization for every type
// #[cfg(feature = "serde")]
// impl<
//     'de,
//     T: mirl_extensions_core::IsPositive + mirl_extensions_math::IsNan + serde::Deserialize<'de>,
// > serde::Deserialize<'de> for NonZeroPositive<T>
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         use serde::de::Error;
//         let t = T::deserialize(deserializer)?;

//         Self::new(t).map_or_else(|| Err(D::Error::custom("T is not a valid value.")), Ok)
//     }
// }



mod non_nan;
pub use non_nan::*;

mod non_zero_non_nan;
pub use non_zero_non_nan::*;

mod non_nan_finite;
pub use non_nan_finite::*;

mod non_zero_non_nan_finite;
pub use non_zero_non_nan_finite::*;

mod positive;
pub use positive::*;

mod positive_finite;
pub use positive_finite::*;

mod non_zero_positive_finite;
pub use non_zero_positive_finite::*;

mod non_zero_positive;
pub use non_zero_positive::*;

mod negative;
pub use negative::*;

mod negative_finite;
pub use negative_finite::*;

mod non_zero_negative_finite;
pub use non_zero_negative_finite::*;

mod non_zero_negative;
pub use non_zero_negative::*;

mod positive_non_nan;
pub use positive_non_nan::*;

mod negative_non_nan;
pub use negative_non_nan::*;

mod normalized;
pub use normalized::*;

mod symmetrically_normalized;
pub use symmetrically_normalized::*;
