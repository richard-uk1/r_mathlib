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
    fn Rf_pgamma_raw(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dgamma(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pgamma(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn lgamma1p(_: libc::c_double) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn qchisq_appr(
    mut p: libc::c_double,
    mut nu: libc::c_double,
    mut g: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
    mut tol: libc::c_double,
) -> libc::c_double {
    let mut alpha: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut ch: libc::c_double = 0.;
    let mut p1: libc::c_double = 0.;
    let mut p2: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int || nu.is_nan() as i32 != 0 as libc::c_int {
        return p + nu;
    }
    if log_p != 0 && p > 0 as libc::c_int as libc::c_double
        || log_p == 0
            && (p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double)
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
    if nu <= 0 as libc::c_int as libc::c_double {
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
    alpha = 0.5f64 * nu;
    c = alpha - 1 as libc::c_int as libc::c_double;
    p1 = if lower_tail != 0 {
        if log_p != 0 {
            p
        } else {
            log(p)
        }
    } else {
        if log_p != 0 {
            if p > -0.693147180559945309417232121458f64 {
                log(-expm1(p))
            } else {
                Rlog1p(-exp(p))
            }
        } else {
            Rlog1p(-p)
        }
    };
    if nu < -1.24f64 * p1 {
        let mut lgam1pa: libc::c_double = if alpha < 0.5f64 {
            lgamma1p(alpha)
        } else {
            log(alpha) + g
        };
        ch = exp((lgam1pa + p1) / alpha + 0.693147180559945309417232121458f64);
    } else if nu > 0.32f64 {
        x = qnorm5(
            p,
            0 as libc::c_int as libc::c_double,
            1 as libc::c_int as libc::c_double,
            lower_tail,
            log_p,
        );
        p1 = 2.0f64 / (9 as libc::c_int as libc::c_double * nu);
        ch = nu
            * pow(
                x * sqrt(p1) + 1 as libc::c_int as libc::c_double - p1,
                3 as libc::c_int as libc::c_double,
            );
        if ch > 2.2f64 * nu + 6 as libc::c_int as libc::c_double {
            ch = -(2 as libc::c_int) as libc::c_double
                * ((if lower_tail != 0 {
                    if log_p != 0 {
                        if p > -0.693147180559945309417232121458f64 {
                            log(-expm1(p))
                        } else {
                            Rlog1p(-exp(p))
                        }
                    } else {
                        Rlog1p(-p)
                    }
                } else {
                    if log_p != 0 {
                        p
                    } else {
                        log(p)
                    }
                }) - c * log(0.5f64 * ch)
                    + g);
        }
    } else {
        ch = 0.4f64;
        a = (if lower_tail != 0 {
            if log_p != 0 {
                if p > -0.693147180559945309417232121458f64 {
                    log(-expm1(p))
                } else {
                    Rlog1p(-exp(p))
                }
            } else {
                Rlog1p(-p)
            }
        } else {
            if log_p != 0 {
                p
            } else {
                log(p)
            }
        }) + g
            + c * 0.693147180559945309417232121458f64;
        loop {
            q = ch;
            p1 = 1.0f64 / (1 as libc::c_int as libc::c_double + ch * (4.67f64 + ch));
            p2 = ch * (6.73f64 + ch * (6.66f64 + ch));
            t = -0.5f64 + (4.67f64 + 2 as libc::c_int as libc::c_double * ch) * p1
                - (6.73f64 + ch * (13.32f64 + 3 as libc::c_int as libc::c_double * ch)) / p2;
            ch -= (1 as libc::c_int as libc::c_double - exp(a + 0.5f64 * ch) * p2 * p1) / t;
            if !(fabs(q - ch) > tol * fabs(ch)) {
                break;
            }
        }
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn qgamma(
    mut p: libc::c_double,
    mut alpha: libc::c_double,
    mut scale: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut i420: libc::c_double = 1.0f64 / 420.0f64;
    static mut i2520: libc::c_double = 1.0f64 / 2520.0f64;
    static mut i5040: libc::c_double = 1.0f64 / 5040 as libc::c_int as libc::c_double;
    let mut p_: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut ch: libc::c_double = 0.;
    let mut ch0: libc::c_double = 0.;
    let mut p1: libc::c_double = 0.;
    let mut p2: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    let mut s2: libc::c_double = 0.;
    let mut s3: libc::c_double = 0.;
    let mut s4: libc::c_double = 0.;
    let mut s5: libc::c_double = 0.;
    let mut s6: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut max_it_Newton: libc::c_int = 1 as libc::c_int;
    if p.is_nan() as i32 != 0 as libc::c_int
        || alpha.is_nan() as i32 != 0 as libc::c_int
        || scale.is_nan() as i32 != 0 as libc::c_int
    {
        return p + alpha + scale;
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
                0.0f64
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                0.0f64
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
                0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                0.0f64
            };
        }
    }
    if alpha < 0 as libc::c_int as libc::c_double || scale <= 0 as libc::c_int as libc::c_double {
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
    if alpha == 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    }
    if alpha < 1e-10f64 {
        max_it_Newton = 7 as libc::c_int;
    }
    p_ = if log_p != 0 {
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
    g = lgammafn(alpha);
    ch = qchisq_appr(
        p,
        2 as libc::c_int as libc::c_double * alpha,
        g,
        lower_tail,
        log_p,
        1e-2f64,
    );
    if R_finite(ch) == 0 {
        max_it_Newton = 0 as libc::c_int;
    } else if ch < 5e-7f64 {
        max_it_Newton = 20 as libc::c_int;
    } else if p_ > 1 as libc::c_int as libc::c_double - 1e-14f64 || p_ < 1e-100f64 {
        max_it_Newton = 20 as libc::c_int;
    } else {
        c = alpha - 1 as libc::c_int as libc::c_double;
        s6 = (120 as libc::c_int as libc::c_double
            + c * (346 as libc::c_int as libc::c_double
                + 127 as libc::c_int as libc::c_double * c))
            * i5040;
        ch0 = ch;
        i = 1 as libc::c_int;
        while i <= 1000 as libc::c_int {
            q = ch;
            p1 = 0.5f64 * ch;
            p2 = p_ - Rf_pgamma_raw(p1, alpha, TRUE as libc::c_int, FALSE as libc::c_int);
            if R_finite(p2) == 0 || ch <= 0 as libc::c_int as libc::c_double {
                ch = ch0;
                max_it_Newton = 27 as libc::c_int;
                break;
            } else {
                t = p2 * exp(alpha * 0.693147180559945309417232121458f64 + g + p1 - c * log(ch));
                b = t / ch;
                a = 0.5f64 * t - b * c;
                s1 = (210 as libc::c_int as libc::c_double
                    + a * (140 as libc::c_int as libc::c_double
                        + a * (105 as libc::c_int as libc::c_double
                            + a * (84 as libc::c_int as libc::c_double
                                + a * (70 as libc::c_int as libc::c_double
                                    + 60 as libc::c_int as libc::c_double * a)))))
                    * i420;
                s2 = (420 as libc::c_int as libc::c_double
                    + a * (735 as libc::c_int as libc::c_double
                        + a * (966 as libc::c_int as libc::c_double
                            + a * (1141 as libc::c_int as libc::c_double
                                + 1278 as libc::c_int as libc::c_double * a))))
                    * i2520;
                s3 = (210 as libc::c_int as libc::c_double
                    + a * (462 as libc::c_int as libc::c_double
                        + a * (707 as libc::c_int as libc::c_double
                            + 932 as libc::c_int as libc::c_double * a)))
                    * i2520;
                s4 = (252 as libc::c_int as libc::c_double
                    + a * (672 as libc::c_int as libc::c_double
                        + 1182 as libc::c_int as libc::c_double * a)
                    + c * (294 as libc::c_int as libc::c_double
                        + a * (889 as libc::c_int as libc::c_double
                            + 1740 as libc::c_int as libc::c_double * a)))
                    * i5040;
                s5 = (84 as libc::c_int as libc::c_double
                    + 2264 as libc::c_int as libc::c_double * a
                    + c * (1175 as libc::c_int as libc::c_double
                        + 606 as libc::c_int as libc::c_double * a))
                    * i2520;
                ch += t
                    * (1 as libc::c_int as libc::c_double + 0.5f64 * t * s1
                        - b * c * (s1 - b * (s2 - b * (s3 - b * (s4 - b * (s5 - b * s6))))));
                if fabs(q - ch) < 5e-7f64 * ch {
                    break;
                }
                if fabs(q - ch) > 0.1f64 * ch {
                    if ch < q {
                        ch = 0.9f64 * q;
                    } else {
                        ch = 1.1f64 * q;
                    }
                }
                i += 1;
            }
        }
    }
    x = 0.5f64 * scale * ch;
    if max_it_Newton != 0 {
        if log_p == 0 {
            p = log(p);
            log_p = TRUE as libc::c_int;
        }
        if x == 0 as libc::c_int as libc::c_double {
            let _1_p: libc::c_double = 1.0f64 + 1e-7f64;
            let _1_m: libc::c_double = 1.0f64 - 1e-7f64;
            x = 2.2250738585072014e-308f64;
            p_ = pgamma(x, alpha, scale, lower_tail, log_p);
            if lower_tail != 0 && p_ > p * _1_p || lower_tail == 0 && p_ < p * _1_m {
                return 0.0f64;
            }
        } else {
            p_ = pgamma(x, alpha, scale, lower_tail, log_p);
        }
        if p_ == -1.0f64 / 0.0f64 {
            return 0 as libc::c_int as libc::c_double;
        }
        i = 1 as libc::c_int;
        while i <= max_it_Newton {
            p1 = p_ - p;
            if fabs(p1) < fabs(1e-15f64 * p) {
                break;
            }
            g = dgamma(x, alpha, scale, log_p);
            if g == (if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 }) {
                break;
            }
            t = if log_p != 0 { p1 * exp(p_ - g) } else { p1 / g };
            t = if lower_tail != 0 { x - t } else { x + t };
            p_ = pgamma(t, alpha, scale, lower_tail, log_p);
            if fabs(p_ - p) > fabs(p1) || i > 1 as libc::c_int && fabs(p_ - p) == fabs(p1) {
                break;
            } else {
                x = t;
                i += 1;
            }
        }
    }
    return x;
}
