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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn qnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dt(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn pt(_: libc::c_double, _: libc::c_double, _: libc::c_int, _: libc::c_int) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn tanpi(_: libc::c_double) -> libc::c_double;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
#[no_mangle]
pub unsafe extern "C" fn qt(
    mut p: libc::c_double,
    mut ndf: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut eps: libc::c_double = 1.0e-12f64;
    let mut P: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int || ndf.is_nan() as i32 != 0 as libc::c_int {
        return p + ndf;
    }
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
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
                            as *const libc::c_char
                            as *mut libc::c_char;
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
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_0: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
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
                printf(msg_0, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
    }
    if ndf <= 0 as libc::c_int as libc::c_double {
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
    if ndf < 1 as libc::c_int as libc::c_double {
        static mut accu: libc::c_double = 1e-13f64;
        static mut Eps: libc::c_double = 1e-11f64;
        let mut ux: libc::c_double = 0.;
        let mut lx: libc::c_double = 0.;
        let mut nx: libc::c_double = 0.;
        let mut pp: libc::c_double = 0.;
        let mut iter: libc::c_int = 0 as libc::c_int;
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
            return 1.0f64 / 0.0f64;
        }
        pp = fmin2(
            1 as libc::c_int as libc::c_double - 2.2204460492503131e-16f64,
            p * (1 as libc::c_int as libc::c_double + Eps),
        );
        ux = 1.0f64;
        while ux < 1.7976931348623157e+308f64
            && pt(ux, ndf, TRUE as libc::c_int, FALSE as libc::c_int) < pp
        {
            ux *= 2 as libc::c_int as libc::c_double;
        }
        pp = p * (1 as libc::c_int as libc::c_double - Eps);
        lx = -1.0f64;
        while lx > -1.7976931348623157e+308f64
            && pt(lx, ndf, TRUE as libc::c_int, FALSE as libc::c_int) > pp
        {
            lx *= 2 as libc::c_int as libc::c_double;
        }
        loop {
            nx = 0.5f64 * (lx + ux);
            if pt(nx, ndf, TRUE as libc::c_int, FALSE as libc::c_int) > p {
                ux = nx;
            } else {
                lx = nx;
            }
            if !((ux - lx) / fabs(nx) > accu && {
                iter += 1;
                iter < 1000 as libc::c_int
            }) {
                break;
            }
        }
        if iter >= 1000 as libc::c_int {
            if 8 as libc::c_int > 1 as libc::c_int {
                let mut msg_2: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 8 as libc::c_int {
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
                printf(msg_2, b"qt\0" as *const u8 as *const libc::c_char);
            }
        }
        return 0.5f64 * (lx + ux);
    }
    if ndf > 1e20f64 {
        return qnorm5(p, 0.0f64, 1.0f64, lower_tail, log_p);
    }
    P = if log_p != 0 { exp(p) } else { p };
    let mut neg: Rboolean = ((lower_tail == 0 || P < 0.5f64) && (lower_tail != 0 || P > 0.5f64))
        as libc::c_int as Rboolean;
    let mut is_neg_lower: Rboolean =
        (lower_tail as libc::c_uint == neg as libc::c_uint) as libc::c_int as Rboolean;
    if neg as u64 != 0 {
        P = 2 as libc::c_int as libc::c_double
            * (if log_p != 0 {
                if lower_tail != 0 {
                    P
                } else {
                    -expm1(p)
                }
            } else {
                if lower_tail != 0 {
                    p
                } else {
                    0.5f64 - p + 0.5f64
                }
            });
    } else {
        P = 2 as libc::c_int as libc::c_double
            * (if log_p != 0 {
                if lower_tail != 0 {
                    -expm1(p)
                } else {
                    P
                }
            } else {
                if lower_tail != 0 {
                    0.5f64 - p + 0.5f64
                } else {
                    p
                }
            });
    }
    if fabs(ndf - 2 as libc::c_int as libc::c_double) < eps {
        if P > 2.2250738585072014e-308f64 {
            if 3 as libc::c_int as libc::c_double * P < 2.2204460492503131e-16f64 {
                q = 1 as libc::c_int as libc::c_double / sqrt(P);
            } else if P > 0.9f64 {
                q = (1 as libc::c_int as libc::c_double - P)
                    * sqrt(
                        2 as libc::c_int as libc::c_double
                            / (P * (2 as libc::c_int as libc::c_double - P)),
                    );
            } else {
                q = sqrt(
                    2 as libc::c_int as libc::c_double
                        / (P * (2 as libc::c_int as libc::c_double - P))
                        - 2 as libc::c_int as libc::c_double,
                );
            }
        } else if log_p != 0 {
            q = if is_neg_lower as libc::c_uint != 0 {
                exp(-p / 2 as libc::c_int as libc::c_double) / 1.414213562373095048801688724210f64
            } else {
                1 as libc::c_int as libc::c_double / sqrt(-expm1(p))
            };
        } else {
            q = 1.0f64 / 0.0f64;
        }
    } else if ndf < 1 as libc::c_int as libc::c_double + eps {
        if P == 1.0f64 {
            q = 0 as libc::c_int as libc::c_double;
        } else if P > 0 as libc::c_int as libc::c_double {
            q = 1 as libc::c_int as libc::c_double / tanpi(P / 2.0f64);
        } else if log_p != 0 {
            q = if is_neg_lower as libc::c_uint != 0 {
                0.318309886183790671537767526745f64 * exp(-p)
            } else {
                -1.0f64 / (3.141592653589793238462643383280f64 * expm1(p))
            };
        } else {
            q = 1.0f64 / 0.0f64;
        }
    } else {
        let mut x: libc::c_double = 0.0f64;
        let mut y: libc::c_double = 0.;
        let mut log_P2: libc::c_double = 0.0f64;
        let mut a: libc::c_double = 1 as libc::c_int as libc::c_double / (ndf - 0.5f64);
        let mut b: libc::c_double = 48 as libc::c_int as libc::c_double / (a * a);
        let mut c: libc::c_double = ((20700 as libc::c_int as libc::c_double * a / b
            - 98 as libc::c_int as libc::c_double)
            * a
            - 16 as libc::c_int as libc::c_double)
            * a
            + 96.36f64;
        let mut d: libc::c_double = ((94.5f64 / (b + c) - 3 as libc::c_int as libc::c_double) / b
            + 1 as libc::c_int as libc::c_double)
            * sqrt(a * 1.570796326794896619231321691640f64)
            * ndf;
        let mut P_ok1: Rboolean =
            (P > 2.2250738585072014e-308f64 || log_p == 0) as libc::c_int as Rboolean;
        let mut P_ok: Rboolean = P_ok1;
        if P_ok1 as u64 != 0 {
            y = pow(d * P, 2.0f64 / ndf);
            P_ok = (y >= 2.2204460492503131e-16f64) as libc::c_int as Rboolean;
        }
        if P_ok as u64 == 0 {
            log_P2 = if is_neg_lower as libc::c_uint != 0 {
                if log_p != 0 {
                    p
                } else {
                    log(p)
                }
            } else if log_p != 0 {
                if p > -0.693147180559945309417232121458f64 {
                    log(-expm1(p))
                } else {
                    Rlog1p(-exp(p))
                }
            } else {
                Rlog1p(-p)
            };
            x = (log(d) + 0.693147180559945309417232121458f64 + log_P2) / ndf;
            y = exp(2 as libc::c_int as libc::c_double * x);
        }
        if ndf < 2.1f64 && P > 0.5f64 || y > 0.05f64 + a {
            if P_ok as u64 != 0 {
                x = qnorm5(
                    0.5f64 * P,
                    0.0f64,
                    1.0f64,
                    TRUE as libc::c_int,
                    FALSE as libc::c_int,
                );
            } else {
                x = qnorm5(log_P2, 0.0f64, 1.0f64, lower_tail, TRUE as libc::c_int);
            }
            y = x * x;
            if ndf < 5 as libc::c_int as libc::c_double {
                c += 0.3f64 * (ndf - 4.5f64) * (x + 0.6f64);
            }
            c = (((0.05f64 * d * x - 5 as libc::c_int as libc::c_double) * x
                - 7 as libc::c_int as libc::c_double)
                * x
                - 2 as libc::c_int as libc::c_double)
                * x
                + b
                + c;
            y = (((((0.4f64 * y + 6.3f64) * y + 36 as libc::c_int as libc::c_double) * y
                + 94.5f64)
                / c
                - y
                - 3 as libc::c_int as libc::c_double)
                / b
                + 1 as libc::c_int as libc::c_double)
                * x;
            y = expm1(a * y * y);
            q = sqrt(ndf * y);
        } else if P_ok as u64 == 0
            && x < -0.693147180559945309417232121458f64 * 53 as libc::c_int as libc::c_double
        {
            q = sqrt(ndf) * exp(-x);
        } else {
            y = ((1 as libc::c_int as libc::c_double
                / (((ndf + 6 as libc::c_int as libc::c_double) / (ndf * y)
                    - 0.089f64 * d
                    - 0.822f64)
                    * (ndf + 2 as libc::c_int as libc::c_double)
                    * 3 as libc::c_int as libc::c_double)
                + 0.5f64 / (ndf + 4 as libc::c_int as libc::c_double))
                * y
                - 1 as libc::c_int as libc::c_double)
                * (ndf + 1 as libc::c_int as libc::c_double)
                / (ndf + 2 as libc::c_int as libc::c_double)
                + 1 as libc::c_int as libc::c_double / y;
            q = sqrt(ndf * y);
        }
        if P_ok1 as u64 != 0 {
            let mut it: libc::c_int = 0 as libc::c_int;
            loop {
                let fresh0 = it;
                it = it + 1;
                if !(fresh0 < 10 as libc::c_int
                    && {
                        y = dt(q, ndf, FALSE as libc::c_int);
                        y > 0 as libc::c_int as libc::c_double
                    }
                    && {
                        x = (pt(q, ndf, FALSE as libc::c_int, FALSE as libc::c_int)
                            - P / 2 as libc::c_int as libc::c_double)
                            / y;
                        R_finite(x) != 0
                    }
                    && fabs(x) > 1e-14f64 * fabs(q))
                {
                    break;
                }
                q += x
                    * (1.0f64
                        + x * q * (ndf + 1 as libc::c_int as libc::c_double)
                            / (2 as libc::c_int as libc::c_double * (q * q + ndf)));
            }
        }
    }
    if neg as u64 != 0 {
        q = -q;
    }
    return q;
}
