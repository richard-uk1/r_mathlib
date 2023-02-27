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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn lgamma(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn dbinom_raw(
        x: libc::c_double,
        n: libc::c_double,
        p: libc::c_double,
        q: libc::c_double,
        give_log_0: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dpois_raw(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn dnbinom(
    mut x: libc::c_double,
    mut size: libc::c_double,
    mut prob: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || size.is_nan() as i32 != 0 as libc::c_int
        || prob.is_nan() as i32 != 0 as libc::c_int
    {
        return x + size + prob;
    }
    if prob <= 0 as libc::c_int as libc::c_double
        || prob > 1 as libc::c_int as libc::c_double
        || size < 0 as libc::c_int as libc::c_double
    {
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
    if x == 0 as libc::c_int as libc::c_double && size == 0 as libc::c_int as libc::c_double {
        return if log_p != 0 { 0.0f64 } else { 1.0f64 };
    }
    x = round(x);
    if R_finite(size) == 0 {
        size = 1.7976931348623157e+308f64;
    }
    ans = dbinom_raw(
        size,
        x + size,
        prob,
        1 as libc::c_int as libc::c_double - prob,
        log_p,
    );
    p = size / (size + x);
    return if log_p != 0 { log(p) + ans } else { p * ans };
}
#[no_mangle]
pub unsafe extern "C" fn dnbinom_mu(
    mut x: libc::c_double,
    mut size: libc::c_double,
    mut mu: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || size.is_nan() as i32 != 0 as libc::c_int
        || mu.is_nan() as i32 != 0 as libc::c_int
    {
        return x + size + mu;
    }
    if mu < 0 as libc::c_int as libc::c_double || size < 0 as libc::c_int as libc::c_double {
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
    if x == 0 as libc::c_int as libc::c_double && size == 0 as libc::c_int as libc::c_double {
        return if log_p != 0 { 0.0f64 } else { 1.0f64 };
    }
    x = round(x);
    if R_finite(size) == 0 {
        return dpois_raw(x, mu, log_p);
    }
    if x == 0 as libc::c_int as libc::c_double {
        return if log_p != 0 {
            size * (if size < mu {
                log(size / (size + mu))
            } else {
                Rlog1p(-mu / (size + mu))
            })
        } else {
            exp(size
                * (if size < mu {
                    log(size / (size + mu))
                } else {
                    Rlog1p(-mu / (size + mu))
                }))
        };
    }
    if x < 1e-10f64 * size {
        let mut p: libc::c_double = if size < mu {
            log(size / (1 as libc::c_int as libc::c_double + size / mu))
        } else {
            log(mu / (1 as libc::c_int as libc::c_double + mu / size))
        };
        return if log_p != 0 {
            x * p - mu - lgamma(x + 1 as libc::c_int as libc::c_double)
                + Rlog1p(
                    x * (x - 1 as libc::c_int as libc::c_double)
                        / (2 as libc::c_int as libc::c_double * size),
                )
        } else {
            exp(x * p - mu - lgamma(x + 1 as libc::c_int as libc::c_double)
                + Rlog1p(
                    x * (x - 1 as libc::c_int as libc::c_double)
                        / (2 as libc::c_int as libc::c_double * size),
                ))
        };
    } else {
        let mut p_0: libc::c_double = size / (size + x);
        let mut ans: libc::c_double =
            dbinom_raw(size, x + size, size / (size + mu), mu / (size + mu), log_p);
        return if log_p != 0 {
            log(p_0) + ans
        } else {
            p_0 * ans
        };
    };
}
