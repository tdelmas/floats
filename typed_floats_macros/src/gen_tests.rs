use quote::quote;

use crate::impl_self::*;
use crate::*;

fn test_op_checks(
    float: &FloatDefinition,
    op_name: &str,
    result_type: &Option<FloatDefinition>,
    var: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let (full_type, accept_inf, accept_zero, accept_positive, accept_negative) = match result_type {
        None => (float.float_type, true, true, true, true),
        Some(result_type) => (
            result_type.name,
            result_type.s.accept_inf,
            result_type.s.accept_zero,
            result_type.s.accept_positive,
            result_type.s.accept_negative,
        ),
    };

    let mut res = proc_macro2::TokenStream::new();

    let check_inf = if accept_inf {
        quote! {
            let has_inf = #var.iter().any(|x| x.is_infinite());
            assert!(has_inf, "No inf generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    let check_zero = if accept_zero {
        quote! {
            let has_zero = #var.iter().any(|x| x == &0.0);
            assert!(has_zero, "No zero generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    let check_positive = if accept_positive {
        quote! {
            let has_positive = #var.iter().any(|x| x.is_sign_positive());
            assert!(has_positive, "No positive generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    let check_negative = if accept_negative {
        quote! {
            let has_negative = #var.iter().any(|x| x.is_sign_negative());
            assert!(has_negative, "No negative generated with {} but the output type {} accept it", #op_name, stringify!(#full_type));
        }
    } else {
        quote! {}
    };

    res.extend(quote! {
        let has_nan = #var.iter().any(|x| x.is_nan());

        if !has_nan {
            #check_inf
            #check_zero
            #check_positive
            #check_negative
        }
    });

    res
}

pub(crate) fn generate_tests(float_type: &'static str) -> proc_macro2::TokenStream {
    let floats_f64 = get_definitions(float_type);

    let mut output = proc_macro2::TokenStream::new();

    let float_type = floats_f64[0].float_type_ident();

    let test_fn_name = quote::format_ident!("test_{}", float_type);

    let ops = get_impl_self();
    let ops_rhs = get_impl_self_rhs();

    for float in &floats_f64 {
        let mut init_test_ops = proc_macro2::TokenStream::new();
        let mut test_ops = proc_macro2::TokenStream::new();
        let mut check_ops = proc_macro2::TokenStream::new();

        for op in &ops {
            let op_name = &op.key;
            let vals = quote::format_ident!("all_{}", op_name);

            init_test_ops.extend(quote! {
                let mut #vals = Vec::<#float_type>::new();
            });

            let test = &op.get_test("num_a");

            let get = match &op.get_result(float, &floats_f64) {
                None => quote! { res },
                Some(_) => {
                    quote! { res.get() }
                }
            };

            test_ops.extend(quote! {
                println!("{:?} = ...",#op_name);
                let res = #test;
                println!("{:?} = {:?}",#op_name, res);
                #vals.push(#get);
            });

            let result_type = op.get_result(float, &floats_f64);
            let checks = test_op_checks(float, op.display.as_str(), &result_type, &vals);

            check_ops.extend(quote! {
                #checks
            });
        }

        let full_type = float.full_type_ident();

        output.extend(quote! {
            #init_test_ops

            for a in values.iter() {
                let a = <#full_type>::try_from(*a);

                if let Ok(num_a) = a {
                    println!("compute with a = {:?}", num_a);

                    #test_ops
                }
            }

            #check_ops
        });

        let float_type = float.float_type_ident();

        for float_rhs in &floats_f64 {
            let mut init_test_ops = proc_macro2::TokenStream::new();
            let mut test_ops = proc_macro2::TokenStream::new();
            let mut check_ops = proc_macro2::TokenStream::new();

            for op in &ops_rhs {
                let op_name = &op.key;
                let vals = quote::format_ident!("all_{}", op_name);

                init_test_ops.extend(quote! {
                    let mut #vals = Vec::<#float_type>::new();
                });

                let test = &op.get_test("num_a", "num_b");

                let get = match &op.get_result(float, float_rhs, &floats_f64) {
                    None => quote! { res },
                    Some(_) => {
                        quote! { res.get() }
                    }
                };

                test_ops.extend(quote! {
                    println!("{:?} = ...",#op_name);
                    let res = #test;
                    println!("{:?} = {:?}",#op_name, res);
                    #vals.push(#get);
                });

                let result_type = op.get_result(float, float_rhs, &floats_f64);
                let checks = test_op_checks(float, op.display.as_str(), &result_type, &vals);

                check_ops.extend(quote! {
                    #checks
                });
            }

            let full_type_rhs = float_rhs.full_type_ident();

            output.extend(quote! {
                #init_test_ops

                for a in values.iter() {
                    let a = <#full_type>::try_from(*a);

                    if let Ok(num_a) = a {
                        for b in values.iter() {
                            let b = <#full_type_rhs>::try_from(*b);

                            if let Ok(num_b) = b {
                                println!("a = {:?} and b = {:?}", num_a, num_b);

                                #test_ops
                            }
                        }
                    }
                }

                #check_ops

            });
        }
    }

    quote! {
        #[test]
        fn #test_fn_name() {

            const MAX_NEGATIVE: #float_type = -#float_type::MIN_POSITIVE;

            assert!(MAX_NEGATIVE.is_sign_negative());
            assert!(MAX_NEGATIVE > -0.1);
            assert!(MAX_NEGATIVE < -0.0);

            let values = [
                #float_type::NAN,
                #float_type::NEG_INFINITY,
                #float_type::MIN,
                -2.0,
                -1.0,
                MAX_NEGATIVE,
                -0.0,
                0.0,
                #float_type::MIN_POSITIVE,
                1.0,
                2.0,
                #float_type::MAX,
                #float_type::INFINITY,
            ];

            const SKIP_NAN:usize = 2;
            for i in SKIP_NAN..values.len() {
                if values[i] != 0.0 {
                    assert!(values[i] > values[i-1], "values[{}] = {} <= values[{}] = {}", i-1, values[i-1], i, values[i]);
                }
            }

            #output
        }
    }
}
