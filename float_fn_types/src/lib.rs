#[derive(Debug, Clone, Copy, Default)]
pub struct FloatPossibilities {
    pub nan: bool,
    pub zero: bool,
    pub infinite: bool,
    pub positive: bool,
    pub negative: bool,
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
                    positive: lhs.negative,
                    negative: lhs.positive,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn abs(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    negative: false,
                    positive: true,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn ceil(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: lhs.zero || lhs.negative,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn floor(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: lhs.zero || lhs.positive,
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
                    positive: true, // Returns POSITIVE zero if the factional part is zero
                    nan: lhs.nan || lhs.infinite,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn signum(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    zero: false,
                    infinite: false,
                    positive: lhs.positive,
                    negative: lhs.negative,
                    nan: false,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn sqrt(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    nan: lhs.negative,
                    ..*lhs
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn exp2(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    negative: false,
                    positive: true,
                    zero: lhs.negative,
                    infinite: lhs.positive,
                    nan: lhs.nan,
                }
            }

            return_possibilities!(lhs)
        }

        pub const fn ln(lhs: &FnArg) -> FnArg {
            const fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    negative: lhs.positive,
                    positive: lhs.positive,
                    zero: lhs.positive,
                    infinite: lhs.infinite || lhs.zero,
                    nan: lhs.negative,
                }
            }

            return_possibilities!(lhs)
        }
    }
}
