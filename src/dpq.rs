use crate::LN2;

#[inline]
#[allow(non_snake_case)]
pub fn r_d__0(log_p: bool) -> f64 {
    if log_p {
        f64::NEG_INFINITY
    } else {
        0.0
    }
}

#[inline]
#[allow(non_snake_case)]
pub fn r_d__1(log_p: bool) -> f64 {
    if log_p {
        0.0
    } else {
        1.0
    }
}

#[inline]
pub fn r_d_half(log_p: bool) -> f64 {
    if log_p {
        -LN2
    } else {
        0.5
    }
}

#[inline]
pub fn r_dt_0(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d__0(log_p)
    } else {
        r_d__1(log_p)
    }
}

#[inline]
pub fn r_dt_1(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d__1(log_p)
    } else {
        r_d__0(log_p)
    }
}

#[inline]
pub fn r_d_cval(p: f64, lower_tail: bool) -> f64 {
    if lower_tail {
        0.5 - p + 0.5
    } else {
        p
    }
}

#[inline]
pub fn r_d_exp(x: f64, log: bool) -> f64 {
    if log {
        x
    } else {
        x.exp()
    }
}

#[inline]
pub fn r_d_fexp(f: f64, x: f64, log: bool) -> f64 {
    if log {
        -0.5 * f.ln() + x
    } else {
        x.ln() / f.sqrt()
    }
}

#[inline]
pub fn r_d_lexp(x: f64, log: bool) -> f64 {
    if log {
        r_log1_exp(x)
    } else {
        (-x.exp()).ln_1p()
    }
}

#[inline]
pub fn r_d_log(p: f64, log: bool) -> f64 {
    if log {
        p.ln()
    } else {
        p
    }
}

#[inline]
pub fn r_log1_exp(x: f64) -> f64 {
    if x > -LN2 {
        (-x.exp_m1()).ln()
    } else {
        (-x.exp()).ln_1p()
    }
}

#[inline]
pub fn r_dt_qiv(p: f64, lower_tail: bool, log_p: bool) -> f64 {
    if log_p {
        if lower_tail {
            p.exp()
        } else {
            -p.exp_m1()
        }
    } else {
        r_d_lval(p, lower_tail)
    }
}

#[inline]
pub fn r_d_lval(p: f64, lower_tail: bool) -> f64 {
    if lower_tail {
        p
    } else {
        0.5 - p + 0.5
    }
}

#[inline]
pub fn r_q_p01_boundaries(
    p: f64,
    left: f64,
    right: f64,
    lower_tail: bool,
    log_p: bool,
) -> Option<f64> {
    if log_p {
        if p > 0.0 {
            Some(f64::NAN)
        } else if p == 0.0 {
            Some(if lower_tail { right } else { left })
        } else if p == f64::NEG_INFINITY {
            Some(if lower_tail { left } else { right })
        } else {
            None
        }
    } else {
        if p < 0.0 || p > 1.0 {
            Some(f64::NAN)
        } else if p == 0.0 {
            Some(if lower_tail { left } else { right })
        } else if p == 1.0 {
            Some(if lower_tail { right } else { left })
        } else {
            None
        }
    }
}

#[inline]
pub fn r_dt_log(p: f64, lower_tail: bool, log: bool) -> f64 {
    if lower_tail {
        r_d_log(p, log)
    } else {
        r_d_lexp(p, log)
    }
}

#[inline]
pub fn r_dt_clog(p: f64, lower_tail: bool, log: bool) -> f64 {
    if lower_tail {
        r_d_lexp(p, log)
    } else {
        r_d_log(p, log)
    }
}

#[inline]
pub fn r_dt_civ(p: f64, lower_tail: bool, log: bool) -> f64 {
    if log {
        if lower_tail {
            -p.exp_m1()
        } else {
            p.exp()
        }
    } else {
        r_d_lval(p, lower_tail)
    }
}

// from mlutils.c

/// Like x.powf(y) but handles edge cases differently (I think).
pub fn r_pow(x: f64, y: f64) -> f64 {
    fn myfmod(x1: f64, x2: f64) -> f64 {
        let q = x1 / x2;
        x1 - q.floor() * x2
    }
    if x == 1. || y == 0. {
        return 1.;
    }
    if x == 0. {
        return if y > 0. { 0. } else { f64::INFINITY };
    }
    if x.is_finite() && y.is_finite() {
        return x.powf(y);
    }
    if x.is_nan() || y.is_nan() {
        return x + y;
    }
    if !x.is_finite() {
        if x > 0. {
            return if y < 0. { 0. } else { f64::INFINITY };
        } else {
            if y.is_finite() && y == y.floor() {
                return if y < 0. {
                    0.
                } else {
                    if myfmod(y, 2.) != 0. {
                        x
                    } else {
                        -x
                    }
                };
            }
        };
    }
    if !y.is_finite() {
        if x >= 0. {
            return if y > 0. {
                if x >= 1. {
                    f64::INFINITY
                } else {
                    0.
                }
            } else {
                if x < 1. {
                    f64::INFINITY
                } else {
                    0.
                }
            };
        }
    }
    f64::NAN
}

#[inline]
pub fn r_pow_di(mut x: f64, mut n: isize) -> f64 {
    let mut pow = 1.;

    if x.is_nan() {
        x
    } else {
        if n != 0 {
            if !x.is_finite() {
                return r_pow(x, n as f64);
            }
            if n < 0 {
                n = -n;
                x = x.recip();
            }
            loop {
                if n & 1 != 0 {
                    pow *= x;
                }
                n = n >> 1;
                if n != 0 {
                    x *= x;
                } else {
                    break;
                }
            }
        }
        pow
    }
}

#[inline]
pub fn r_dt_log(p: f64, lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d_log(p, log_p)
    } else {
        r_d_lexp(p, log_p)
    }
}

#[inline]
pub fn r_dt_clog(p: f64, lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d_lexp(p, log_p)
    } else {
        r_d_log(p, log_p)
    }
}
