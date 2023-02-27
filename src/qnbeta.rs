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
    fn pnbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn qnbeta(
    mut p: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut accu: libc::c_double = 1e-15f64;
    static mut Eps: libc::c_double = 1e-14f64;
    let mut ux: libc::c_double = 0.;
    let mut lx: libc::c_double = 0.;
    let mut nx: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int
        || a.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return p + a + b + ncp;
    }
    if R_finite(a) == 0 {
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
    if ncp < 0.0f64 || a <= 0.0f64 || b <= 0.0f64 {
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
            return (if lower_tail != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_double;
        }
        if p == -1.0f64 / 0.0f64 {
            return (if lower_tail != 0 {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_double;
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
            return (if lower_tail != 0 {
                0 as libc::c_int
            } else {
                1 as libc::c_int
            }) as libc::c_double;
        }
        if p == 1 as libc::c_int as libc::c_double {
            return (if lower_tail != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_double;
        }
    }
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
    if p > 1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64 {
        return 1.0f64;
    }
    pp = fmin2(
        1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64,
        p * (1 as libc::c_int as libc::c_double + Eps),
    );
    ux = 0.5f64;
    while ux < 1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64
        && pnbeta(ux, a, b, ncp, TRUE as libc::c_int, FALSE as libc::c_int) < pp
    {
        ux = 0.5f64 * (1 as libc::c_int as libc::c_double + ux);
    }
    pp = p * (1 as libc::c_int as libc::c_double - Eps);
    lx = 0.5f64;
    while lx > 2.2250738585072014e-308f64
        && pnbeta(lx, a, b, ncp, TRUE as libc::c_int, FALSE as libc::c_int) > pp
    {
        lx *= 0.5f64;
    }
    loop {
        nx = 0.5f64 * (lx + ux);
        if pnbeta(nx, a, b, ncp, TRUE as libc::c_int, FALSE as libc::c_int) > p {
            ux = nx;
        } else {
            lx = nx;
        }
        if !((ux - lx) / nx > accu) {
            break;
        }
    }
    return 0.5f64 * (ux + lx);
}
