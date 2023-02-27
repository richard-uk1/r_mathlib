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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn R_pow_di(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn pbeta_raw(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn lbeta(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
#[no_mangle]
pub unsafe extern "C" fn qbeta(
    mut alpha: libc::c_double,
    mut p: libc::c_double,
    mut q: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if p.is_nan() as i32 != 0 as libc::c_int
        || q.is_nan() as i32 != 0 as libc::c_int
        || alpha.is_nan() as i32 != 0 as libc::c_int
    {
        return p + q + alpha;
    }
    if p < 0.0f64 || q < 0.0f64 {
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
    let mut qbet: [libc::c_double; 2] = [0.; 2];
    qbeta_raw(
        alpha,
        p,
        q,
        lower_tail,
        log_p,
        -(1 as libc::c_int),
        -5.0f64,
        4 as libc::c_int,
        qbet.as_mut_ptr(),
    );
    return qbet[0 as libc::c_int as usize];
}
static mut DBL_very_MIN: libc::c_double = 2.2250738585072014e-308f64 / 4.0f64;
static mut DBL_log_v_MIN: libc::c_double = 0.693147180559945309417232121458f64
    * (-(1021 as libc::c_int) - 2 as libc::c_int) as libc::c_double;
static mut DBL_1__eps: libc::c_double = 0.9999999999999999f64;
unsafe extern "C" fn qbeta_raw(
    mut alpha: libc::c_double,
    mut p: libc::c_double,
    mut q: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
    mut swap_01: libc::c_int,
    mut log_q_cut: libc::c_double,
    mut n_N: libc::c_int,
    mut qb: *mut libc::c_double,
) {
    let mut bad_u: Rboolean = FALSE;
    let mut bad_init: Rboolean = FALSE;
    let mut u_n: libc::c_double = 0.;
    let mut wprev: libc::c_double = 0.;
    let mut prev: libc::c_double = 0.;
    let mut adj: libc::c_double = 0.;
    let mut current_block: u64;
    let mut swap_choose: Rboolean = (swap_01 == -(1 as libc::c_int)) as libc::c_int as Rboolean;
    let mut swap_tail: Rboolean = FALSE;
    let mut log_: Rboolean = FALSE;
    let mut give_log_q: Rboolean = (log_q_cut == 1.0f64 / 0.0f64) as libc::c_int as Rboolean;
    let mut use_log_x: Rboolean = give_log_q;
    let mut warned: Rboolean = FALSE;
    let mut add_N_step: Rboolean = TRUE;
    let mut i_pb: libc::c_int = 0;
    let mut i_inn: libc::c_int = 0;
    let mut a: libc::c_double = 0.;
    let mut la: libc::c_double = 0.;
    let mut logbeta: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut pp: libc::c_double = 0.;
    let mut p_: libc::c_double = 0.;
    let mut qq: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut y: libc::c_double = -1.0f64;
    let mut u: libc::c_double = 0.;
    let mut xinbta: libc::c_double = 0.;
    if alpha
        == (if lower_tail != 0 {
            if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 }
        } else {
            if log_p != 0 { 0.0f64 } else { 1.0f64 }
        })
    {
        if give_log_q as u64 != 0 {
            *qb.offset(0 as libc::c_int as isize) = -1.0f64 / 0.0f64;
            *qb.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        } else {
            *qb.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *qb.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
        }
        return;
    }
    if alpha
        == (if lower_tail != 0 {
            if log_p != 0 { 0.0f64 } else { 1.0f64 }
        } else {
            if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 }
        })
    {
        if give_log_q as u64 != 0 {
            *qb.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
            *qb.offset(1 as libc::c_int as isize) = -1.0f64 / 0.0f64;
        } else {
            *qb.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
            *qb.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
        }
        return;
    }
    if log_p != 0 && alpha > 0 as libc::c_int as libc::c_double
        || log_p == 0
            && (alpha < 0 as libc::c_int as libc::c_double
                || alpha > 1 as libc::c_int as libc::c_double)
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
        let ref mut fresh0 = *qb.offset(1 as libc::c_int as isize);
        *fresh0 = 0.0f64 / 0.0f64;
        *qb.offset(0 as libc::c_int as isize) = *fresh0;
        return;
    }
    if p == 0 as libc::c_int as libc::c_double
        || q == 0 as libc::c_int as libc::c_double
        || R_finite(p) == 0
        || R_finite(q) == 0
    {
        if p == 0 as libc::c_int as libc::c_double && q == 0 as libc::c_int as libc::c_double {
            if alpha
                < (if log_p != 0 {
                    -0.693147180559945309417232121458f64
                } else {
                    0.5f64
                })
            {
                if give_log_q as u64 != 0 {
                    *qb.offset(0 as libc::c_int as isize) = -1.0f64 / 0.0f64;
                    *qb.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                } else {
                    *qb.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                    *qb.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
                }
                return;
            }
            if alpha
                > (if log_p != 0 {
                    -0.693147180559945309417232121458f64
                } else {
                    0.5f64
                })
            {
                if give_log_q as u64 != 0 {
                    *qb.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                    *qb.offset(1 as libc::c_int as isize) = -1.0f64 / 0.0f64;
                } else {
                    *qb.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
                    *qb.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                }
                return;
            }
            if give_log_q as u64 != 0 {
                let ref mut fresh1 = *qb.offset(1 as libc::c_int as isize);
                *fresh1 = -0.693147180559945309417232121458f64;
                *qb.offset(0 as libc::c_int as isize) = *fresh1;
            } else {
                let ref mut fresh2 = *qb.offset(1 as libc::c_int as isize);
                *fresh2 = 0.5f64;
                *qb.offset(0 as libc::c_int as isize) = *fresh2;
            }
            return;
        } else {
            if p == 0 as libc::c_int as libc::c_double
                || p / q == 0 as libc::c_int as libc::c_double
            {
                if give_log_q as u64 != 0 {
                    *qb.offset(0 as libc::c_int as isize) = -1.0f64 / 0.0f64;
                    *qb.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                } else {
                    *qb.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                    *qb.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
                }
                return;
            } else {
                if q == 0 as libc::c_int as libc::c_double
                    || q / p == 0 as libc::c_int as libc::c_double
                {
                    if give_log_q as u64 != 0 {
                        *qb.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                        *qb.offset(1 as libc::c_int as isize) = -1.0f64 / 0.0f64;
                    } else {
                        *qb.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double;
                        *qb.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
                    }
                    return;
                }
            }
        }
        if give_log_q as u64 != 0 {
            let ref mut fresh3 = *qb.offset(1 as libc::c_int as isize);
            *fresh3 = -0.693147180559945309417232121458f64;
            *qb.offset(0 as libc::c_int as isize) = *fresh3;
        } else {
            let ref mut fresh4 = *qb.offset(1 as libc::c_int as isize);
            *fresh4 = 0.5f64;
            *qb.offset(0 as libc::c_int as isize) = *fresh4;
        }
        return;
    }
    p_ = if log_p != 0 {
        if lower_tail != 0 {
            exp(alpha)
        } else {
            -expm1(alpha)
        }
    } else if lower_tail != 0 {
        alpha
    } else {
        0.5f64 - alpha + 0.5f64
    };
    logbeta = lbeta(p, q);
    swap_tail = (if swap_choose as libc::c_uint != 0 {
        (p_ > 0.5f64) as libc::c_int
    } else {
        swap_01
    }) as Rboolean;
    if swap_tail as u64 != 0 {
        a = if log_p != 0 {
            if lower_tail != 0 {
                -expm1(alpha)
            } else {
                exp(alpha)
            }
        } else if lower_tail != 0 {
            0.5f64 - alpha + 0.5f64
        } else {
            alpha
        };
        la = if lower_tail != 0 {
            if log_p != 0 {
                if alpha > -0.693147180559945309417232121458f64 {
                    log(-expm1(alpha))
                } else {
                    Rlog1p(-exp(alpha))
                }
            } else {
                Rlog1p(-alpha)
            }
        } else if log_p != 0 {
            alpha
        } else {
            log(alpha)
        };
        pp = q;
        qq = p;
    } else {
        a = p_;
        la = if lower_tail != 0 {
            if log_p != 0 {
                alpha
            } else {
                log(alpha)
            }
        } else if log_p != 0 {
            if alpha > -0.693147180559945309417232121458f64 {
                log(-expm1(alpha))
            } else {
                Rlog1p(-exp(alpha))
            }
        } else {
            Rlog1p(-alpha)
        };
        pp = p;
        qq = q;
    }
    let mut acu: libc::c_double = fmax2(
        1e-300f64,
        pow(10.0f64, -13.0f64 - 2.5f64 / (pp * pp) - 0.5f64 / (a * a)),
    );
    let mut tx: libc::c_double = 0.;
    let mut u0: libc::c_double = (la + log(pp) + logbeta) / pp;
    static mut log_eps_c: libc::c_double =
        0.693147180559945309417232121458f64 * (1.0f64 - 53 as libc::c_int as libc::c_double);
    r = pp * (1.0f64 - qq) / (pp + 1.0f64);
    t = 0.2f64;
    if (0.693147180559945309417232121458f64 * -(1021 as libc::c_int) as libc::c_double) < u0
        && u0 < -0.01f64
        && u0
            < (t * log_eps_c
                - log(fabs(
                    pp * (1.0f64 - qq) * (2.0f64 - qq) / (2.0f64 * (pp + 2.0f64)),
                )))
                / 2.0f64
    {
        r = r * exp(u0);
        if r > -1.0f64 {
            ::core::ptr::write_volatile(&mut u as *mut libc::c_double, u0 - Rlog1p(r) / pp);
        } else {
            ::core::ptr::write_volatile(&mut u as *mut libc::c_double, u0);
        }
        ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, exp(u));
        tx = ::core::ptr::read_volatile::<libc::c_double>(&xinbta as *const libc::c_double);
        use_log_x = TRUE;
        current_block = 1655995746202251676;
    } else {
        r = sqrt(-(2 as libc::c_int) as libc::c_double * la);
        y = r - (2.30753f64 + 0.27061f64 * r) / (1.0f64 + (0.99229f64 + 0.04481f64 * r) * r);
        if pp > 1 as libc::c_int as libc::c_double && qq > 1 as libc::c_int as libc::c_double {
            r = (y * y - 3.0f64) / 6.0f64;
            s = 1.0f64 / (pp + pp - 1.0f64);
            t = 1.0f64 / (qq + qq - 1.0f64);
            h = 2.0f64 / (s + t);
            w = y * sqrt(h + r) / h - (t - s) * (r + 5.0f64 / 6.0f64 - 2.0f64 / (3.0f64 * h));
            if w > 300 as libc::c_int as libc::c_double {
                t = w + w + log(qq) - log(pp);
                ::core::ptr::write_volatile(
                    &mut u as *mut libc::c_double,
                    if t <= 18 as libc::c_int as libc::c_double {
                        -Rlog1p(exp(t))
                    } else {
                        -t - exp(-t)
                    },
                );
                ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, exp(u));
            } else {
                ::core::ptr::write_volatile(
                    &mut xinbta as *mut libc::c_double,
                    pp / (pp + qq * exp(w + w)),
                );
                ::core::ptr::write_volatile(
                    &mut u as *mut libc::c_double,
                    -Rlog1p(qq / pp * exp(w + w)),
                );
            }
        } else {
            r = qq + qq;
            t = 1.0f64 / (3.0f64 * sqrt(qq));
            t = r * R_pow_di(1.0f64 + t * (-t + y), 3 as libc::c_int);
            s = 4.0f64 * pp + r - 2.0f64;
            if t == 0 as libc::c_int as libc::c_double || t < 0.0f64 && s >= t {
                let mut l1ma: libc::c_double = 0.;
                if swap_tail as u64 != 0 {
                    l1ma = if lower_tail != 0 {
                        if log_p != 0 {
                            alpha
                        } else {
                            log(alpha)
                        }
                    } else if log_p != 0 {
                        if alpha > -0.693147180559945309417232121458f64 {
                            log(-expm1(alpha))
                        } else {
                            Rlog1p(-exp(alpha))
                        }
                    } else {
                        Rlog1p(-alpha)
                    };
                } else {
                    l1ma = if lower_tail != 0 {
                        if log_p != 0 {
                            if alpha > -0.693147180559945309417232121458f64 {
                                log(-expm1(alpha))
                            } else {
                                Rlog1p(-exp(alpha))
                            }
                        } else {
                            Rlog1p(-alpha)
                        }
                    } else if log_p != 0 {
                        alpha
                    } else {
                        log(alpha)
                    };
                }
                let mut xx: libc::c_double = (l1ma + log(qq) + logbeta) / qq;
                if xx <= 0.0f64 {
                    ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, -expm1(xx));
                    ::core::ptr::write_volatile(
                        &mut u as *mut libc::c_double,
                        if xx > -0.693147180559945309417232121458f64 {
                            log(-expm1(xx))
                        } else {
                            Rlog1p(-exp(xx))
                        },
                    );
                } else {
                    ::core::ptr::write_volatile(
                        &mut xinbta as *mut libc::c_double,
                        0 as libc::c_int as libc::c_double,
                    );
                    ::core::ptr::write_volatile(&mut u as *mut libc::c_double, -1.0f64 / 0.0f64);
                }
            } else {
                t = s / t;
                if t <= 1.0f64 {
                    ::core::ptr::write_volatile(
                        &mut u as *mut libc::c_double,
                        (la + log(pp) + logbeta) / pp,
                    );
                    ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, exp(u));
                } else {
                    ::core::ptr::write_volatile(
                        &mut xinbta as *mut libc::c_double,
                        1.0f64 - 2.0f64 / (t + 1.0f64),
                    );
                    ::core::ptr::write_volatile(
                        &mut u as *mut libc::c_double,
                        Rlog1p(-2.0f64 / (t + 1.0f64)),
                    );
                }
            }
        }
        if swap_choose as libc::c_uint != 0
            && (swap_tail as libc::c_uint != 0 && u >= -exp(log_q_cut)
                || swap_tail as u64 == 0
                    && u >= -exp(4 as libc::c_int as libc::c_double * log_q_cut)
                    && pp / qq < 1000.0f64)
        {
            swap_tail = (swap_tail as u64 == 0) as libc::c_int as Rboolean;
            if swap_tail as u64 != 0 {
                a = if log_p != 0 {
                    if lower_tail != 0 {
                        -expm1(alpha)
                    } else {
                        exp(alpha)
                    }
                } else if lower_tail != 0 {
                    0.5f64 - alpha + 0.5f64
                } else {
                    alpha
                };
                la = if lower_tail != 0 {
                    if log_p != 0 {
                        if alpha > -0.693147180559945309417232121458f64 {
                            log(-expm1(alpha))
                        } else {
                            Rlog1p(-exp(alpha))
                        }
                    } else {
                        Rlog1p(-alpha)
                    }
                } else if log_p != 0 {
                    alpha
                } else {
                    log(alpha)
                };
                pp = q;
                qq = p;
            } else {
                a = p_;
                la = if lower_tail != 0 {
                    if log_p != 0 {
                        alpha
                    } else {
                        log(alpha)
                    }
                } else if log_p != 0 {
                    if alpha > -0.693147180559945309417232121458f64 {
                        log(-expm1(alpha))
                    } else {
                        Rlog1p(-exp(alpha))
                    }
                } else {
                    Rlog1p(-alpha)
                };
                pp = p;
                qq = q;
            }
            ::core::ptr::write_volatile(
                &mut u as *mut libc::c_double,
                if u > -0.693147180559945309417232121458f64 {
                    log(-expm1(u))
                } else {
                    Rlog1p(-exp(u))
                },
            );
            ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, exp(u));
        }
        if use_log_x as u64 == 0 {
            use_log_x = (u < log_q_cut) as libc::c_int as Rboolean;
        }
        bad_u = (R_finite(u) == 0) as libc::c_int as Rboolean;
        bad_init = (bad_u as libc::c_uint != 0
            || xinbta > 1 as libc::c_int as libc::c_double - 2.22e-16f64)
            as libc::c_int as Rboolean;
        u_n = 1.0f64;
        tx = xinbta;
        if bad_u as libc::c_uint != 0 || u < log_q_cut {
            w = pbeta_raw(DBL_very_MIN, pp, qq, TRUE as libc::c_int, log_p);
            if w > (if log_p != 0 { la } else { a }) {
                if log_p != 0 || fabs(w - a) < fabs(0 as libc::c_int as libc::c_double - a) {
                    tx = DBL_very_MIN;
                    u_n = DBL_log_v_MIN;
                } else {
                    tx = 0.0f64;
                    u_n = -1.0f64 / 0.0f64;
                }
                use_log_x = log_p as Rboolean;
                add_N_step = FALSE;
                current_block = 8914367006305986581;
            } else {
                if u < DBL_log_v_MIN {
                    ::core::ptr::write_volatile(&mut u as *mut libc::c_double, DBL_log_v_MIN);
                    ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, DBL_very_MIN);
                }
                current_block = 12129449210080749085;
            }
        } else {
            current_block = 12129449210080749085;
        }
        match current_block {
            8914367006305986581 => {}
            _ => {
                if bad_init as libc::c_uint != 0
                    && !(use_log_x as libc::c_uint != 0 && tx > 0 as libc::c_int as libc::c_double)
                {
                    if u == -1.0f64 / 0.0f64 {
                        ::core::ptr::write_volatile(
                            &mut u as *mut libc::c_double,
                            0.693147180559945309417232121458f64
                                * -(1021 as libc::c_int) as libc::c_double,
                        );
                        ::core::ptr::write_volatile(
                            &mut xinbta as *mut libc::c_double,
                            2.2250738585072014e-308f64,
                        );
                    } else {
                        ::core::ptr::write_volatile(
                            &mut xinbta as *mut libc::c_double,
                            if xinbta > 1.1f64 {
                                0.5f64
                            } else if xinbta < 3e-308f64 {
                                exp(u)
                            } else {
                                1 as libc::c_int as libc::c_double - 2.22e-16f64
                            },
                        );
                        if bad_u as u64 != 0 {
                            ::core::ptr::write_volatile(&mut u as *mut libc::c_double, log(xinbta));
                        }
                    }
                }
                current_block = 1655995746202251676;
            }
        }
    }
    match current_block {
        1655995746202251676 => {
            r = 1 as libc::c_int as libc::c_double - pp;
            t = 1 as libc::c_int as libc::c_double - qq;
            wprev = 0.0f64;
            prev = 1.0f64;
            adj = 1.0f64;
            if use_log_x as u64 != 0 {
                i_pb = 0 as libc::c_int;
                's_855: loop {
                    if !(i_pb < 1000 as libc::c_int) {
                        current_block = 2455751414656213165;
                        break;
                    }
                    y = pbeta_raw(xinbta, pp, qq, TRUE as libc::c_int, TRUE as libc::c_int);
                    w = if y == -1.0f64 / 0.0f64 {
                        0.0f64
                    } else {
                        (y - la)
                            * exp(y - u
                                + logbeta
                                + r * u
                                + t * (if u > -0.693147180559945309417232121458f64 {
                                    log(-expm1(u))
                                } else {
                                    Rlog1p(-exp(u))
                                }))
                    };
                    if R_finite(w) == 0 {
                        current_block = 2455751414656213165;
                        break;
                    }
                    if i_pb >= n_N && w * wprev <= 0.0f64 {
                        prev = fmax2(fabs(adj), 3e-308f64);
                    }
                    g = 1 as libc::c_int as libc::c_double;
                    i_inn = 0 as libc::c_int;
                    while i_inn < 1000 as libc::c_int {
                        adj = g * w;
                        if fabs(adj) < prev {
                            u_n = u - adj;
                            if u_n <= 0.0f64 {
                                if prev <= acu || fabs(w) <= acu {
                                    current_block = 18252426801853021985;
                                    break 's_855;
                                }
                                break;
                            }
                        }
                        g /= 3 as libc::c_int as libc::c_double;
                        i_inn += 1;
                    }
                    let mut D: libc::c_double = fmin2(fabs(adj), fabs(u_n - u));
                    if D <= 4e-16f64 * fabs(u_n + u) {
                        current_block = 18252426801853021985;
                        break;
                    }
                    ::core::ptr::write_volatile(&mut u as *mut libc::c_double, u_n);
                    ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, exp(u));
                    wprev = w;
                    i_pb += 1;
                }
            } else {
                i_pb = 0 as libc::c_int;
                's_961: loop {
                    if !(i_pb < 1000 as libc::c_int) {
                        current_block = 2455751414656213165;
                        break;
                    }
                    y = pbeta_raw(xinbta, pp, qq, TRUE as libc::c_int, log_p);
                    if R_finite(y) == 0 && !(log_p != 0 && y == -1.0f64 / 0.0f64) {
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
                                        as *const u8
                                        as *const libc::c_char
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
                        let ref mut fresh5 = *qb.offset(1 as libc::c_int as isize);
                        *fresh5 = 0.0f64 / 0.0f64;
                        *qb.offset(0 as libc::c_int as isize) = *fresh5;
                        return;
                    }
                    w = if log_p != 0 {
                        (y - la) * exp(y + logbeta + r * log(xinbta) + t * Rlog1p(-xinbta))
                    } else {
                        (y - a) * exp(logbeta + r * log(xinbta) + t * Rlog1p(-xinbta))
                    };
                    if i_pb >= n_N && w * wprev <= 0.0f64 {
                        prev = fmax2(fabs(adj), 3e-308f64);
                    }
                    g = 1 as libc::c_int as libc::c_double;
                    i_inn = 0 as libc::c_int;
                    while i_inn < 1000 as libc::c_int {
                        adj = g * w;
                        if i_pb < n_N || fabs(adj) < prev {
                            tx = xinbta - adj;
                            if 0.0f64 <= tx && tx <= 1.0f64 {
                                if prev <= acu || fabs(w) <= acu {
                                    current_block = 18252426801853021985;
                                    break 's_961;
                                }
                                if tx != 0.0f64 && tx != 1 as libc::c_int as libc::c_double {
                                    break;
                                }
                            }
                        }
                        g /= 3 as libc::c_int as libc::c_double;
                        i_inn += 1;
                    }
                    if fabs(tx - xinbta) <= 4e-16f64 * (tx + xinbta) {
                        current_block = 18252426801853021985;
                        break;
                    } else {
                        ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, tx);
                        if tx == 0 as libc::c_int as libc::c_double {
                            current_block = 2455751414656213165;
                            break;
                        }
                        wprev = w;
                        i_pb += 1;
                    }
                }
            }
            match current_block {
                2455751414656213165 => {
                    warned = TRUE;
                    if 8 as libc::c_int > 1 as libc::c_int {
                        let mut msg_1: *mut libc::c_char =
                            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                        match 8 as libc::c_int {
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
                                    as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            16 => {
                                msg_1 = b"underflow occurred in '%s'\n\0" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            _ => {}
                        }
                        printf(msg_1, b"qbeta\0" as *const u8 as *const libc::c_char);
                    }
                }
                _ => {}
            }
            log_ = (log_p != 0 || use_log_x as libc::c_uint != 0) as libc::c_int as Rboolean;
            if log_ as libc::c_uint != 0 && y == -1.0f64 / 0.0f64
                || log_ as u64 == 0 && y == 0 as libc::c_int as libc::c_double
            {
                w = pbeta_raw(
                    DBL_very_MIN,
                    pp,
                    qq,
                    TRUE as libc::c_int,
                    log_ as libc::c_int,
                );
                if log_ as libc::c_uint != 0 || fabs(w - a) <= fabs(y - a) {
                    tx = DBL_very_MIN;
                    u_n = DBL_log_v_MIN;
                }
                add_N_step = FALSE;
            } else if warned as u64 == 0
                && (if log_ as libc::c_uint != 0 {
                    (fabs(y - la) > 3 as libc::c_int as libc::c_double) as libc::c_int
                } else {
                    (fabs(y - a) > 1e-4f64) as libc::c_int
                }) != 0
            {
                if !(log_ as libc::c_uint != 0
                    && y == -1.0f64 / 0.0f64
                    && pbeta_raw(DBL_1__eps, pp, qq, TRUE as libc::c_int, TRUE as libc::c_int)
                        > la + 2 as libc::c_int as libc::c_double)
                {
                    printf(
                        b"qbeta(a, *) =: x0 with |pbeta(x0,*%s) - alpha| = %.5g is not accurate\0"
                            as *const u8 as *const libc::c_char,
                        if log_ as libc::c_uint != 0 {
                            b", log_\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        fabs(y - (if log_ as libc::c_uint != 0 { la } else { a })),
                    );
                }
            }
        }
        _ => {}
    }
    if give_log_q as u64 != 0 {
        if use_log_x as u64 == 0 {
            printf(
                b"qbeta() L_return, u_n=%g;  give_log_q=TRUE but use_log_x=FALSE -- please report!\0"
                    as *const u8 as *const libc::c_char,
                u_n,
            );
        }
        let mut r_0: libc::c_double = if u_n > -0.693147180559945309417232121458f64 {
            log(-expm1(u_n))
        } else {
            Rlog1p(-exp(u_n))
        };
        if swap_tail as u64 != 0 {
            *qb.offset(0 as libc::c_int as isize) = r_0;
            *qb.offset(1 as libc::c_int as isize) = u_n;
        } else {
            *qb.offset(0 as libc::c_int as isize) = u_n;
            *qb.offset(1 as libc::c_int as isize) = r_0;
        }
    } else {
        if use_log_x as u64 != 0 {
            if add_N_step as u64 != 0 {
                ::core::ptr::write_volatile(&mut xinbta as *mut libc::c_double, exp(u_n));
                y = pbeta_raw(xinbta, pp, qq, TRUE as libc::c_int, log_p);
                w = if log_p != 0 {
                    (y - la) * exp(y + logbeta + r * log(xinbta) + t * Rlog1p(-xinbta))
                } else {
                    (y - a) * exp(logbeta + r * log(xinbta) + t * Rlog1p(-xinbta))
                };
                tx = xinbta - w;
            } else {
                if swap_tail as u64 != 0 {
                    *qb.offset(0 as libc::c_int as isize) = -expm1(u_n);
                    *qb.offset(1 as libc::c_int as isize) = exp(u_n);
                } else {
                    *qb.offset(0 as libc::c_int as isize) = exp(u_n);
                    *qb.offset(1 as libc::c_int as isize) = -expm1(u_n);
                }
                return;
            }
        }
        if swap_tail as u64 != 0 {
            *qb.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double - tx;
            *qb.offset(1 as libc::c_int as isize) = tx;
        } else {
            *qb.offset(0 as libc::c_int as isize) = tx;
            *qb.offset(1 as libc::c_int as isize) = 1 as libc::c_int as libc::c_double - tx;
        }
    };
}
