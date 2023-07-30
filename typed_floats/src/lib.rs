#![doc = include_str!("../README.md")]
//! # Rules
//!
//! Conversions rules for operations are summarized in [`Float`].
//!
//! # Examples
//!
//! Operations will return the strictest type possible.
//!
//! ```
//! use typed_floats::*;
//!
//! let a: StrictlyPositiveFinite = 1.0f64.try_into().unwrap();
//! let b: StrictlyNegativeFinite = (-1.0f64).try_into().unwrap();
//!
//! let c: StrictlyPositive = a - b;
//! let d: NonNaNFinite = a + b;
//!
//! assert_eq!(c.get(), 2.0f64);
//! assert_eq!(d.get(), 0.0f64);   
//! ```
//!
//! ```
//! use typed_floats::*;
//!
//! let a: StrictlyPositiveFinite = 1.0f64.try_into().unwrap();
//! let b: Positive = 0.0f64.try_into().unwrap();
//!
//! let c = a + b;
//!
//! assert_eq!(c, StrictlyPositive::try_from(1.0f64).unwrap());   
//! ```
//!
//! Operations that assign the result to the left operand are only
//! implemented when it is safe to do so.
//!
//! ```
//! use typed_floats::*;
//!
//! let mut a: StrictlyPositive = core::f64::MAX.try_into().unwrap();
//! let b: StrictlyPositive = core::f64::MAX.try_into().unwrap();
//!
//! a += b;// Would not compile with StrictlyPositiveFinite
//!
//! assert_eq!(a.get(), core::f64::INFINITY);
//! ```
//!
//!
//! ```
//! use typed_floats::*;
//! use core::num::NonZeroU64;
//!
//! let a = NonZeroU64::new(1).unwrap();
//! let b: StrictlyPositive = a.into(); // no need for try_into
//!
//! assert_eq!(b.get(), 1.0);
//! ```
//!
//!
//!
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::indexing_slicing)]

pub trait Hypot<T> {
    type Output;

    fn hypot(self, rhs: T) -> Self::Output;
}

pub trait Min<T> {
    type Output;

    fn min(self, rhs: T) -> Self::Output;
}

pub trait Max<T> {
    type Output;

    fn max(self, rhs: T) -> Self::Output;
}

typed_floats_macros::generate_floats!();

pub trait Float:
    Eq
    + Copy
    + Ord
    + core::fmt::Debug
    + core::str::FromStr
    + TryFrom<Self::Content, Error = InvalidNumber>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<NonZeroU8>
    + TryFrom<NonZeroU16>
    + TryFrom<NonZeroU32>
    + TryFrom<NonZeroU64>
    + TryFrom<NonZeroI8>
    + TryFrom<NonZeroI16>
    + TryFrom<NonZeroI32>
    + TryFrom<NonZeroI64>
    + std::ops::Add
    + std::ops::Sub
    + std::ops::Mul
    + std::ops::Div
    + std::ops::Rem
{
    type Content: Sized
        + Clone
        + Copy
        + PartialOrd
        + PartialEq
        + core::fmt::Debug
        + core::fmt::Display
        + std::ops::Add<Output = Self::Content>
        + std::ops::Sub<Output = Self::Content>
        + std::ops::Mul<Output = Self::Content>
        + std::ops::Div<Output = Self::Content>
        + std::ops::Rem<Output = Self::Content>
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign;

    /// # Errors
    /// Returns an error if the value is not valid
    fn new(value: Self::Content) -> Result<Self, InvalidNumber>;

    /// # Safety
    /// The caller must ensure that the value is valid
    /// It will panic in debug mode if the value is not valid
    /// but in release mode the behavior is undefined
    unsafe fn new_unchecked(value: Self::Content) -> Self;

    fn get(&self) -> Self::Content;

    fn is_nan(&self) -> bool;
    fn is_infinite(&self) -> bool;
    fn is_finite(&self) -> bool;
    fn is_subnormal(&self) -> bool;
    fn is_normal(&self) -> bool;
    fn classify(&self) -> core::num::FpCategory;
    fn is_sign_positive(&self) -> bool;
    fn is_sign_negative(&self) -> bool;
}

pub enum FromStrError {
    ParseFloatError(core::num::ParseFloatError),
    InvalidNumber(InvalidNumber),
}

use core::num::{NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8};
use core::num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidNumber {
    #[error("Number is NaN")]
    NaN,
    #[error("Number is zero")]
    Zero,
    #[error("Number is negative")]
    Negative,
    #[error("Number is positive")]
    Positive,
    #[error("Number is infinite")]
    Infinite,
}

pub mod f64 {
    pub const INFINITY: crate::StrictlyPositive = crate::StrictlyPositive(core::f64::INFINITY);
    pub const NEG_INFINITY: crate::StrictlyNegative =
        crate::StrictlyNegative(core::f64::NEG_INFINITY);
    pub const MAX: crate::StrictlyPositiveFinite = crate::StrictlyPositiveFinite(core::f64::MAX);
    pub const MIN: crate::StrictlyNegativeFinite = crate::StrictlyNegativeFinite(core::f64::MIN);
    pub const MIN_POSITIVE: crate::StrictlyPositiveFinite =
        crate::StrictlyPositiveFinite(core::f64::MIN_POSITIVE);
    pub const ZERO: crate::PositiveFinite = crate::PositiveFinite(0.0f64);
    pub const NEGATIVE_ZERO: crate::NegativeFinite = crate::NegativeFinite(-0.0f64);

    pub mod consts {
        pub const PI: crate::PositiveFinite = crate::PositiveFinite(core::f64::consts::PI);
    }
}

pub mod f32 {
    pub const INFINITY: crate::StrictlyPositive<f32> =
        crate::StrictlyPositive::<f32>(core::f32::INFINITY);
    pub const NEG_INFINITY: crate::StrictlyNegative<f32> =
        crate::StrictlyNegative::<f32>(core::f32::NEG_INFINITY);
    pub const MAX: crate::StrictlyPositiveFinite<f32> =
        crate::StrictlyPositiveFinite::<f32>(core::f32::MAX);
    pub const MIN: crate::StrictlyNegativeFinite<f32> =
        crate::StrictlyNegativeFinite::<f32>(core::f32::MIN);
    pub const MIN_POSITIVE: crate::StrictlyPositiveFinite<f32> =
        crate::StrictlyPositiveFinite::<f32>(core::f32::MIN_POSITIVE);
    pub const ZERO: crate::PositiveFinite<f32> = crate::PositiveFinite::<f32>(0.0f32);
    pub const NEGATIVE_ZERO: crate::NegativeFinite<f32> = crate::NegativeFinite::<f32>(-0.0f32);

    pub mod consts {

        pub const PI: crate::PositiveFinite<f32> =
            crate::PositiveFinite::<f32>(core::f32::consts::PI);
    }
}
