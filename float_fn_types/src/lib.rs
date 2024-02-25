#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Range {
    #[default]
    Full,
    Negative,
    Positive,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Possible {
    #[default]
    Yes,
    No,
    WithRoundingError,
}

macro_rules! or {
    ($lhs:expr, $rhs:expr) => {
        if $lhs == Possible::Yes || $rhs == Possible::Yes {
            Possible::Yes
        } else if $lhs == Possible::WithRoundingError || $rhs == Possible::WithRoundingError {
            Possible::WithRoundingError
        } else {
            Possible::No
        }
    };
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

#[derive(Clone, Copy)]
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
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: lhs.range.opposite(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn abs(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn ceil(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: or!(lhs.zero, lhs.range.can_be_negative()),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn floor(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: or!(lhs.zero, lhs.range.can_be_positive()),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn round(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn trunc(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn fract(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: Possible::Yes,
                    nan: or!(lhs.nan, lhs.infinite),
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
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: Possible::No,
                    infinite: Possible::No,
                    range: lhs.range,
                    nan: Possible::No,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn sqrt(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    nan: lhs.range.can_be_negative(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn exp(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
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
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Full,
                    zero: lhs.range.can_be_positive(),
                    infinite: or!(lhs.infinite, lhs.zero),
                    nan: lhs.range.can_be_negative(),
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
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
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
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Full,
                    zero: Possible::Yes,
                    infinite: Possible::No,
                    nan: or!(lhs.nan, lhs.infinite),
                }
            }

            return_possibilities!(lhs)
        }

        pub fn cos(lhs: &FnArg) -> FnArg {
            sin(lhs)
        }

        pub fn tan(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Full,
                    zero: Possible::Yes,
                    infinite: Possible::Yes,
                    nan: or!(lhs.nan, lhs.infinite),
                }
            }

            return_possibilities!(lhs)
        }

        pub fn asin(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: lhs.range,
                    zero: lhs.zero,
                    infinite: Possible::No,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn acos(lhs: &FnArg) -> FnArg {
            fn possibilities(_lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: Possible::Yes,
                    infinite: Possible::No,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn atan(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: Possible::No,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn exp_m1(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: lhs.range.can_be_positive(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn ln_1p(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    nan: or!(lhs.nan, lhs.range.can_be_negative()),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn sinh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn cosh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: Possible::No,
                    infinite: Possible::Yes,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn tanh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: Possible::No,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn asinh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: Possible::Yes,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub fn acosh(lhs: &FnArg) -> FnArg {
            fn possibilities(_lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: Possible::Yes,
                    infinite: Possible::Yes,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn atanh(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: lhs.range,
                    zero: lhs.zero,
                    infinite: Possible::Yes,
                    nan: Possible::Yes,
                }
            }

            return_possibilities!(lhs)
        }

        pub fn recip(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
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
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
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
