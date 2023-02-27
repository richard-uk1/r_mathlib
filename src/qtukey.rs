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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn ptukey(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
unsafe extern "C" fn qinv(
    mut p: libc::c_double,
    mut c: libc::c_double,
    mut v: libc::c_double,
) -> libc::c_double {
    static mut p0: libc::c_double = 0.322232421088f64;
    static mut q0: libc::c_double = 0.993484626060e-01f64;
    static mut p1: libc::c_double = -1.0f64;
    static mut q1: libc::c_double = 0.588581570495f64;
    static mut p2: libc::c_double = -0.342242088547f64;
    static mut q2: libc::c_double = 0.531103462366f64;
    static mut p3: libc::c_double = -0.204231210125f64;
    static mut q3: libc::c_double = 0.103537752850f64;
    static mut p4: libc::c_double = -0.453642210148e-04f64;
    static mut q4: libc::c_double = 0.38560700634e-02f64;
    static mut c1: libc::c_double = 0.8832f64;
    static mut c2: libc::c_double = 0.2368f64;
    static mut c3: libc::c_double = 1.214f64;
    static mut c4: libc::c_double = 1.208f64;
    static mut c5: libc::c_double = 1.4142f64;
    static mut vmax: libc::c_double = 120.0f64;
    let mut ps: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut yi: libc::c_double = 0.;
    ps = 0.5f64 - 0.5f64 * p;
    yi = sqrt(log(1.0f64 / (ps * ps)));
    t = yi
        + ((((yi * p4 + p3) * yi + p2) * yi + p1) * yi + p0)
            / ((((yi * q4 + q3) * yi + q2) * yi + q1) * yi + q0);
    if v < vmax {
        t += (t * t * t + t) / v / 4.0f64;
    }
    q = c1 - c2 * t;
    if v < vmax {
        q += -c3 / v + c4 * t / v;
    }
    return t * (q * log(c - 1.0f64) + c5);
}
#[no_mangle]
pub unsafe extern "C" fn qtukey(
    mut p: libc::c_double,
    mut rr: libc::c_double,
    mut cc: libc::c_double,
    mut df: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut eps: libc::c_double = 0.0001f64;
    let maxiter: libc::c_int = 50 as libc::c_int;
    let mut ans: libc::c_double = 0.0f64;
    let mut valx0: libc::c_double = 0.;
    let mut valx1: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut xabs: libc::c_double = 0.;
    let mut iter: libc::c_int = 0;
    if p.is_nan() as i32 != 0 as libc::c_int
        || rr.is_nan() as i32 != 0 as libc::c_int
        || cc.is_nan() as i32 != 0 as libc::c_int
        || df.is_nan() as i32 != 0 as libc::c_int
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
            printf(msg, b"qtukey\0" as *const u8 as *const libc::c_char);
        }
        return p + rr + cc + df;
    }
    if df < 2 as libc::c_int as libc::c_double
        || rr < 1 as libc::c_int as libc::c_double
        || cc < 2 as libc::c_int as libc::c_double
    {
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
    x0 = qinv(p, cc, df);
    valx0 = ptukey(x0, rr, cc, df, TRUE as libc::c_int, FALSE as libc::c_int) - p;
    if valx0 > 0.0f64 {
        x1 = fmax2(0.0f64, x0 - 1.0f64);
    } else {
        x1 = x0 + 1.0f64;
    }
    valx1 = ptukey(x1, rr, cc, df, TRUE as libc::c_int, FALSE as libc::c_int) - p;
    iter = 1 as libc::c_int;
    while iter < maxiter {
        ans = x1 - valx1 * (x1 - x0) / (valx1 - valx0);
        valx0 = valx1;
        x0 = x1;
        if ans < 0.0f64 {
            ans = 0.0f64;
            valx1 = -p;
        }
        valx1 = ptukey(ans, rr, cc, df, TRUE as libc::c_int, FALSE as libc::c_int) - p;
        x1 = ans;
        xabs = fabs(x1 - x0);
        if xabs < eps {
            return ans;
        }
        iter += 1;
    }
    if 4 as libc::c_int > 1 as libc::c_int {
        let mut msg_3: *mut libc::c_char =
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        match 4 as libc::c_int {
            1 => {
                msg_3 = b"argument out of domain in '%s'\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            2 => {
                msg_3 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            4 => {
                msg_3 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            8 => {
                msg_3 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            16 => {
                msg_3 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {}
        }
        printf(msg_3, b"qtukey\0" as *const u8 as *const libc::c_char);
    }
    return ans;
}
