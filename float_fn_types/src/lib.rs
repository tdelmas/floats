
#[derive(Debug,Clone,Copy, Default)]
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

pub mod core {
    pub mod ops {
        use crate::*;

        pub fn neg(lhs: &FnArg) -> FnArg {
            fn possibilities(lhs: &FloatPossibilities) -> FloatPossibilities {
                FloatPossibilities {
                    positive: lhs.negative,
                    negative: lhs.positive,
                    ..*lhs
                }
            }

            match lhs {
                FnArg::F32(lhs) => FnArg::F32(possibilities(lhs)),
                FnArg::F64(lhs) => FnArg::F64(possibilities(lhs)),
            }
        }
    }
}
