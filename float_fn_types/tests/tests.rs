use float_fn_types::{FloatPossibilities, FnArg, Possible, Range};

macro_rules! get_test_values {
    ($float_type:ident) => {
        [
            core::$float_type::NAN,
            core::$float_type::NEG_INFINITY,
            core::$float_type::MIN,
            core::$float_type::MIN / 2.0,
            -core::$float_type::consts::PI,
            -core::$float_type::consts::E,
            -2.0,
            -core::$float_type::consts::FRAC_PI_2,
            -1.0,
            -core::$float_type::MIN_POSITIVE,
            -1.0e-308,
            -0.0,
            0.0,
            1.0e-308,
            core::$float_type::MIN_POSITIVE,
            1.0,
            core::$float_type::consts::FRAC_PI_2,
            2.0,
            core::$float_type::consts::E,
            core::$float_type::consts::PI,
            core::$float_type::MAX / 2.0,
            core::$float_type::MAX,
            core::$float_type::INFINITY,
        ]
    };
}

const YESNO: [Possible; 2] = [Possible::Yes, Possible::No];
const RANGES: [Range; 3] = [Range::Full, Range::Positive, Range::Negative];

fn get_possibilities() -> Vec<FloatPossibilities> {
    let mut possibles = vec![];

    for nan in YESNO {
        for zero in YESNO {
            for infinite in YESNO {
                for range in RANGES {
                    possibles.push(FloatPossibilities {
                        nan,
                        zero,
                        infinite,
                        range,
                    });
                }
            }
        }
    }

    possibles
}

#[test]
fn test_values() {
    let values = get_test_values!(f64);

    for w in values.windows(2) {
        let a = w[0];
        let b = w[1];

        if a.is_nan() || b.is_nan() {
            continue;
        }

        if a == b {
            assert_eq!(a, 0.0);
            assert_eq!(b, 0.0);
            assert!(a.is_sign_negative());
            assert!(b.is_sign_positive());
        } else {
            assert!(a < b);
        }
    }
}

fn test_op(name: &str, op: fn(f64) -> f64, ty: fn(&FnArg) -> FnArg) {
    let possibles = get_possibilities();
    let values = get_test_values!(f64);

    for v in values.iter() {
        for p in possibles.iter() {
            if !p.accept(*v) {
                continue;
            }

            let result = op(*v);
            let res_p = ty(&FnArg::F64(*p));

            println!("Testing {name}");
            println!("Testing {v:?} = {result:?}");
            println!("Testing {p:?} = {res_p:?}");

            match res_p {
                FnArg::F64(res_p) => {
                    assert!(res_p.accept(result));
                }
                _ => panic!("Expected f64"),
            }
        }
    }
}

macro_rules! test_op {
    ($op:ident) => {
        test_op(stringify!($op), |x| x.$op(), float_fn_types::core::ops::$op);
    };
}
#[test]
fn test_ops() {
    test_op("neg", |x| -x, float_fn_types::core::ops::neg);
    test_op!(abs);
    test_op!(ceil);
    test_op!(floor);
    test_op!(round);
    test_op!(trunc);
    test_op!(fract);
    test_op!(signum);
    test_op!(sqrt);
    test_op!(exp);
    test_op!(exp2);
    test_op!(ln);
    test_op!(log2);
    test_op!(log10);
    test_op!(to_degrees);
    test_op!(to_radians);
    test_op!(cbrt);
    test_op!(sin);
    test_op!(cos);
    test_op!(tan);
    test_op!(asin);
    test_op!(acos);
    test_op!(atan);
    test_op!(exp_m1);
    test_op!(ln_1p);
    test_op!(sinh);
    test_op!(cosh);
    test_op!(tanh);
    test_op!(asinh);
    test_op!(acosh);
    test_op!(atanh);
    test_op!(recip);
    test_op(
        "powi",
        |x| x.powi(2),
        |x| float_fn_types::core::ops::powi(x),
    );
}
