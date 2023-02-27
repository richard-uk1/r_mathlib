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
    fn lgamma(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn logspace_add(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn pchisq(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
static mut _dbl_min_exp: libc::c_double =
    0.693147180559945309417232121458f64 * -(1021 as libc::c_int) as libc::c_double;
#[no_mangle]
pub unsafe extern "C" fn pnchisq(
    mut x: libc::c_double,
    mut df: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || df.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return x + df + ncp;
    }
    if R_finite(df) == 0 || R_finite(ncp) == 0 {
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
    if df < 0.0f64 || ncp < 0.0f64 {
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
    ans = Rf_pnchisq_raw(
        x,
        df,
        ncp,
        1e-12f64,
        8 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64,
        1000000 as libc::c_int,
        lower_tail as Rboolean,
        log_p as Rboolean,
    );
    if x <= 0.0f64 || x == 1.0f64 / 0.0f64 {
        return ans;
    }
    if ncp >= 80 as libc::c_int as libc::c_double {
        if lower_tail != 0 {
            ans = fmin2(ans, if log_p != 0 { 0.0f64 } else { 1.0f64 });
        } else {
            if ans
                < (if log_p != 0 {
                    -10.0f64 * 2.302585092994045684017991454684f64
                } else {
                    1e-10f64
                })
            {
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
                    printf(msg_1, b"pnchisq\0" as *const u8 as *const libc::c_char);
                }
            }
            if log_p == 0 && ans < 0.0f64 {
                ans = 0.0f64;
            }
        }
    }
    if log_p == 0 || ans < -1e-8f64 {
        return ans;
    } else {
        ans = Rf_pnchisq_raw(
            x,
            df,
            ncp,
            1e-12f64,
            8 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64,
            1000000 as libc::c_int,
            (lower_tail == 0) as libc::c_int as Rboolean,
            FALSE,
        );
        return Rlog1p(-ans);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Rf_pnchisq_raw(
    mut x: libc::c_double,
    mut f: libc::c_double,
    mut theta: libc::c_double,
    mut errmax: libc::c_double,
    mut reltol: libc::c_double,
    mut itrmax: libc::c_int,
    mut lower_tail: Rboolean,
    mut log_p: Rboolean,
) -> libc::c_double {
    let mut lam: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut f2: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    let mut bound: libc::c_double = 0.;
    let mut f_x_2n: libc::c_double = 0.;
    let mut f_2n: libc::c_double = 0.;
    let mut l_lam: libc::c_double = -1.0f64;
    let mut l_x: libc::c_double = -1.0f64;
    let mut n: libc::c_int = 0;
    let mut lamSml: Rboolean = FALSE;
    let mut tSml: Rboolean = FALSE;
    let mut is_r: Rboolean = FALSE;
    let mut is_b: Rboolean = FALSE;
    let mut ans: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut lt: libc::c_double = 0.;
    let mut lu: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    if x <= 0.0f64 {
        if x == 0.0f64 && f == 0.0f64 {
            return if lower_tail as libc::c_uint != 0 {
                if log_p as libc::c_uint != 0 {
                    -0.5f64 * theta
                } else {
                    exp(-0.5f64 * theta)
                }
            } else if log_p as libc::c_uint != 0 {
                if -0.5f64 * theta > -0.693147180559945309417232121458f64 {
                    log(-expm1(-0.5f64 * theta))
                } else {
                    Rlog1p(-exp(-0.5f64 * theta))
                }
            } else {
                -expm1(-0.5f64 * theta)
            };
        }
        return if lower_tail as libc::c_uint != 0 {
            if log_p as libc::c_uint != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p as libc::c_uint != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    if R_finite(x) == 0 {
        return if lower_tail as libc::c_uint != 0 {
            if log_p as libc::c_uint != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p as libc::c_uint != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if theta < 80 as libc::c_int as libc::c_double {
        let mut ans_0: libc::c_double = 0.;
        let mut i: libc::c_int = 0;
        if lower_tail as libc::c_uint != 0
            && f > 0.0f64
            && log(x)
                < 0.693147180559945309417232121458f64
                    + 2 as libc::c_int as libc::c_double / f
                        * (lgamma(f / 2.0f64 + 1 as libc::c_int as libc::c_double) + _dbl_min_exp)
        {
            let mut lambda: libc::c_double = 0.5f64 * theta;
            let mut sum: libc::c_double = 0.;
            let mut sum2: libc::c_double = 0.;
            let mut pr: libc::c_double = -lambda;
            sum2 = -1.0f64 / 0.0f64;
            sum = sum2;
            i = 0 as libc::c_int;
            while i < 110 as libc::c_int {
                sum2 = logspace_add(sum2, pr);
                sum = logspace_add(
                    sum,
                    pr + pchisq(
                        x,
                        f + (2 as libc::c_int * i) as libc::c_double,
                        lower_tail as libc::c_int,
                        TRUE as libc::c_int,
                    ),
                );
                if sum2 >= -1e-15f64 {
                    break;
                }
                i += 1;
                pr += log(lambda) - log(i as libc::c_double);
            }
            ans_0 = sum - sum2;
            return if log_p as libc::c_uint != 0 {
                ans_0
            } else {
                exp(ans_0)
            };
        } else {
            let mut lambda_0: libc::c_double = 0.5f64 * theta;
            let mut sum_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut sum2_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut pr_0: libc::c_double = exp(-lambda_0);
            i = 0 as libc::c_int;
            while i < 110 as libc::c_int {
                sum2_0 += pr_0;
                sum_0 += pr_0
                    * pchisq(
                        x,
                        f + (2 as libc::c_int * i) as libc::c_double,
                        lower_tail as libc::c_int,
                        FALSE as libc::c_int,
                    );
                if sum2_0 >= 1 as libc::c_int as libc::c_double - 1e-15f64 {
                    break;
                }
                i += 1;
                pr_0 *= lambda_0 / i as libc::c_double;
            }
            ans_0 = sum_0 / sum2_0;
            return if log_p as libc::c_uint != 0 {
                log(ans_0)
            } else {
                ans_0
            };
        }
    }
    lam = 0.5f64 * theta;
    lamSml = (-lam < _dbl_min_exp) as libc::c_int as Rboolean;
    if lamSml as u64 != 0 {
        u = 0 as libc::c_int as libc::c_double;
        lu = -lam;
        l_lam = log(lam);
    } else {
        u = exp(-lam);
    }
    v = u;
    x2 = 0.5f64 * x;
    f2 = 0.5f64 * f;
    f_x_2n = f - x;
    if f2 * 2.2204460492503131e-16f64 > 0.125f64 && {
        t = x2 - f2;
        fabs(t) < sqrt(2.2204460492503131e-16f64) * f2
    } {
        lt = (1 as libc::c_int as libc::c_double - t)
            * (2 as libc::c_int as libc::c_double - t / (f2 + 1 as libc::c_int as libc::c_double))
            - 0.918938533204672741780329736406f64
            - 0.5f64 * log(f2 + 1 as libc::c_int as libc::c_double);
    } else {
        lt = f2 * log(x2) - x2 - lgammafn(f2 + 1 as libc::c_int as libc::c_double);
    }
    tSml = (lt < _dbl_min_exp) as libc::c_int as Rboolean;
    if tSml as u64 != 0 {
        if x > f
            + theta
            + 5 as libc::c_int as libc::c_double
                * sqrt(
                    2 as libc::c_int as libc::c_double
                        * (f + 2 as libc::c_int as libc::c_double * theta),
                )
        {
            return if lower_tail as libc::c_uint != 0 {
                if log_p as libc::c_uint != 0 {
                    0.0f64
                } else {
                    1.0f64
                }
            } else if log_p as libc::c_uint != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            };
        }
        l_x = log(x);
        term = 0.0f64;
        ans = term;
        t = 0 as libc::c_int as libc::c_double;
    } else {
        t = exp(lt);
        term = v * t;
        ans = term;
    }
    n = 1 as libc::c_int;
    f_2n = f + 2.0f64;
    f_x_2n += 2.0f64;
    while n <= itrmax {
        if f_x_2n > 0 as libc::c_int as libc::c_double {
            bound = t * x / f_x_2n;
            is_r = FALSE;
            is_b = (bound <= errmax) as libc::c_int as Rboolean;
            if is_b as libc::c_uint != 0 && {
                is_r = (term <= reltol * ans) as libc::c_int as Rboolean;
                is_r as libc::c_uint != 0
            } {
                break;
            }
        }
        if lamSml as u64 != 0 {
            lu += l_lam - log(n as libc::c_double);
            if lu >= _dbl_min_exp {
                u = exp(lu);
                v = u;
                lamSml = FALSE;
            }
        } else {
            u *= lam / n as libc::c_double;
            v += u;
        }
        if tSml as u64 != 0 {
            lt += l_x - log(f_2n);
            if lt >= _dbl_min_exp {
                t = exp(lt);
                tSml = FALSE;
            }
        } else {
            t *= x / f_2n;
        }
        if lamSml as u64 == 0 && tSml as u64 == 0 {
            term = v * t;
            ans += term;
        }
        n += 1;
        f_2n += 2 as libc::c_int as libc::c_double;
        f_x_2n += 2 as libc::c_int as libc::c_double;
    }
    if n > itrmax {
        printf(
            b"pnchisq(x=%g, f=%g, theta=%g, ..): not converged in %d iter.\0" as *const u8
                as *const libc::c_char,
            x,
            f,
            theta,
            itrmax,
        );
    }
    let mut dans: libc::c_double = ans;
    return if lower_tail as libc::c_uint != 0 {
        if log_p as libc::c_uint != 0 {
            log(dans)
        } else {
            dans
        }
    } else if log_p as libc::c_uint != 0 {
        Rlog1p(-dans)
    } else {
        0.5f64 - dans + 0.5f64
    };
}
