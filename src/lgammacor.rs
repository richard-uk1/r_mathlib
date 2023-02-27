#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_chebyshev_eval(
        _: libc::c_double,
        _: *const libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_lgammacor(mut x: libc::c_double) -> libc::c_double {
    static mut algmcs: [libc::c_double; 15] = [
        0.1666389480451863247205729650822e+0f64,
        -0.1384948176067563840732986059135e-4f64,
        0.9810825646924729426157171547487e-8f64,
        -0.1809129475572494194263306266719e-10f64,
        0.6221098041892605227126015543416e-13f64,
        -0.3399615005417721944303330599666e-15f64,
        0.2683181998482698748957538846666e-17f64,
        -0.2868042435334643284144622399999e-19f64,
        0.3962837061046434803679306666666e-21f64,
        -0.6831888753985766870111999999999e-23f64,
        0.1429227355942498147573333333333e-24f64,
        -0.3547598158101070547199999999999e-26f64,
        0.1025680058010470912000000000000e-27f64,
        -0.3401102254316748799999999999999e-29f64,
        0.1276642195630062933333333333333e-30f64,
    ];
    let mut tmp: libc::c_double = 0.;
    if x < 10 as libc::c_int as libc::c_double {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg = b"argument out of domain in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                2 => {
                    msg = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    } else {
        if x >= 3.745194030963158e306f64 {
            if 16 as libc::c_int > 1 as libc::c_int {
                let mut msg_0: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 16 as libc::c_int {
                    1 => {
                        msg_0 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_0 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_0 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_0 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_0 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_0, b"lgammacor\0" as *const u8 as *const libc::c_char);
            }
        } else if x < 94906265.62425156f64 {
            tmp = 10 as libc::c_int as libc::c_double / x;
            return Rf_chebyshev_eval(
                tmp * tmp * 2 as libc::c_int as libc::c_double - 1 as libc::c_int as libc::c_double,
                algmcs.as_ptr(),
                5 as libc::c_int,
            ) / x;
        }
    }
    return 1 as libc::c_int as libc::c_double / (x * 12 as libc::c_int as libc::c_double);
}
