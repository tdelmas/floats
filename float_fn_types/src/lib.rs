#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Range {
    #[default]
    Full,
    Negative,
    Positive,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Possible {
    #[default]
    Yes,
    // Theorically yes, but may not happen because of the rounding error
    Should,
    // Theorically no, byt may happen because of the rounding error
    ShouldNot,
    No,
}

impl From<Possible> for u8 {
    fn from(val: Possible) -> Self {
        match val {
            Possible::No => 0,
            Possible::ShouldNot => 1,
            Possible::Should => 2,
            Possible::Yes => 3,
        }
    }
}

impl std::cmp::Ord for Possible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (lhs, rhs): (u8, u8) = ((*self).into(), (*other).into());

        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Possible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Possible {
    /// If something is possible for two reasons,
    /// we return the strongest one :
    ///
    /// ```
    /// use float_fn_types::Possible;
    ///
    /// assert_eq!(Possible::Yes, Possible::any(Possible::Yes, Possible::Should));
    /// assert_eq!(Possible::Yes, Possible::any(Possible::Should, Possible::Yes));
    /// assert_eq!(Possible::Should, Possible::any(Possible::Should, Possible::ShouldNot));
    /// assert_eq!(Possible::Should, Possible::any(Possible::ShouldNot, Possible::Should));
    /// assert_eq!(Possible::ShouldNot, Possible::any(Possible::ShouldNot, Possible::No));
    /// assert_eq!(Possible::ShouldNot, Possible::any(Possible::No, Possible::ShouldNot));
    /// assert_eq!(Possible::No, Possible::any(Possible::No, Possible::No));
    /// ```
    pub fn any(a: Self, b: Self) -> Self {
        std::cmp::max(a, b)
    }
}

impl Range {
    pub fn opposite(&self) -> Self {
        match self {
            Range::Full => Range::Full,
            Range::Negative => Range::Positive,
            Range::Positive => Range::Negative,
        }
    }

    pub fn can_be_positive(&self) -> Possible {
        match self {
            Range::Full | Range::Positive => Possible::Yes,
            Range::Negative => Possible::No,
        }
    }

