#[cfg(any(feature = "std", feature = "libm"))]
/// This trait is used to specify the return type of the [`Hypot::hypot()`] function.
pub trait Hypot<T> {
    /// The resulting type after applying [`Hypot::hypot()`].
    type Output;

    /// Compute the distance between the origin and a point (`x`, `y`) on the
    /// Euclidean plane. Equivalently, compute the length of the hypotenuse of a
    /// right-angle triangle with other sides having length `x.abs()` and
    /// `y.abs()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(x.hypot(y), 5.0);
    /// ```
    ///
    /// See [`f64::hypot()`] for more details.
    fn hypot(self, rhs: T) -> Self::Output;
}

/// This trait is used to specify the return type of the [`Min::min()`] function.
pub trait Min<T> {
    /// The resulting type after applying [`Min::min()`].
    type Output;

    /// Returns the minimum of the two numbers.
    ///
    /// This follows the IEEE 754-2008 semantics for minNum.
    /// This also matches the behavior of libm’s fmin.
    ///
    /// The min of `+0.0` and `-0.0` may return either operand.
    /// <https://llvm.org/docs/LangRef.html#llvm-minnum-intrinsic>
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(Min::min(x, y), 3.0);
    /// ```
    ///
    /// See [`f64::min()`] for more details.
    fn min(self, rhs: T) -> Self::Output;
}

/// This trait is used to specify the return type of the [`Max::max()`] function.
pub trait Max<T> {
    /// The resulting type after applying [`Max::max()`].
    type Output;

    /// Returns the maximum of the two numbers.
    ///
    /// This follows the IEEE 754-2008 semantics for maxNum;
    /// This also matches the behavior of libm’s fmax.
    ///
    /// The max of `+0.0` and `-0.0` may return either operand.
    /// <https://llvm.org/docs/LangRef.html#llvm-maxnum-intrinsic>
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.0.try_into().unwrap();
    /// let y: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(Max::max(x, y), 4.0);
    /// ```
    ///
    /// See [`f64::max()`] for more details.
    fn max(self, rhs: T) -> Self::Output;
}

#[cfg(any(feature = "std", feature = "libm"))]
/// This trait is used to specify the return type of the [`Copysign::copysign()`] function.
pub trait Copysign<T> {
    /// The resulting type after applying [`Copysign::copysign()`].
    type Output;

    /// Returns a number composed of the magnitude of self and the sign of sign.
    /// Equal to self if the sign of self and sign are the same, otherwise equal to -self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let x: NonNaN = 3.5.try_into().unwrap();
    /// let y: NonNaN = (-3.5).try_into().unwrap();
    /// let a: NonNaN = 1.0.try_into().unwrap();
    /// let b: NonNaN = 0.0.try_into().unwrap();
    /// let c: NonNaN = (-0.0).try_into().unwrap();
    /// let d: NonNaN = (-1.0).try_into().unwrap();
    ///
    /// assert_eq!(x.copysign(a), 3.5);
    /// assert_eq!(x.copysign(b), 3.5);
    /// assert_eq!(x.copysign(c), -3.5);
    /// assert_eq!(x.copysign(d), -3.5);
    ///
    /// assert_eq!(y.copysign(a), 3.5);
    /// assert_eq!(y.copysign(b), 3.5);
    /// assert_eq!(y.copysign(c), -3.5);
    /// assert_eq!(y.copysign(d), -3.5);
    /// ```
    ///
    /// See [`f64::copysign()`] for more details.
    fn copysign(self, rhs: T) -> Self::Output;
}

#[cfg(any(feature = "std", feature = "libm"))]
/// This trait is used to specify the return type of the [`DivEuclid::div_euclid()`] function.
pub trait DivEuclid<T> {
    /// The resulting type after applying [`DivEuclid::div_euclid()`].
    type Output;

    /// Calculates Euclidean division, the matching method for `rem_euclid`.
    /// Equal to self if the sign of self and sign are the same, otherwise equal to -self.
    ///
    /// This computes the integer `n` such that
    /// `self = n * rhs + self.rem_euclid(rhs)`.
    /// In other words, the result is `self / rhs` rounded to the integer `n`
    /// such that `self >= n * rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// let a: NonNaN = 7.0.try_into().unwrap();
    /// let b: NonNaN = 4.0.try_into().unwrap();
    ///
    /// assert_eq!(a.div_euclid(b), 1.0);
    /// assert_eq!((-a).div_euclid(b), -2.0);
    /// assert_eq!(a.div_euclid(-b), -1.0);
    /// assert_eq!((-a).div_euclid(-b), 2.0);
    /// ```
    ///
    /// See [`f64::div_euclid()`] for more details.
    fn div_euclid(self, rhs: T) -> Self::Output;
}

#[cfg(any(feature = "std", feature = "libm"))]
/// This trait is used to specify the return type of the [`Atan2::atan2()`] function.
pub trait Atan2<T> {
    /// The resulting type after applying [`Atan2::atan2()`].
    type Output;

    /// Computes the four quadrant arctangent of `self` (`y`) and `other` (`x`) in radians.
    ///
    /// * `x = 0`, `y = 0`: `0`
    /// * `x >= 0`: `arctan(y/x)` -> `[-pi/2, pi/2]`
    /// * `y >= 0`: `arctan(y/x) + pi` -> `(pi/2, pi]`
    /// * `y < 0`: `arctan(y/x) - pi` -> `(-pi, -pi/2)`
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::*;
    /// use typed_floats::tf64::NonNaN;
    /// use typed_floats::tf64::consts::FRAC_PI_4;
    ///
    /// // Positive angles measured counter-clockwise
    /// // from positive x axis
    /// // -pi/4 radians (45 deg clockwise)
    /// let x1: NonNaN = 3.0_f64.try_into().unwrap();
    /// let y1: NonNaN = (-3.0_f64).try_into().unwrap();
    ///
    /// // 3pi/4 radians (135 deg counter-clockwise)
    /// let x2 = -3.0_f64;
    /// let y2 = 3.0_f64;
    ///
    /// assert_relative_eq!(y1.atan2(x1), -core::f64::consts::FRAC_PI_4);
    /// assert_relative_eq!(y2.atan2(x2), 3.0 * core::f64::consts::FRAC_PI_4);
    /// ```
    ///
    /// See [`f64::atan2()`] for more details.
    fn atan2(self, rhs: T) -> Self::Output;
}

#[cfg(any(feature = "std", feature = "libm"))]
/// This trait is used to specify the return type of the [`Powf::powf()`] function.
pub trait Powf<T> {
    /// The resulting type after applying [`Powf::powf()`].
    type Output;

    /// See [`f64::powf()`] for more details.
    fn powf(self, rhs: T) -> Self::Output;
}

pub trait RestrictInf: Sized {
    type Output: Sized;

    fn restrict_inf(self) ->  Result<Self::Output, Self>;
}

pub trait RestrictZero: Sized {
    type Output: Sized;

    fn restrict_zero(self) ->  Result<Self::Output, Self>;
}

pub trait RestrictPositive: Sized {
    type Output: Sized;

    fn restrict_positive(self) ->  Result<Self::Output, Self>;
}

pub trait RestrictNegative: Sized {
    type Output: Sized;

    fn restrict_negative(self) ->  Result<Self::Output, Self>;
}