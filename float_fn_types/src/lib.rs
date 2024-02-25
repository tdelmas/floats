#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Range {
    #[default]
    Full,
    Negative,
    Positive,
}

// TODO: true/false/roundingError

impl Range {
    pub const fn opposite(&self) -> Self {
        match self {
            Range::Full => Range::Full,
            Range::Negative => Range::Positive,
            Range::Positive => Range::Negative,
        }
    }

    pub const fn can_be_positive(&self) -> bool {
        matches!(self, Range::Full | Range::Positive)
    }

    pub const fn can_be_negative(&self) -> bool {
        matches!(self, Range::Full | Range::Negative)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FloatPossibilities {
    pub nan: bool,
    pub zero: bool,
    pub infinite: bool,
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

        pub const fn neg(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: lhs.range.opposite(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn abs(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn ceil(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: lhs.zero || lhs.range.can_be_negative(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn floor(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: lhs.zero || lhs.range.can_be_positive(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn round(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities { zero: true, ..*lhs }
            }

            return_possibilities!(lhs)
        }

        pub const fn trunc(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities { zero: true, ..*lhs }
            }

            return_possibilities!(lhs)
        }

        pub const fn fract(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: true,
                    nan: lhs.nan || lhs.infinite,
                    // Returns POSITIVE zero if the factional part is zero
                    range: if lhs.range.can_be_negative() {
                        Range::Full
                    } else {
                        Range::Positive
                    },
                    infinite: lhs.infinite,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn signum(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: false,
                    infinite: false,
                    range: lhs.range,
                    nan: false,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn sqrt(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    nan: lhs.range.can_be_negative(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn exp(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: lhs.range.can_be_negative(),
                    infinite: lhs.range.can_be_positive(),
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn exp2(lhs: &FnArg) -> FnArg {
            exp(lhs)
        }

        pub const fn ln(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Full,
                    zero: lhs.range.can_be_positive(),
                    infinite: lhs.infinite || lhs.zero,
                    nan: lhs.range.can_be_negative(),
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn log2(lhs: &FnArg) -> FnArg {
            ln(lhs)
        }

        pub const fn log10(lhs: &FnArg) -> FnArg {
            ln(lhs)
        }

        pub const fn to_degrees(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    // May reach Infinity with large values
                    infinite: true,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn to_radians(lhs: &FnArg) -> FnArg {
            *lhs
        }

        pub const fn cbrt(lhs: &FnArg) -> FnArg {
            *lhs
        }

        pub const fn sin(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Full,
                    zero: true,
                    infinite: false,
                    nan: lhs.nan || lhs.infinite,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn cos(lhs: &FnArg) -> FnArg {
            sin(lhs)
        }

        pub const fn tan(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Full,
                    zero: true,
                    infinite: true,
                    nan: lhs.nan || lhs.infinite,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn asin(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: lhs.range,
                    zero: lhs.zero,
                    infinite: false,
                    nan: true,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn acos(lhs: &FnArg) -> FnArg {
            const fn possibilities(_lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: true,
                    infinite: false,
                    nan: true,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn atan(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: false,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn exp_m1(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: lhs.range.can_be_positive(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn ln_1p(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    nan: lhs.nan || lhs.range.can_be_negative(),
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn sinh(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: true,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn cosh(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: false,
                    infinite: true,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn tanh(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: false,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn asinh(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    infinite: true,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn acosh(lhs: &FnArg) -> FnArg {
            const fn possibilities(_lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: Range::Positive,
                    zero: true,
                    infinite: true,
                    nan: true,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn atanh(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: lhs.range,
                    zero: lhs.zero,
                    infinite: true,
                    nan: true,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn recip(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
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
        pub const fn powi(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    range: if lhs.range.can_be_negative() {
                        Range::Full
                    } else {
                        Range::Positive
                    },
                    zero: true,
                    infinite: true,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }
    }
}
