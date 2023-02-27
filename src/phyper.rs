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
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn dhyper(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
}
unsafe extern "C" fn pdhyper(
    mut x: libc::c_double,
    mut NR: libc::c_double,
    mut NB: libc::c_double,
    mut n: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut term: libc::c_double = 1 as libc::c_int as libc::c_double;
    while x > 0 as libc::c_int as libc::c_double && term >= 2.2204460492503131e-16f64 * sum {
        term *= x * (NB - n + x)
            / (n + 1 as libc::c_int as libc::c_double - x)
            / (NR + 1 as libc::c_int as libc::c_double - x);
        sum += term;
        x -= 1.;
    }
    let mut ss: libc::c_double = sum;
    return if log_p != 0 {
        Rlog1p(ss)
    } else {
        1 as libc::c_int as libc::c_double + ss
    };
}
#[no_mangle]
pub unsafe extern "C" fn phyper(
    mut x: libc::c_double,
    mut NR: libc::c_double,
    mut NB: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    let mut pd: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || NR.is_nan() as i32 != 0 as libc::c_int
        || NB.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return x + NR + NB + n;
    }
    x = floor(x + 1e-7f64);
    NR = round(NR);
    NB = round(NB);
    n = round(n);
    if NR < 0 as libc::c_int as libc::c_double
        || NB < 0 as libc::c_int as libc::c_double
        || R_finite(NR + NB) == 0
        || n < 0 as libc::c_int as libc::c_double
        || n > NR + NB
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
    if x * (NR + NB) > n * NR {
        let mut oldNB: libc::c_double = NB;
        NB = NR;
        NR = oldNB;
        x = n - x - 1 as libc::c_int as libc::c_double;
        lower_tail = (lower_tail == 0) as libc::c_int;
    }
    if x < 0 as libc::c_int as libc::c_double || x < n - NB {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    if x >= NR || x >= n {
        return if lower_tail != 0 {
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
    d = dhyper(x, NR, NB, n, log_p);
    if log_p == 0 && d == 0.0f64 || log_p != 0 && d == -1.0f64 / 0.0f64 {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    pd = pdhyper(x, NR, NB, n, log_p);
    return if log_p != 0 {
        if lower_tail != 0 {
            d + pd
        } else if d + pd > -0.693147180559945309417232121458f64 {
            log(-expm1(d + pd))
        } else {
            Rlog1p(-exp(d + pd))
        }
    } else if lower_tail != 0 {
        d * pd
    } else {
        0.5f64 - d * pd + 0.5f64
    };
}
