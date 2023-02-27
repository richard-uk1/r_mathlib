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
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dnorm4(
    mut x: libc::c_double,
    mut mu: libc::c_double,
    mut sigma: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || mu.is_nan() as i32 != 0 as libc::c_int
        || sigma.is_nan() as i32 != 0 as libc::c_int
    {
        return x + mu + sigma;
    }
    if sigma < 0 as libc::c_int as libc::c_double {
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
    if R_finite(sigma) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if R_finite(x) == 0 && mu == x {
        return 0.0f64 / 0.0f64;
    }
    if sigma == 0 as libc::c_int as libc::c_double {
        return if x == mu {
            1.0f64 / 0.0f64
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    x = (x - mu) / sigma;
    if R_finite(x) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    x = fabs(x);
    if x >= 2 as libc::c_int as libc::c_double * sqrt(1.7976931348623157e+308f64) {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if log_p != 0 {
        return -(0.918938533204672741780329736406f64 + 0.5f64 * x * x + log(sigma));
    }
    if x < 5 as libc::c_int as libc::c_double {
        return 0.398942280401432677939946059934f64 * exp(-0.5f64 * x * x) / sigma;
    }
    if x > sqrt(
        -(2 as libc::c_int) as libc::c_double
            * 0.693147180559945309417232121458f64
            * (-(1021 as libc::c_int) + 1 as libc::c_int - 53 as libc::c_int) as libc::c_double,
    ) {
        return 0.0f64;
    }
    let mut x1: libc::c_double = ldexp(round(ldexp(x, 16 as libc::c_int)), -(16 as libc::c_int));
    let mut x2: libc::c_double = x - x1;
    return 0.398942280401432677939946059934f64 / sigma
        * (exp(-0.5f64 * x1 * x1) * exp((-0.5f64 * x2 - x1) * x2));
}
