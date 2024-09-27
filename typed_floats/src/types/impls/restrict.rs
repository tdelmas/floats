use crate::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
    RestrictInf, RestrictZero, RestrictPositive, RestrictNegative,
};


macro_rules! impl_exclude {
    ($trait:ident, $fn:ident, $check:ident, $type:ident, $output:ident) => {
        impl $trait for $type<f32> {
            type Output = $output<f32>;

            /// Filters out infinite values.
            /// 
            /// ```
            /// use typed_floats::*;
            /// 
            /// let a: Positive<f32> = 0.0.try_into().unwrap();
            /// let b: PositiveFinite<f32> = a.restrict_inf().unwrap();
            /// let c: PositiveFinite<f32> = match a.restrict_inf() {
            ///    Ok(x) => x,
            ///    Err(x) => panic!("{} is infinite", x),
            /// };
            /// ```
            #[inline]
            fn $fn(self) -> Result<Self::Output, Self> {
                if self.$check() {
                    Err(self)
                } else {
                    Ok(unsafe { $output::<f32>::new_unchecked(self.get()) })
                }
            }
        }
        
        impl $trait for $type<f64> {
            type Output = $output<f64>;

            /// Filters out infinite values.
            /// 
            /// ```
            /// use typed_floats::*;
            /// 
            /// let a: Positive<f64> = 0.0.try_into().unwrap();
            /// let b: PositiveFinite<f64> = a.restrict_inf().unwrap();
            /// ```
            #[inline]
            fn $fn(self) -> Result<Self::Output, Self> {
                if self.$check() {
                    Err(self)
                } else {
                    Ok(unsafe { $output::<f64>::new_unchecked(self.get()) })
                }
            }
        }
    }
}

macro_rules! impl_exclude_inf {
    ($type:ident, $output:ident) => {
        impl_exclude!(RestrictInf, restrict_inf, is_infinite, $type, $output);
    }
}

macro_rules! impl_exclude_zero {
    ($type:ident, $output:ident) => {
        //impl_exclude!(RestrictZero, restrict_zero, is_zero, $type, $output);
    }
}

macro_rules! impl_exclude_positive {
    ($type:ident, $output:ident) => {
        impl_exclude!(RestrictPositive, restrict_positive, is_sign_positive, $type, $output);
    }
}

macro_rules! impl_exclude_negative {
    ($type:ident, $output:ident) => {
        impl_exclude!(RestrictNegative, restrict_negative, is_sign_positive, $type, $output);
    }
}

impl_exclude_inf!(NonNaN, NonNaNFinite);
impl_exclude_inf!(NonZeroNonNaN, NonZeroNonNaNFinite);
impl_exclude_inf!(StrictlyPositive, StrictlyPositiveFinite);
impl_exclude_inf!(StrictlyNegative, StrictlyNegativeFinite);
impl_exclude_inf!(Positive, PositiveFinite);
impl_exclude_inf!(Negative, NegativeFinite);

impl_exclude_zero!(NonNaN, NonZeroNonNaN);
impl_exclude_zero!(NonNaNFinite, NonZeroNonNaNFinite);
impl_exclude_zero!(PositiveFinite, StrictlyPositiveFinite);
impl_exclude_zero!(NegativeFinite, StrictlyNegativeFinite);
impl_exclude_zero!(Positive, StrictlyPositive);
impl_exclude_zero!(Negative, StrictlyNegative);

impl_exclude_positive!(NonNaN, Negative);
impl_exclude_positive!(NonNaNFinite, NegativeFinite);
impl_exclude_positive!(NonZeroNonNaN, StrictlyNegative);
impl_exclude_positive!(NonZeroNonNaNFinite, StrictlyNegativeFinite);

impl_exclude_negative!(NonNaN, Positive);
impl_exclude_negative!(NonNaNFinite, PositiveFinite);
impl_exclude_negative!(NonZeroNonNaN, StrictlyPositive);
impl_exclude_negative!(NonZeroNonNaNFinite, StrictlyPositiveFinite);