    pub fn can_be_negative(&self) -> Possible {
        match self {
            Range::Full | Range::Negative => Possible::Yes,
            Range::Positive => Possible::No,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FloatPossibilities {
    pub nan: Possible,
    pub zero: Possible,
    pub infinite: Possible,
    pub range: Range,
}

type FP = FloatPossibilities;

impl FloatPossibilities {
    /// Returns true if the value is accepted
    ///
    /// ```
    /// use float_fn_types::{FloatPossibilities, Possible, Range};
    ///
    /// let possibilities = FloatPossibilities {
    ///     nan: Possible::Yes,
    ///     zero: Possible::Yes,
    ///     infinite: Possible::Yes,
    ///     range: Range::Full,
    /// };
    ///
    /// assert!(possibilities.accept(f64::NAN));
    /// assert!(possibilities.accept(f64::INFINITY));
    /// assert!(possibilities.accept(f64::NEG_INFINITY));
    /// assert!(possibilities.accept(0.0));
    /// assert!(possibilities.accept(-0.0));
    /// assert!(possibilities.accept(1.0));
    /// assert!(possibilities.accept(-1.0));
    /// ```
    pub fn accept(&self, value: f64) -> bool {
        if value.is_nan() {
            return self.nan != Possible::No;
        }

        if value.is_infinite() && self.infinite == Possible::No {
            return false;
        }

        if value == 0.0 && self.zero == Possible::No {
            return false;
        }

        if value.is_sign_positive() && self.range.can_be_positive() == Possible::No {
            return false;
        }

        if value.is_sign_negative() && self.range.can_be_negative() == Possible::No {
            return false;
        }

        true
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FnArg {
    F32(FloatPossibilities),
    F64(FloatPossibilities),
}

macro_rules! return_possibilities {
    ($lhs:ident) => {
        match $lhs {
            FnArg::F32(lhs) => FnArg::F32(possibilities(lhs)),
            FnArg::F64(lhs) => FnArg::F64(possibilities(lhs)),
        }
    };
}

pub mod core {
    pub mod ops {
        use crate::*;

        pub fn neg(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: lhs.range.opposite(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn abs(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: Range::Positive,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn ceil(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                let (a, b) = (lhs.zero, lhs.range.can_be_negative());

                FP {
                    zero: Possible::any(a, b),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn floor(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    zero: Possible::any(lhs.zero, lhs.range.can_be_positive()),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn round(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    zero: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn trunc(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    zero: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn fract(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    zero: Possible::Yes,
                    nan: Possible::any(lhs.nan, lhs.infinite),
                    // Returns POSITIVE zero if the factional part is zero
                    range: if lhs.range.can_be_negative() == Possible::Yes {
                        Range::Full
                    } else {
                        Range::Positive
                    },
                    infinite: lhs.infinite,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn signum(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    zero: Possible::No,
                    infinite: Possible::No,
                    range: lhs.range,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn sqrt(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    nan: Possible::any(lhs.nan, lhs.range.can_be_negative()),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn exp(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: Range::Positive,
                    zero: lhs.range.can_be_negative(),
                    infinite: lhs.range.can_be_positive(),
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn exp2(lhs: &FnArg) -> FnArg {
            exp(lhs)
        }

        pub fn ln(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: Range::Full,
                    zero: lhs.range.can_be_positive(),
                    infinite: Possible::any(lhs.infinite, lhs.zero),
                    nan: Possible::any(lhs.nan, lhs.range.can_be_negative()),
                }
            }

            return_possibilities!(lhs)
        }

        pub fn log2(lhs: &FnArg) -> FnArg {
            ln(lhs)
        }

        pub fn log10(lhs: &FnArg) -> FnArg {
            ln(lhs)
        }

        pub fn to_degrees(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    // May reach Infinity with large values
                    infinite: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn to_radians(lhs: &FnArg) -> FnArg {
            *lhs
        }

        pub fn cbrt(lhs: &FnArg) -> FnArg {
            *lhs
        }

        pub fn sin(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: Range::Full,
                    zero: Possible::Yes,
                    infinite: Possible::No,
                    nan: Possible::any(lhs.nan, lhs.infinite),
                }
            }

            return_possibilities!(lhs)
        }

        pub fn cos(lhs: &FnArg) -> FnArg {
            sin(lhs)
        }

        pub fn tan(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: Range::Full,
                    zero: Possible::Yes,
                    infinite: Possible::Yes,
                    nan: Possible::any(lhs.nan, lhs.infinite),
                }
            }

            return_possibilities!(lhs)
        }

        pub fn asin(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: lhs.range,
                    zero: lhs.zero,
                    infinite: Possible::No,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn acos(lhs: &FnArg) -> FnArg {
            fn possibilities(_lhs: &FP) -> FP {
                FP {
                    range: Range::Positive,
                    zero: Possible::Yes,
                    infinite: Possible::No,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn atan(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    infinite: Possible::No,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn exp_m1(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    infinite: lhs.range.can_be_positive(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn ln_1p(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    nan: Possible::any(lhs.nan, lhs.range.can_be_negative()),
                    infinite: Possible::any(lhs.infinite, lhs.range.can_be_negative()),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn sinh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    infinite: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn cosh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: Range::Positive,
                    zero: Possible::No,
                    infinite: Possible::Yes,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn tanh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    infinite: Possible::No,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn asinh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    infinite: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn acosh(lhs: &FnArg) -> FnArg {
            fn possibilities(_lhs: &FP) -> FP {
                FP {
                    range: Range::Positive,
                    zero: Possible::Yes,
                    infinite: Possible::Yes,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn atanh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: lhs.range,
                    zero: lhs.zero,
                    infinite: Possible::Yes,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn recip(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: lhs.range,
                    zero: lhs.infinite,
                    infinite: lhs.zero,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        // TODO: add argument
        pub fn powi(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FP) -> FP {
                FP {
                    range: if lhs.range.can_be_negative() == Possible::Yes {
                        Range::Full
                    } else {
                        Range::Positive
                    },
                    zero: Possible::Yes,
                    infinite: Possible::Yes,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }
    }
}
