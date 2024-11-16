#![doc = include_str!("../README.truncated.md")]
//! # Rules
//!
//! Conversions rules for operations are summarized in [`conversions_rules`].
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
//! assert_eq!(c, 2.0);
//! assert_eq!(d, 0.0);   
//! ```
//!
//! ```
//! use typed_floats::*;
//!
//! let a: StrictlyPositiveFinite = 1.0f64.try_into().unwrap();
//! let b: Positive = 0.0f64.try_into().unwrap();
//!
//! let c: StrictlyPositive = a + b;
//!
//! assert_eq!(c, 1.0);   
//! ```
//!
//! Operations that assign the result to the left operand are only
//! implemented when it is safe to do so:
//!
//! ```
//! use typed_floats::*;
//!
//! let mut a: StrictlyPositive = f64::MAX.try_into().unwrap();
//! let b: StrictlyPositive = f64::MAX.try_into().unwrap();
//!
//! a += b;// Would not compile with StrictlyPositiveFinite
//!
//! assert_eq!(a, f64::INFINITY);
//! ```
//!
//! ```compile_fail
//! use typed_floats::*;
//!
//! let mut a: StrictlyPositiveFinite = f64::MAX.try_into().unwrap();
//! let b: StrictlyPositiveFinite = f64::MAX.try_into().unwrap();
//!
//! a += b;// Does not compile
//!
//! assert_eq!(a, f64::INFINITY);
//! ```
//!
//! Conversions from non-zero integers are available:
//!
//! ```
//! use typed_floats::*;
//! use core::num::NonZeroU64;
//!
//! let a = NonZeroU64::new(1).unwrap();
//! let b: StrictlyPositive = a.into(); // no need for try_into
//!
//! assert_eq!(b, 1.0);
//! ```
//!
//! Also, comparaison between types is available:
//!
//! ```
//! use typed_floats::*;
//!
//! let a: f64 = 1.0;
//! let b: StrictlyPositive = 1.0.try_into().unwrap();
//! let c: StrictlyPositiveFinite = 1.0.try_into().unwrap();
//!
//! assert_eq!(a, b);
//! assert_eq!(b, a);
//! assert_eq!(b, c);
//! ```
//!
//! To return early in a function:
//! ```
//! use typed_floats::*;
//!
//! fn early_return(a:f64,b:f64) -> Result<PositiveFinite,InvalidNumber> {
//!   let a: StrictlyPositiveFinite = a.try_into()?;
//!   let b: StrictlyPositiveFinite = b.try_into()?;
//!
//!   Ok(a % b)
//! }
//!
//! assert_eq!(early_return(-1.0,2.0), Err(InvalidNumber::Negative));
//! assert_eq!(early_return(1.0,2.0).unwrap().get(), 1.0);
//! ```
//!
//!
//!
#![warn(clippy::indexing_slicing)]
#![warn(clippy::nursery)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_crate_dependencies)]
#![cfg_attr(not(feature = "std"), no_std)]

// `format!` is used during the tests even in `no_std` environments
#[cfg(all(test, not(feature = "std")))]
#[macro_use]
extern crate alloc;

mod macros;
mod traits;
mod types;

#[cfg(feature = "serde")]
mod serde;

pub use traits::*;
pub use types::*;

typed_floats_macros::generate_docs!(
    pub mod conversions_rules {}
);

/// This module contains constants from [`core::f64`], casted to the corresponding type
pub mod tf64 {
    /// Equivalent to `NonNaN<f64>`
    pub type NonNaN = crate::NonNaN<f64>;

    /// Equivalent to `NonNaNFinite<f64>`
    pub type NonNaNFinite = crate::NonNaNFinite<f64>;

    /// Equivalent to `NonZeroNonNaN<f64>`
    pub type NonZeroNonNaN = crate::NonZeroNonNaN<f64>;

    /// Equivalent to `NonZeroNonNaNFinite<f64>`
    pub type NonZeroNonNaNFinite = crate::NonZeroNonNaNFinite<f64>;

    /// Equivalent to `StrictlyPositive<f64>`
    pub type StrictlyPositive = crate::StrictlyPositive<f64>;

    /// Equivalent to `StrictlyNegative<f64>`
    pub type StrictlyNegative = crate::StrictlyNegative<f64>;

    /// Equivalent to `Positive<f64>`
    pub type Positive = crate::Positive<f64>;

    /// Equivalent to `Negative<f64>`
    pub type Negative = crate::Negative<f64>;

    /// Equivalent to `StrictlyPositiveFinite<f64>`
    pub type StrictlyPositiveFinite = crate::StrictlyPositiveFinite<f64>;

    /// Equivalent to `StrictlyNegativeFinite<f64>`
    pub type StrictlyNegativeFinite = crate::StrictlyNegativeFinite<f64>;

    /// Equivalent to `PositiveFinite<f64>`
    pub type PositiveFinite = crate::PositiveFinite<f64>;

    /// Equivalent to `NegativeFinite<f64>`
    pub type NegativeFinite = crate::NegativeFinite<f64>;

