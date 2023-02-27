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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn qnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pbinom(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
unsafe extern "C" fn do_search(
    mut y: libc::c_double,
    mut z: *mut libc::c_double,
    mut p: libc::c_double,
    mut n: libc::c_double,
    mut pr: libc::c_double,
    mut incr: libc::c_double,
) -> libc::c_double {
    if *z >= p {
        loop {
            let mut newz: libc::c_double = 0.;
            if y == 0 as libc::c_int as libc::c_double || {
                newz = pbinom(y - incr, n, pr, TRUE as libc::c_int, FALSE as libc::c_int);
                newz < p
            } {
                return y;
            }
            y = fmax2(0 as libc::c_int as libc::c_double, y - incr);
            *z = newz;
        }
    } else {
        loop {
            y = fmin2(y + incr, n);
            if y == n || {
                *z = pbinom(y, n, pr, TRUE as libc::c_int, FALSE as libc::c_int);
                *z >= p
            } {
                return y;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn qbinom(
    mut p: libc::c_double,
    mut n: libc::c_double,
    mut pr: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut q: libc::c_double = 0.;
    let mut mu: libc::c_double = 0.;
    let mut sigma: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
        || pr.is_nan() as i32 != 0 as libc::c_int
    {
        return p + n + pr;
    }
    if R_finite(n) == 0 || R_finite(pr) == 0 {
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
    if R_finite(p) == 0 && log_p == 0 {
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
    if n != floor(n + 0.5f64) {
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
    if pr < 0 as libc::c_int as libc::c_double
        || pr > 1 as libc::c_int as libc::c_double
        || n < 0 as libc::c_int as libc::c_double
    {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg_2: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg_2 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_2 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_2 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_2 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_2 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_2, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_3: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_3 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_3 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_3 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_3 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_3 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_3, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                n
            } else {
                0 as libc::c_int as libc::c_double
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                0 as libc::c_int as libc::c_double
            } else {
                n
            };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_4: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_4 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_4 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_4 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_4 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_4 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_4, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                0 as libc::c_int as libc::c_double
            } else {
                n
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                n
            } else {
                0 as libc::c_int as libc::c_double
            };
        }
    }
    if pr == 0.0f64 || n == 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    }
    q = 1 as libc::c_int as libc::c_double - pr;
    if q == 0.0f64 {
        return n;
    }
    mu = n * pr;
    sigma = sqrt(n * pr * q);
    gamma = (q - pr) / sigma;
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
            return 0.0f64;
        }
        if p == 1.0f64 {
            return n;
        }
    }
    if p + 1.01f64 * 2.2204460492503131e-16f64 >= 1.0f64 {
        return n;
    }
    z = qnorm5(p, 0.0f64, 1.0f64, TRUE as libc::c_int, FALSE as libc::c_int);
    y = floor(
        mu + sigma
            * (z + gamma * (z * z - 1 as libc::c_int as libc::c_double)
                / 6 as libc::c_int as libc::c_double)
            + 0.5f64,
    );
    if y > n {
        y = n;
    }
    z = pbinom(y, n, pr, TRUE as libc::c_int, FALSE as libc::c_int);
    p *= 1 as libc::c_int as libc::c_double
        - 64 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
    if n < 1e5f64 {
        return do_search(y, &mut z, p, n, pr, 1 as libc::c_int as libc::c_double);
    }
    let mut incr: libc::c_double = floor(n * 0.001f64);
    let mut oldincr: libc::c_double = 0.;
    loop {
        oldincr = incr;
        y = do_search(y, &mut z, p, n, pr, incr);
        incr = fmax2(
            1 as libc::c_int as libc::c_double,
            floor(incr / 100 as libc::c_int as libc::c_double),
        );
        if !(oldincr > 1 as libc::c_int as libc::c_double && incr > n * 1e-15f64) {
            break;
        }
    }
    return y;
}
