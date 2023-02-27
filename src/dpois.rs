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
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn Rf_bd0(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn Rf_stirlerr(_: libc::c_double) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn dpois_raw(
    mut x: libc::c_double,
    mut lambda: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if lambda == 0 as libc::c_int as libc::c_double {
        return if x == 0 as libc::c_int as libc::c_double {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if R_finite(lambda) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x < 0 as libc::c_int as libc::c_double {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x <= lambda * 2.2250738585072014e-308f64 {
        return if log_p != 0 { -lambda } else { exp(-lambda) };
    }
    if lambda < x * 2.2250738585072014e-308f64 {
        if R_finite(x) == 0 {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
        return if log_p != 0 {
            -lambda + x * log(lambda) - lgammafn(x + 1 as libc::c_int as libc::c_double)
        } else {
            exp(-lambda + x * log(lambda) - lgammafn(x + 1 as libc::c_int as libc::c_double))
        };
    }
    return if log_p != 0 {
        -0.5f64 * log(6.283185307179586476925286766559f64 * x)
            + (-Rf_stirlerr(x) - Rf_bd0(x, lambda))
    } else {
        exp(-Rf_stirlerr(x) - Rf_bd0(x, lambda)) / sqrt(6.283185307179586476925286766559f64 * x)
    };
}
#[no_mangle]
pub unsafe extern "C" fn dpois(
    mut x: libc::c_double,
    mut lambda: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int || lambda.is_nan() as i32 != 0 as libc::c_int {
        return x + lambda;
    }
    if lambda < 0 as libc::c_int as libc::c_double {
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
    }
    if fabs(x - round(x)) > 1e-7f64 * fmax2(1.0f64, fabs(x)) {
        printf(
            b"non-integer x = %f\0" as *const u8 as *const libc::c_char,
            x,
        );
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x < 0 as libc::c_int as libc::c_double || R_finite(x) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    x = round(x);
    return dpois_raw(x, lambda, log_p);
}