    /// Returns `true` if the number is positive zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf64::is_positive_zero(3.0), false);
    /// assert_eq!(tf64::is_positive_zero(-0.0), false);
    /// assert_eq!(tf64::is_positive_zero(0.0), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_positive_zero(x: f64) -> bool {
        x == 0.0 && x.is_sign_positive()
    }

    /// Returns `true` if the number is negative zero.
    ///    
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf64::is_negative_zero(3.0), false);
    /// assert_eq!(tf64::is_negative_zero(-0.0), true);
    /// assert_eq!(tf64::is_negative_zero(0.0), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_negative_zero(x: f64) -> bool {
        x == 0.0 && x.is_sign_negative()
    }

    crate::generate_const!(
        INFINITY,
        StrictlyPositive,
        f64,
        f64::INFINITY,
        "Infinity (∞)."
    );

    crate::generate_const!(
        NEG_INFINITY,
        StrictlyNegative,
        f64,
        f64::NEG_INFINITY,
        "Negative infinity (−∞)."
    );

    crate::generate_const!(ZERO, PositiveFinite, f64, 0.0f64, "Positive zero (+0.0).");

    crate::generate_const!(
        NEG_ZERO,
        NegativeFinite,
        f64,
        -0.0f64,
        "Negative zero (-0.0)."
    );

    crate::generate_const!(
        MAX,
        StrictlyPositiveFinite,
        f64,
        f64::MAX,
        "Largest finite `f64` value."
    );

    crate::generate_const!(
        MIN,
        StrictlyNegativeFinite,
        f64,
        f64::MIN,
        "Smallest finite `f64` value."
    );

    crate::generate_const!(
        MIN_POSITIVE,
        StrictlyPositiveFinite,
        f64,
        f64::MIN_POSITIVE,
        "Smallest positive normal `f64` value."
    );

    /// This module contains constants from [`core::f64::consts`], casted to the corresponding type
    pub mod consts {
        crate::generate_const!(
            PI,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::PI,
            "Archimedes' constant (π)"
        );
        crate::generate_const!(
            TAU,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::TAU,
            "The full circle constant (τ). Equal to 2π."
        );
        crate::generate_const!(
            FRAC_PI_2,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_PI_2,
            "π/2"
        );
        crate::generate_const!(
            FRAC_PI_3,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_PI_3,
            "π/3"
        );
        crate::generate_const!(
            FRAC_PI_4,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_PI_4,
            "π/4"
        );
        crate::generate_const!(
            FRAC_PI_6,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_PI_6,
            "π/6"
        );
        crate::generate_const!(
            FRAC_PI_8,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_PI_8,
            "π/8"
        );
        crate::generate_const!(
            FRAC_1_PI,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_1_PI,
            "1/π"
        );
        crate::generate_const!(
            FRAC_2_PI,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_2_PI,
            "2/π"
        );
        crate::generate_const!(
            FRAC_2_SQRT_PI,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_2_SQRT_PI,
            "2/sqrt(π)"
        );
        crate::generate_const!(
            SQRT_2,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::SQRT_2,
            "sqrt(2)"
        );
        crate::generate_const!(
            FRAC_1_SQRT_2,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::FRAC_1_SQRT_2,
            "1/sqrt(2)"
        );
        crate::generate_const!(
            E,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::E,
            "Euler's number (e)"
        );
        crate::generate_const!(
            LOG2_10,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::LOG2_10,
            "log<sub>2</sub>(10)"
        );
        crate::generate_const!(
            LOG2_E,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::LOG2_E,
            "log<sub>2</sub>(e)"
        );
        crate::generate_const!(
            LOG10_2,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::LOG10_2,
            "log<sub>10</sub>(2)"
        );
        crate::generate_const!(
            LOG10_E,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::LOG10_E,
            "log<sub>10</sub>(e)"
        );
        crate::generate_const!(
            LN_2,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::LN_2,
            "ln(2)"
        );
        crate::generate_const!(
            LN_10,
            StrictlyPositiveFinite,
            f64,
            core::f64::consts::LN_10,
            "ln(10)"
        );
    }

    #[cfg(test)]
    pub(crate) const TEST_VALUES: [f64; 21] = typed_floats_macros::test_values!(f64);
}

/// This module contains constants from [`core::f32`], casted to the corresponding type
pub mod tf32 {
    /// Equivalent to `NonNaN<f32>`
    pub type NonNaN = crate::NonNaN<f32>;

    /// Equivalent to `NonNaNFinite<f32>`
    pub type NonNaNFinite = crate::NonNaNFinite<f32>;

    /// Equivalent to `NonZeroNonNaN<f32>`
    pub type NonZeroNonNaN = crate::NonZeroNonNaN<f32>;

    /// Equivalent to `NonZeroNonNaNFinite<f32>`
    pub type NonZeroNonNaNFinite = crate::NonZeroNonNaNFinite<f32>;

    /// Equivalent to `StrictlyPositive<f32>`
    pub type StrictlyPositive = crate::StrictlyPositive<f32>;

    /// Equivalent to `StrictlyNegative<f32>`
    pub type StrictlyNegative = crate::StrictlyNegative<f32>;

