use crate::types::{f64, InvalidNumber, NonNaNFinite};

impl NonNaNFinite<f64> {
    /// Creates a new value from a primitive type
    /// It adds a little overhead compared to `new_unchecked`
    /// because it checks that the value is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::tf64::NonNaNFinite;
    /// let x = NonNaNFinite::new(3.0).unwrap();
    ///
    /// assert_eq!(x, 3.0);
    /// ```
    ///
    /// # Errors
    /// Returns an error if the value is not valid
    #[inline]
    pub const fn new(value: f64) -> Result<Self, InvalidNumber> {
        if value.is_nan() {
            return Err(InvalidNumber::NaN);
        }

        if value.is_infinite() {
            return Err(InvalidNumber::Infinite);
        }

        Ok(Self(value))
    }

    /// Creates a new value from a primitive type with zero overhead (in release mode).
    /// It is up to the caller to ensure that the value is valid
    ///
    /// # Examples
    ///
    /// ```
    /// # use typed_floats::tf64::NonNaNFinite;
    /// let x = unsafe { NonNaNFinite::new_unchecked(3.0) };
    ///
    /// assert_eq!(x, 3.0);
    /// ```
    /// # Safety
    /// The caller must ensure that the value is valid.
    /// It will panic in debug mode if the value is not valid,
    /// but in release mode the behavior is undefined
    #[inline]
    #[must_use]
    pub unsafe fn new_unchecked(value: f64) -> Self {
        if Self::new(value).is_err() {
            debug_assert!(false, "{value} is not a valid NonNaNFinite<f64>");

            #[cfg(feature = "ensure_no_undefined_behavior")]
            panic!("{value} is not a valid NonNaNFinite<f64>");

            #[cfg(all(
                feature = "compiler_hints",
                not(feature = "ensure_no_undefined_behavior")
            ))]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }

        Self(value)
    }

    /// Returns the value as a primitive type
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    ///
    /// let x = NonNaNFinite::new(3.0).unwrap();
    ///
    /// let y: f64 = x.into();
    ///
    /// assert_eq!(y, 3.0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> f64 {
        self.0
    }

    /// Returns `true` if this value is NaN.
    /// This is never the case for the provided types
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_nan(), false);
    /// ```
    ///
    /// See [`f64::is_nan()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_nan(&self) -> bool {
        false
    }

    /// Returns `true` if this value is positive infinity or negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_infinite(), false);
    /// ```
    ///
    /// See [`f64::is_infinite()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_infinite(&self) -> bool {
        false
    }

    /// Returns `true` if this number is positive infinity nor negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_finite(), true);
    /// ```
    ///
    /// See [`f64::is_finite()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_finite(&self) -> bool {
        true
    }

    /// Returns `true` if the number is [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_subnormal(), false);
    /// ```
    ///
    /// See [`f64::is_subnormal()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_subnormal(&self) -> bool {
        self.0.is_subnormal()
    }

    /// Returns `true` if the number is neither zero, infinite or [subnormal](https://en.wikipedia.org/wiki/Denormal_number).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_normal(), true);
    /// ```
    ///
    /// See [`f64::is_normal()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_normal(&self) -> bool {
        self.0.is_normal()
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.classify(), core::num::FpCategory::Normal);
    /// ```
    ///
    /// See [`f64::classify()`] for more details.
    #[inline]
    #[must_use]
    pub const fn classify(&self) -> core::num::FpCategory {
        self.0.classify()
    }

    /// Returns `true` if `self` has a positive sign, including `+0.0` and positive infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_sign_positive(), true);
    /// ```
    ///
    /// See [`f64::is_sign_positive()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    /// Returns `true` if `self` has a negative sign, including `-0.0` and negative infinity.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_sign_negative(), false);
    /// ```
    ///
    /// See [`f64::is_sign_negative()`] for more details.
    #[inline]
    #[must_use]
    pub const fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    /// Returns `true` if the number is negative zero.
    ///     
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    /// let y: NonNaNFinite = (-0.0).try_into().unwrap();
    /// let z: NonNaNFinite = 0.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_negative_zero(), false);
    /// assert_eq!(y.is_negative_zero(), true);
    /// assert_eq!(z.is_negative_zero(), false);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_negative_zero(&self) -> bool {
        self.0 == 0.0 && self.0.is_sign_negative()
    }

    /// Returns `true` if the number is positive zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_floats::tf64::NonNaNFinite;
    /// let x: NonNaNFinite = 3.0.try_into().unwrap();
    /// let y: NonNaNFinite = (-0.0).try_into().unwrap();
    /// let z: NonNaNFinite = 0.0.try_into().unwrap();
    ///
    /// assert_eq!(x.is_positive_zero(), false);
    /// assert_eq!(y.is_positive_zero(), false);
    /// assert_eq!(z.is_positive_zero(), true);
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_positive_zero(&self) -> bool {
        self.0 == 0.0 && self.0.is_sign_positive()
    }
}
