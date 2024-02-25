use quote::quote;

use crate::types::{FloatSpecifications, Op, OpBuilder, ReturnTypeSpecification};

pub fn get_impl_self() -> Vec<Op> {
    vec![
        OpBuilder::new("neg")
            .trait_name("core::ops::Neg")
            .display("-")
            .op_fn(Box::new(|_| {
                quote! { -self.get() }
            }))
            .op_test(Box::new(|var| {
                quote! { -#var }
            }))
            .result2(Box::new(float_fn_types::core::ops::neg))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("abs")
            .description(quote! {
                /// Computes the absolute value of `self`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.0.try_into().unwrap();
                /// let b: NonNaN = (-4.0).try_into().unwrap();
                ///
                /// assert_eq!(a.abs(), 3.0);
                /// assert_eq!(b.abs(), 4.0);
                ///
                /// assert!(tf64::NEG_ZERO.abs().is_sign_positive());
                ///
                /// assert_eq!(tf64::NEG_INFINITY.abs(), tf64::INFINITY);
                /// ```
                ///
                /// See [`f64::abs()`] for more details.
            })
            .op_fn(Box::new(|float| {
                if !float.s.accept_negative {
                    // no-op
                    quote! { self.get() }
                } else if !float.s.accept_positive {
                    // inv
                    quote! { -self.get() }
                } else {
                    quote! { self.get().abs() }
                }
            }))
            .result2(Box::new(float_fn_types::core::ops::abs))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("ceil")
            .description(quote! {
                /// Returns the smallest integer greater than or equal to `self`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.ceil(), 4.0);
                /// assert_eq!(b.ceil(), -3.0);
                ///
                /// assert_eq!(tf64::INFINITY.ceil(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.ceil(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::ceil()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::ceil))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("floor")
            .description(quote! {
                /// Returns the largest integer less than or equal to `self`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.floor(), 3.0);
                /// assert_eq!(b.floor(), -4.0);
                ///
                /// assert_eq!(tf64::INFINITY.floor(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.floor(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::floor()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::floor))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("round")
            .description(quote! {
                /// Returns the nearest integer to `self`. If a value is half-way between two
                /// integers, round away from `0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.round(), 4.0);
                /// assert_eq!(b.round(), -4.0);
                ///
                /// assert_eq!(tf64::INFINITY.round(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.round(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::round()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::round))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("trunc")
            .description(quote! {
                /// Returns the integer part of `self`.
                /// This means that non-integer numbers are always truncated towards zero.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.trunc(), 3.0);
                /// assert_eq!(b.trunc(), -3.0);
                ///
                /// assert_eq!(tf64::INFINITY.trunc(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.trunc(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::trunc()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::trunc))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("fract")
            .description(quote! {
                /// Returns the fractional part of `self`.
                /// For negative numbers, the result is negative except when the fractional part is zero.
                /// For INIFINITY and NEG_INFINITY, the result is NaN.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaNFinite = 3.5.try_into().unwrap();
                /// let b: NonNaNFinite = (-3.5).try_into().unwrap();
                /// let c: NonNaNFinite = (-3.0).try_into().unwrap();
                ///
                /// assert_eq!(a.fract(), 0.5);
                /// assert_eq!(b.fract(), -0.5);
                /// assert_is_positive_zero!(c.fract());
                ///
                /// assert_is_nan!(tf64::INFINITY.fract());
                /// ```
                ///
                /// See [`f64::fract()`] for more details.
            })
            .comment(
                "`fract` returns `+0.0` if the factional part is zero, even for negative numbers.",
            )
            .result2(Box::new(float_fn_types::core::ops::fract))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("signum")
            .description(quote! {
                /// Returns a number that represents the sign of `self`.
                ///
                /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
                /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 3.5.try_into().unwrap();
                /// let b: NonNaN = (-3.5).try_into().unwrap();
                ///
                /// assert_eq!(a.signum(), 1.0);
                /// assert_eq!(b.signum(), -1.0);
                ///
                /// assert_eq!(tf64::ZERO.signum(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.signum(), -1.0);
                ///
                /// assert_eq!(tf64::INFINITY.signum(), 1.0);
                /// assert_eq!(tf64::NEG_INFINITY.signum(), -1.0);
                /// ```
                ///
                /// See [`f64::signum()`] for more details.
            })
            .op_fn(Box::new(|float| {
                if !float.s.accept_negative {
                    quote! { 1.0 }
                } else if !float.s.accept_positive {
                    quote! { -1.0 }
                } else {
                    quote! { self.get().signum() }
                }
            }))
            .result2(Box::new(float_fn_types::core::ops::signum))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("sqrt")
            .description(quote! {
                /// Returns the square root of a number.
                ///
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                /// Returns `-0.0` if `self` is `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 25.0.try_into().unwrap();
                /// let b: NonNaN = (-1).try_into().unwrap();
                ///
                /// assert_eq!(a.sqrt(), 5.0);
                /// assert_is_nan!(b.sqrt());
                ///
                /// assert!(tf64::is_positive_zero(tf64::ZERO.sqrt().into()));
                /// assert!(tf64::is_negative_zero(tf64::NEG_ZERO.sqrt()));
                ///
                /// assert_eq!(tf64::INFINITY.sqrt(), tf64::INFINITY);
                /// assert_is_nan!(tf64::NEG_INFINITY.sqrt());
                /// ```
                ///
                /// See [`f64::sqrt()`] for more details.
            })
            .comment("`sqrt(-0.0) = -0.0`")
            .op_fn(Box::new(|float| {
                // sqrt(-0.0) = -0.0
                if !float.s.accept_positive && !float.s.accept_zero {
                    let float_type = float.float_type_ident();

                    quote! { core::#float_type::NAN }
                } else {
                    quote! { self.get().sqrt() }
                }
            }))
            .result2(Box::new(float_fn_types::core::ops::sqrt))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("exp")
            .description(quote! {
                /// Returns `e^(self)`, (the exponential function).
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.exp(), core::f64::consts::E);
                /// assert_eq!(b.exp(), 1.0 / core::f64::consts::E);
                ///
                /// assert_eq!(tf64::ZERO.exp(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.exp(), 1.0);
                ///
                /// assert_eq!(tf64::INFINITY.exp(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.exp(), tf64::ZERO);
                /// ```
                ///
                /// See [`f64::exp()`] for more details.
            })
            .result(Box::new(|float| {
                ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                    accept_negative: false,
                    accept_positive: true,
                    accept_zero: float.s.accept_negative,
                    accept_inf: float.s.accept_positive,
                })
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("exp2")
            .description(quote! {
                /// Returns `2^(self)`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 2.0.try_into().unwrap();
                /// let b: NonNaN = (-2.0).try_into().unwrap();
                ///
                /// assert_eq!(a.exp2(), 4.0);
                /// assert_eq!(b.exp2(), 0.25);
                ///
                /// assert_eq!(tf64::ZERO.exp2(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.exp2(), 1.0);
                ///
                /// assert_eq!(tf64::INFINITY.exp2(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.exp2(), tf64::ZERO);
                /// ```
                ///
                /// See [`f64::exp2()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::exp2))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("ln")
            .description(quote! {
                /// Returns the natural logarithm of the number.
                ///
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.ln(), 0.0);
                /// assert_is_nan!(b.ln());
                ///
                /// assert_eq!(tf64::ZERO.ln(), f64::NEG_INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.ln(), f64::NEG_INFINITY);
                ///
                /// assert_eq!(tf64::INFINITY.ln(), tf64::INFINITY);
                /// assert_is_nan!(tf64::NEG_INFINITY.ln());
                /// ```
                ///
                /// See [`f64::ln()`] for more details.
            })
            .op_fn(Box::new(|float| {
                let is_strictly_negative = !float.s.accept_positive && !float.s.accept_zero;

                if is_strictly_negative {
                    let float_type = float.float_type_ident();

                    quote! { core::#float_type::NAN }
                } else {
                    quote! { self.get().ln() }
                }
            }))
            .result2(Box::new(float_fn_types::core::ops::ln))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("log2")
            .description(quote! {
                /// Returns the base 2 logarithm of the number.
                ///
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 2.0.try_into().unwrap();
                /// let b: NonNaN = 1.0.try_into().unwrap();
                /// let c: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.log2(), 1.0);
                /// assert!(tf64::is_positive_zero(b.log2()));
                /// assert_is_nan!(c.log2());
                ///
                /// assert_eq!(tf64::ZERO.log2(), f64::NEG_INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.log2(), f64::NEG_INFINITY);
                ///
                /// assert_eq!(tf64::INFINITY.log2(), tf64::INFINITY);
                /// assert_is_nan!(tf64::NEG_INFINITY.log2());
                /// ```
                ///
                /// See [`f64::log2()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::log2))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("log10")
            .description(quote! {
                /// Returns the base 10 logarithm of the number.
                ///
                /// Returns NaN if `self` is a negative number other than `-0.0`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 100.0.try_into().unwrap();
                /// let b: NonNaN = 1.0.try_into().unwrap();
                /// let c: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.log10(), 2.0);
                /// assert_eq!(b.log10(), 0.0);
                /// assert_is_nan!(c.log10());
                ///
                /// assert_eq!(tf64::ZERO.log10(), f64::NEG_INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.log10(), f64::NEG_INFINITY);
                ///
                /// assert_eq!(tf64::INFINITY.log10(), tf64::INFINITY);
                /// assert_is_nan!(tf64::NEG_INFINITY.log10());
                /// ```
                ///
                /// See [`f64::log10()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::log10))
            .build(),
        OpBuilder::new("to_degrees")
            .description(quote! {
                /// Converts degrees to radians.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = core::f64::consts::PI.try_into().unwrap();
                /// let b: NonNaN = (-core::f64::consts::PI).try_into().unwrap();
                /// let c: NonNaN = (2.0 * core::f64::consts::PI).try_into().unwrap();
                ///
                /// assert_eq!(a.to_degrees(), 180.0);
                /// assert_eq!(b.to_degrees(), -180.0);
                /// assert_eq!(c.to_degrees(), 360.0);
                ///
                /// assert_is_positive_zero!(tf64::ZERO.to_degrees());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.to_degrees());
                ///
                /// assert_eq!(tf64::INFINITY.to_degrees(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.to_degrees(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::to_degrees()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::to_degrees))
            .build(),
        OpBuilder::new("to_radians")
            .description(quote! {
                /// Converts degrees to radians.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 180.0.try_into().unwrap();
                /// let b: NonNaN = 0.0.try_into().unwrap();
                /// let c: NonNaN = (-180.0).try_into().unwrap();
                /// let d: NonNaN = 360.0.try_into().unwrap();
                ///
                /// assert_eq!(a.to_radians(), core::f64::consts::PI);
                /// assert_eq!(b.to_radians(), 0.0);
                /// assert_eq!(c.to_radians(), -core::f64::consts::PI);
                /// assert_eq!(d.to_radians(), 2.0 * core::f64::consts::PI);
                ///
                /// assert_eq!(tf64::INFINITY.to_radians(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.to_radians(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::to_radians()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::to_radians))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("cbrt")
            .description(quote! {
                /// Returns the cube root of a number.
                ///
                /// The result will be finite unless the argument is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 8.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.cbrt(), 2.0);
                /// assert_eq!(b.cbrt(), -1.0);
                ///
                /// assert_eq!(tf64::ZERO.cbrt(), tf64::ZERO);
                /// assert_eq!(tf64::NEG_ZERO.cbrt(), tf64::NEG_ZERO);
                ///
                /// assert_eq!(tf64::INFINITY.cbrt(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.cbrt(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::cbrt()`] for more details.
            })
            .result(Box::new(|float| {
                ReturnTypeSpecification::FloatSpecifications(float.s.clone())
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("sin")
            // For Non-zero, `sin` still produces zeros but the tests can't check it
            .skip_check_return_type_strictness()
            .description(quote! {
                /// Computes the sine of a number (in radians).
                ///
                /// The result will be in the range [-1, 1] if the input is finite,
                /// and NaN if the input is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaNFinite = core::f64::consts::PI.try_into().unwrap();
                /// let b: NonNaNFinite = 0.0.try_into().unwrap();
                /// let c: NonNaNFinite = (-0.0).try_into().unwrap();
                /// let d: NonNaNFinite = (-core::f64::consts::PI).try_into().unwrap();
                /// let e: NonNaNFinite = core::f64::consts::FRAC_PI_2.try_into().unwrap();
                /// let f: NonNaNFinite = (-core::f64::consts::FRAC_PI_2).try_into().unwrap();
                ///
                /// assert_relative_eq!(a.sin().get(), 0.0);
                /// assert_relative_eq!(b.sin().get(), 0.0);
                /// assert_relative_eq!(c.sin().get(), 0.0);
                /// assert_relative_eq!(d.sin().get(), 0.0);
                /// assert_relative_eq!(e.sin().get(), 1.0);
                /// assert_relative_eq!(f.sin().get(), -1.0);
                ///
                /// assert_is_nan!(tf64::INFINITY.sin());
                /// assert_is_nan!(tf64::NEG_INFINITY.sin());
                ///
                /// ```
                ///
                /// See [`f64::sin()`] for more details.
            })
            .result(Box::new(|float| {
                if float.s.accept_inf {
                    return ReturnTypeSpecification::NativeFloat;
                }

                ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                    accept_negative: true,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: false,
                })
            }))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("cos")
            // For Non-zero, `cos` still produces zeros but the tests can't check it
            .skip_check_return_type_strictness()
            .description(quote! {
                /// Computes the cosine of a number (in radians).
                ///
                /// The result will be in the range [-1, 1] if the input is finite,
                /// and NaN if the input is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaNFinite = core::f64::consts::PI.try_into().unwrap();
                /// let b: NonNaNFinite = 0.0.try_into().unwrap();
                /// let c: NonNaNFinite = (-0.0).try_into().unwrap();
                /// let d: NonNaNFinite = (-core::f64::consts::PI).try_into().unwrap();
                /// let e: NonNaNFinite = core::f64::consts::FRAC_PI_2.try_into().unwrap();
                /// let f: NonNaNFinite = (-core::f64::consts::FRAC_PI_2).try_into().unwrap();
                ///
                /// assert_relative_eq!(a.cos().get(), -1.0);
                /// assert_relative_eq!(b.cos().get(), 1.0);
                /// assert_relative_eq!(c.cos().get(), 1.0);
                /// assert_relative_eq!(d.cos().get(), -1.0);
                /// assert_relative_eq!(e.cos().get(), 0.0);
                /// assert_relative_eq!(f.cos().get(), 0.0);
                ///
                /// assert_is_nan!(tf64::INFINITY.cos());
                /// assert_is_nan!(tf64::NEG_INFINITY.cos());
                /// ```
                ///
                /// See [`f64::cos()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::cos))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("tan")
            // For Non-zero, `tan` still produces zeros but the tests can't check it
            .skip_check_return_type_strictness()
            .description(quote! {
                /// Computes the tangent of a number (in radians).
                ///
                /// The result NaN if the input is infinite.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_eq!(tf64::ZERO.tan(), 0.0);
                /// assert_relative_eq!(tf64::consts::FRAC_PI_4.tan().get(), 1.0);
                ///
                /// assert_is_nan!(tf64::INFINITY.tan());
                /// assert_is_nan!(tf64::NEG_INFINITY.tan());
                /// ```
                ///
                /// See [`f64::tan()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::tan))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("asin")
            .description(quote! {
                /// Computes the arcsine of a number.
                ///
                /// Return value is in radians in the range [-pi/2, pi/2] or NaN if the number is outside the range [-1, 1].
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 2.0.try_into().unwrap();
                /// let b: NonNaN = (-2.0).try_into().unwrap();
                ///
                /// assert_is_nan!(a.asin());
                /// assert_is_nan!(b.asin());
                ///
                /// assert!(tf64::is_positive_zero(tf64::ZERO.asin()));
                /// assert!(tf64::is_negative_zero(tf64::NEG_ZERO.asin()));
                ///
                /// assert_is_nan!(tf64::INFINITY.asin());
                /// assert_is_nan!(tf64::NEG_INFINITY.asin());
                /// ```
                ///
                /// See [`f64::asin()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::asin))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("acos")
            .description(quote! {
                /// Computes the arccosine of a number.
                ///
                /// Return value is in radians in the range [0, pi] or NaN if the number is outside the range [-1, 1].
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 2.0.try_into().unwrap();
                /// let b: NonNaN = (-2.0).try_into().unwrap();
                /// let c: NonNaN = (-1).try_into().unwrap();
                /// let d: NonNaN = 1.try_into().unwrap();
                ///
                /// assert_is_nan!(a.acos());
                /// assert_is_nan!(b.acos());
                /// assert_relative_eq!(c.acos(), core::f64::consts::PI);
                /// assert_relative_eq!(d.acos(), 0.0);
                ///
                /// assert_relative_eq!(tf64::ZERO.acos(), core::f64::consts::FRAC_PI_2);
                ///
                /// assert_is_nan!(tf64::INFINITY.acos());
                /// assert_is_nan!(tf64::NEG_INFINITY.acos());
                /// ```
                ///
                /// See [`f64::acos()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::acos))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("atan")
            .description(quote! {
                /// Computes the arctangent of a number.
                ///
                /// Return value is in radians in the range [-pi/2, pi/2];
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_is_positive_zero!(tf64::ZERO.atan());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.atan());
                ///
                /// assert_relative_eq!(tf64::INFINITY.atan().get(), core::f64::consts::FRAC_PI_2);
                /// assert_relative_eq!(tf64::NEG_INFINITY.atan().get(), -core::f64::consts::FRAC_PI_2);
                /// ```
                ///
                /// See [`f64::atan()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::atan))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("exp_m1")
            .description(quote! {
                /// Returns `e^(self) - 1` in a way that is accurate even if the number is close to zero.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_is_positive_zero!(tf64::ZERO.exp_m1());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.exp_m1());
                ///
                /// assert_eq!(tf64::INFINITY.exp_m1(), core::f64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.exp_m1(), -1.0);
                /// ```
                ///
                /// See [`f64::exp_m1()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::exp_m1))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("ln_1p")
            .description(quote! {
                /// Returns `ln(1+n)` (natural logarithm) more accurately than if the operations were performed separately.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.ln_1p(), core::f64::NEG_INFINITY);
                ///
                /// assert!(tf64::is_positive_zero(tf64::ZERO.ln_1p().get()));
                /// assert!(tf64::is_negative_zero(tf64::NEG_ZERO.ln_1p()));
                ///
                /// assert_eq!(tf64::INFINITY.ln_1p(), core::f64::INFINITY);
                /// assert_is_nan!(tf64::NEG_INFINITY.ln_1p());
                /// ```
                ///
                /// See [`f64::ln_1p()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::ln_1p))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("sinh")
            .description(quote! {
                /// Hyperbolic sine function.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_is_positive_zero!(tf64::ZERO.sinh());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.sinh());
                ///
                /// assert_eq!(tf64::INFINITY.sinh(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.sinh(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::sinh()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::sinh))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("cosh")
            .description(quote! {
                /// Hyperbolic cosine function.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_eq!(tf64::ZERO.cosh(), 1.0);
                /// assert_eq!(tf64::NEG_ZERO.cosh(), 1.0);
                ///
                /// assert_eq!(tf64::INFINITY.cosh(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.cosh(), tf64::INFINITY);
                /// ```
                ///
                /// See [`f64::cosh()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::cosh))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("tanh")
            .description(quote! {
                /// Hyperbolic tangent function.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_is_positive_zero!(tf64::ZERO.tanh());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.tanh());
                ///
                /// assert_eq!(tf64::INFINITY.tanh(), 1.0);
                /// assert_eq!(tf64::NEG_INFINITY.tanh(), -1.0);
                /// ```
                ///
                /// See [`f64::tanh()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::tanh))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("asinh")
            .description(quote! {
                /// Inverse hyperbolic sine function.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// assert_is_positive_zero!(tf64::ZERO.asinh());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.asinh());
                ///
                /// assert_eq!(tf64::INFINITY.asinh(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_INFINITY.asinh(), tf64::NEG_INFINITY);
                /// ```
                ///
                /// See [`f64::asinh()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::asinh))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("acosh")
            .description(quote! {
                /// Inverse hyperbolic cosine function.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                ///
                /// assert_is_positive_zero!(a.acosh());
                ///
                /// assert_is_nan!(tf64::ZERO.acosh());
                ///
                /// assert_eq!(tf64::INFINITY.acosh(), tf64::INFINITY);
                /// assert_is_nan!(tf64::NEG_INFINITY.acosh());
                /// ```
                ///
                /// See [`f64::acosh()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::acosh))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("atanh")
            .description(quote! {
                /// Inverse hyperbolic tangent function.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.atanh(), tf64::INFINITY);
                /// assert_eq!(b.atanh(), tf64::NEG_INFINITY);
                ///
                /// assert_is_positive_zero!(tf64::ZERO.atanh());
                /// assert_is_negative_zero!(tf64::NEG_ZERO.atanh());
                ///
                /// assert_is_nan!(tf64::INFINITY.atanh());
                /// assert_is_nan!(tf64::NEG_INFINITY.atanh());
                /// ```
                ///
                /// See [`f64::atanh()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::atanh))
            .build(),
        OpBuilder::new("recip")
            .description(quote! {
                /// Takes the reciprocal (inverse) of a number, `1/x`.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let a: NonNaN = 1.0.try_into().unwrap();
                /// let b: NonNaN = (-1.0).try_into().unwrap();
                ///
                /// assert_eq!(a.recip(), 1.0);
                /// assert_eq!(b.recip(), -1.0);
                ///
                /// assert_eq!(tf64::ZERO.recip(), tf64::INFINITY);
                /// assert_eq!(tf64::NEG_ZERO.recip(), tf64::NEG_INFINITY);
                ///
                /// assert_is_positive_zero!(tf64::INFINITY.recip());
                /// assert_is_negative_zero!(tf64::NEG_INFINITY.recip());
                /// ```
                ///
                /// See [`f64::recip()`] for more details.
            })
            .result2(Box::new(float_fn_types::core::ops::recip))
            .build(),
        #[cfg(any(feature = "std", feature = "libm"))]
        OpBuilder::new("powi")
            .params(quote! {self, n: i32})
            .op_fn(Box::new(|_| {
                let n = syn::Ident::new("n", proc_macro2::Span::call_site());

                quote! { self.get().powi(#n) }
            }))
            .description(quote! {
                /// Raises a number to an integer power.
                ///
                /// Using this function is generally faster than using `powf`.
                /// It might have a different sequence of rounding operations than `powf`,
                /// so the results are not guaranteed to agree.
                ///
                /// # Examples
                ///
                /// ```
                /// # use typed_floats::*;
                /// let x: NonNaN = (-2.0).try_into().unwrap();
                ///
                /// assert_eq!(x.powi(3), -8.0);
                /// assert_eq!(x.powi(2), 4.0);
                /// assert_eq!(x.powi(1), -2.0);
                /// assert_eq!(x.powi(0), 1.0);
                /// assert_eq!(x.powi(-1), -0.5);
                /// assert_eq!(x.powi(-2), 0.25);
                /// assert_eq!(x.powi(-3), -0.125);
                /// ```
                /// See [`f64::powi()`] for more details.
            })
            .result(Box::new(|float| {
                ReturnTypeSpecification::FloatSpecifications(FloatSpecifications {
                    accept_negative: float.s.accept_negative,
                    accept_positive: true,
                    accept_zero: true,
                    accept_inf: true,
                })
            }))
            .skip_check_return_type_strictness()
            .op_test(Box::new(|var| {
                quote! { #var.powi(2) }
            }))
            .build(),
    ]
}