    /// Equivalent to `Positive<f32>`
    pub type Positive = crate::Positive<f32>;

    /// Equivalent to `Negative<f32>`
    pub type Negative = crate::Negative<f32>;

    /// Equivalent to `StrictlyPositiveFinite<f32>`
    pub type StrictlyPositiveFinite = crate::StrictlyPositiveFinite<f32>;

    /// Equivalent to `StrictlyNegativeFinite<f32>`
    pub type StrictlyNegativeFinite = crate::StrictlyNegativeFinite<f32>;

    /// Equivalent to `PositiveFinite<f32>`
    pub type PositiveFinite = crate::PositiveFinite<f32>;

    /// Equivalent to `NegativeFinite<f32>`
    pub type NegativeFinite = crate::NegativeFinite<f32>;

    /// Returns `true` if the number is positive zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf32::is_positive_zero(3.0), false);
    /// assert_eq!(tf32::is_positive_zero(-0.0), false);
    /// assert_eq!(tf32::is_positive_zero(0.0), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_positive_zero(x: f32) -> bool {
        x == 0.0 && x.is_sign_positive()
    }

    /// Returns `true` if the number is negative zero.
    ///    
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    ///
    /// assert_eq!(tf32::is_negative_zero(3.0), false);
    /// assert_eq!(tf32::is_negative_zero(-0.0), true);
    /// assert_eq!(tf32::is_negative_zero(0.0), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_negative_zero(x: f32) -> bool {
        x == 0.0 && x.is_sign_negative()
    }

    crate::generate_const!(
        INFINITY,
        StrictlyPositive,
        f32,
        f32::INFINITY,
        "Infinity (∞)."
    );

    crate::generate_const!(
        NEG_INFINITY,
        StrictlyNegative,
        f32,
        f32::NEG_INFINITY,
        "Negative infinity (−∞)."
    );

    crate::generate_const!(ZERO, PositiveFinite, f32, 0.0f32, "Positive zero (+0.0).");

    crate::generate_const!(
        NEG_ZERO,
        NegativeFinite,
        f32,
        -0.0f32,
        "Negative zero (-0.0)."
    );

    crate::generate_const!(
        MAX,
        StrictlyPositiveFinite,
        f32,
        f32::MAX,
        "Largest finite `f32` value."
    );

    crate::generate_const!(
        MIN,
        StrictlyNegativeFinite,
        f32,
        f32::MIN,
        "Smallest finite `f32` value."
    );

    crate::generate_const!(
        MIN_POSITIVE,
        StrictlyPositiveFinite,
        f32,
        f32::MIN_POSITIVE,
        "Smallest positive normal `f32` value."
    );

    /// This module contains constants from [`core::f32::consts`], casted to the corresponding type
    pub mod consts {
        crate::generate_const!(
            PI,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::PI,
            "Archimedes' constant (π)"
        );
        crate::generate_const!(
            TAU,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::TAU,
            "The full circle constant (τ). Equal to 2π."
        );
        crate::generate_const!(
            FRAC_PI_2,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_PI_2,
            "π/2"
        );
        crate::generate_const!(
            FRAC_PI_3,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_PI_3,
            "π/3"
        );
        crate::generate_const!(
            FRAC_PI_4,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_PI_4,
            "π/4"
        );
        crate::generate_const!(
            FRAC_PI_6,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_PI_6,
            "π/6"
        );
        crate::generate_const!(
            FRAC_PI_8,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_PI_8,
            "π/8"
        );
        crate::generate_const!(
            FRAC_1_PI,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_1_PI,
            "1/π"
        );
        crate::generate_const!(
            FRAC_2_PI,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_2_PI,
            "2/π"
        );
        crate::generate_const!(
            FRAC_2_SQRT_PI,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_2_SQRT_PI,
            "2/sqrt(π)"
        );
        crate::generate_const!(
            SQRT_2,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::SQRT_2,
            "sqrt(2)"
        );
        crate::generate_const!(
            FRAC_1_SQRT_2,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::FRAC_1_SQRT_2,
            "1/sqrt(2)"
        );
        crate::generate_const!(
            E,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::E,
            "Euler's number (e)"
        );
        crate::generate_const!(
            LOG2_10,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::LOG2_10,
            "log<sub>2</sub>(10)"
        );
        crate::generate_const!(
            LOG2_E,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::LOG2_E,
            "log<sub>2</sub>(e)"
        );
        crate::generate_const!(
            LOG10_2,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::LOG10_2,
            "log<sub>10</sub>(2)"
        );
        crate::generate_const!(
            LOG10_E,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::LOG10_E,
            "log<sub>10</sub>(e)"
        );
        crate::generate_const!(
            LN_2,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::LN_2,
            "ln(2)"
        );
        crate::generate_const!(
            LN_10,
            StrictlyPositiveFinite,
            f32,
            core::f32::consts::LN_10,
            "ln(10)"
        );
    }

    #[cfg(test)]
    pub(crate) const TEST_VALUES: [f32; 21] = typed_floats_macros::test_values!(f32);
}
