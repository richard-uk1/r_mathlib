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
    fn qchisq(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn Rf_pnchisq_raw(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: Rboolean,
        _: Rboolean,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
#[no_mangle]
pub unsafe extern "C" fn qnchisq(
    mut p: libc::c_double,
    mut df: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut accu: libc::c_double = 1e-13f64;
    static mut racc: libc::c_double =
        4 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
    static mut Eps: libc::c_double = 1e-11f64;
    static mut rEps: libc::c_double = 1e-10f64;
    let mut ux: libc::c_double = 0.;
    let mut lx: libc::c_double = 0.;
    let mut ux0: libc::c_double = 0.;
    let mut nx: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int
        || df.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return p + df + ncp;
    }
    if R_finite(df) == 0 {
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
    if df < 0 as libc::c_int as libc::c_double || ncp < 0 as libc::c_int as libc::c_double {
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
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_1: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_1 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_1 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_1 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_1 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_1, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                0 as libc::c_int as libc::c_double
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                0 as libc::c_int as libc::c_double
            } else {
                1.0f64 / 0.0f64
            };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_2: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_2 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_2 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_2 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_2 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_2 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_2, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                0 as libc::c_int as libc::c_double
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                0 as libc::c_int as libc::c_double
            };
        }
    }
    pp = if log_p != 0 { exp(p) } else { p };
    if pp > 1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64 {
        return if lower_tail != 0 {
            1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut ff: libc::c_double = 0.;
    b = ncp * ncp / (df + 3 as libc::c_int as libc::c_double * ncp);
    c = (df + 3 as libc::c_int as libc::c_double * ncp)
        / (df + 2 as libc::c_int as libc::c_double * ncp);
    ff = (df + 2 as libc::c_int as libc::c_double * ncp) / (c * c);
    ux = b + c * qchisq(p, ff, lower_tail, log_p);
    if ux < 0 as libc::c_int as libc::c_double {
        ux = 1 as libc::c_int as libc::c_double;
    }
    ux0 = ux;
    if lower_tail == 0 && ncp >= 80 as libc::c_int as libc::c_double {
        if pp < 1e-10f64 {
            if 8 as libc::c_int > 1 as libc::c_int {
                let mut msg_3: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 8 as libc::c_int {
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
                printf(msg_3, b"qnchisq\0" as *const u8 as *const libc::c_char);
            }
        }
        p = if log_p != 0 {
            -expm1(p)
        } else {
            0.5f64 - p + 0.5f64
        };
        lower_tail = TRUE as libc::c_int;
    } else {
        p = pp;
    }
    pp = fmin2(
        1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64,
        p * (1 as libc::c_int as libc::c_double + Eps),
    );
    if lower_tail != 0 {
        while ux < 1.7976931348623157e+308f64
            && Rf_pnchisq_raw(ux, df, ncp, Eps, rEps, 10000 as libc::c_int, TRUE, FALSE) < pp
        {
            ux *= 2 as libc::c_int as libc::c_double;
        }
        pp = p * (1 as libc::c_int as libc::c_double - Eps);
        lx = fmin2(ux0, 1.7976931348623157e+308f64);
        while lx > 2.2250738585072014e-308f64
            && Rf_pnchisq_raw(lx, df, ncp, Eps, rEps, 10000 as libc::c_int, TRUE, FALSE) > pp
        {
            lx *= 0.5f64;
        }
    } else {
        while ux < 1.7976931348623157e+308f64
            && Rf_pnchisq_raw(ux, df, ncp, Eps, rEps, 10000 as libc::c_int, FALSE, FALSE) > pp
        {
            ux *= 2 as libc::c_int as libc::c_double;
        }
        pp = p * (1 as libc::c_int as libc::c_double - Eps);
        lx = fmin2(ux0, 1.7976931348623157e+308f64);
        while lx > 2.2250738585072014e-308f64
            && Rf_pnchisq_raw(lx, df, ncp, Eps, rEps, 10000 as libc::c_int, FALSE, FALSE) < pp
        {
            lx *= 0.5f64;
        }
    }
    if lower_tail != 0 {
        loop {
            nx = 0.5f64 * (lx + ux);
            if Rf_pnchisq_raw(nx, df, ncp, accu, racc, 100000 as libc::c_int, TRUE, FALSE) > p {
                ux = nx;
            } else {
                lx = nx;
            }
            if !((ux - lx) / nx > accu) {
                break;
            }
        }
    } else {
        loop {
            nx = 0.5f64 * (lx + ux);
            if Rf_pnchisq_raw(nx, df, ncp, accu, racc, 100000 as libc::c_int, FALSE, FALSE) < p {
                ux = nx;
            } else {
                lx = nx;
            }
            if !((ux - lx) / nx > accu) {
                break;
            }
        }
    }
    return 0.5f64 * (ux + lx);
}
