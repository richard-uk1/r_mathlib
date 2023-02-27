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
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn nearbyint(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn qnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn ppois(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
unsafe extern "C" fn do_search(
    mut y: libc::c_double,
    mut z: *mut libc::c_double,
    mut p: libc::c_double,
    mut lambda: libc::c_double,
    mut incr: libc::c_double,
) -> libc::c_double {
    if *z >= p {
        loop {
            if y == 0 as libc::c_int as libc::c_double || {
                *z = ppois(y - incr, lambda, TRUE as libc::c_int, FALSE as libc::c_int);
                *z < p
            } {
                return y;
            }
            y = fmax2(0 as libc::c_int as libc::c_double, y - incr);
        }
    } else {
        loop {
            y = y + incr;
            *z = ppois(y, lambda, TRUE as libc::c_int, FALSE as libc::c_int);
            if *z >= p {
                return y;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn qpois(
    mut p: libc::c_double,
    mut lambda: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut mu: libc::c_double = 0.;
    let mut sigma: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int || lambda.is_nan() as i32 != 0 as libc::c_int {
        return p + lambda;
    }
    if R_finite(lambda) == 0 {
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
    if lambda < 0 as libc::c_int as libc::c_double {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg_0: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg_0 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_0 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_0 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_0 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_0 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_0, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if log_p != 0 && p > 0 as libc::c_int as libc::c_double
        || log_p == 0
            && (p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double)
    {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg_1: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_1 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_1 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_1 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_1 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_1, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if lambda == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    if p == (if lower_tail != 0 {
        if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        }
    } else {
        if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        }
    }) {
        return 0 as libc::c_int as libc::c_double;
    }
    if p == (if lower_tail != 0 {
        if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        }
    } else {
        if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        }
    }) {
        return 1.0f64 / 0.0f64;
    }
    mu = lambda;
    sigma = sqrt(lambda);
    gamma = 1.0f64 / sigma;
    if lower_tail == 0 || log_p != 0 {
        p = if log_p != 0 {
            if lower_tail != 0 {
                exp(p)
            } else {
                -expm1(p)
            }
        } else if lower_tail != 0 {
            p
        } else {
            0.5f64 - p + 0.5f64
        };
        if p == 0.0f64 {
            return 0 as libc::c_int as libc::c_double;
        }
        if p == 1.0f64 {
            return 1.0f64 / 0.0f64;
        }
    }
    if p + 1.01f64 * 2.2204460492503131e-16f64 >= 1.0f64 {
        return 1.0f64 / 0.0f64;
    }
    z = qnorm5(p, 0.0f64, 1.0f64, TRUE as libc::c_int, FALSE as libc::c_int);
    y = nearbyint(
        mu + sigma
            * (z + gamma * (z * z - 1 as libc::c_int as libc::c_double)
                / 6 as libc::c_int as libc::c_double),
    );
    z = ppois(y, lambda, TRUE as libc::c_int, FALSE as libc::c_int);
    p *= 1 as libc::c_int as libc::c_double
        - 64 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
    if lambda < 1e5f64 {
        return do_search(y, &mut z, p, lambda, 1 as libc::c_int as libc::c_double);
    }
    let mut incr: libc::c_double = floor(y * 0.001f64);
    let mut oldincr: libc::c_double = 0.;
    loop {
        oldincr = incr;
        y = do_search(y, &mut z, p, lambda, incr);
        incr = fmax2(
            1 as libc::c_int as libc::c_double,
            floor(incr / 100 as libc::c_int as libc::c_double),
        );
        if !(oldincr > 1 as libc::c_int as libc::c_double && incr > lambda * 1e-15f64) {
            break;
        }
    }
    return y;
}
