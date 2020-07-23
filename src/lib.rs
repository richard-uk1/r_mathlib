//! This module provides functions from R's `stats` module, but rewritten for rust.
//!
//! I'm sticking to the `stats` module's naming for functions, even though they aren't at all
//! clear. Maybe this could change in the future.
//!
//! I'm also not implementing the non-stats stuff, such as graph drawing.

mod dpq;
mod puruspe;

use dpq::*;
use puruspe::{betai, erf, gamma, inverf, ln_gamma};
use std::f64::consts::PI;

const FRAC_1_SQRT_2PI: f64 = 0.3989422804014327; // (2.0f64 * PI).sqrt().recip()
const LN_SQRT_2PI: f64 = 0.9189385332046727; // (2.0 * PI).sqrt().ln()
const LN_SQRT_PID2: f64 = 0.22579135264472733; // (0.5 * PI).sqrt().ln()
const LN_2PI: f64 = 1.8378770664093453; // (2.0 * PI).ln()
const LN2: f64 = 0.6931471805599453; // (2.0).ln()
const PGAMMA_SCALE_FACTOR: f64 = 1.157920892373162e+77;
const M_CUTOFF: f64 = LN2 * (f64::MAX_EXP as f64) / f64::EPSILON;
const MIN_EXP: f64 = f64::MIN_EXP as f64;

/// Evaluate the probability density function of the normal distribution at x.
///
/// NOTE naive implementation (not R)
pub fn dnorm(x: f64, mean: f64, sd: f64, log: bool) -> f64 {
    let out = (sd * FRAC_1_SQRT_2PI).recip() * (-0.5 * ((x - mean) / sd).powi(2)).exp();
    if log {
        out.ln()
    } else {
        out
    }
}

/// Evaluate the culmulative distribution function of the normal distribution at x.
///
/// NOTE naive implementation (not R)
pub fn pnorm(q: f64, mean: f64, sd: f64, lower_tail: bool, log: bool) -> f64 {
    let mut out = 0.5 * (1. + erf((q - mean) / (sd * 2f64.sqrt())));
    if !lower_tail {
        out = 1. - out;
    }
    if log {
        out.ln()
    } else {
        out
    }
}

/// Evaluate the quantile function of the normal distribution at probability p.
///
/// NOTE naive implementation (not R)
pub fn qnorm(mut p: f64, mean: f64, sd: f64, lower_tail: bool, log: bool) -> f64 {
    if !lower_tail {
        p = 1. - p;
    }
    let out = mean + sd * 2.0f64.sqrt() * inverf(2. * p - 1.);
    if log {
        out.ln()
    } else {
        out
    }
}

/// Compute the probability density function for the t distribution at `x`. `df` is the degrees of
/// freedom and for hypothesis testing is the size of the sample - 1. `ncp` isn't implemented yet.
pub fn dt(x: f64, df: f64, ncp: Option<f64>, log: bool) -> f64 {
    let n = df;
    if x.is_nan() || !(n > 0.0) {
        return f64::NAN;
    }
    if ncp.is_some() {
        todo!()
    }
    if !x.is_finite() {
        return if log { f64::NEG_INFINITY } else { 0.0 };
    }
    if !n.is_finite() {
        return dnorm(x, 0.0, 1.0, log);
    }
    let t = -bd0(n * 0.5, (n + 1.0) * 0.5) + stirlerr((n + 1.0) * 0.5) - stirlerr(n * 0.5);
    let x2n = (x * x) / n;
    let lrg_x2n = x2n > f64::EPSILON.recip();
    let (l_x2n, u) = if lrg_x2n {
        let ax = x.abs();
        let l_x2n = ax.ln() - n.ln() * 0.5;
        (l_x2n, n * l_x2n)
    } else if x2n > 0.2 {
        let l_x2n = (1.0 + x2n).ln() * 0.5;
        (l_x2n, n * l_x2n)
    } else {
        let l_x2n = x2n.ln_1p() * 0.5;
        (l_x2n, -bd0(n * 0.5, (n + x * x) * 0.5) + x * x * 0.5)
    };
    if log {
        t - u - (LN_SQRT_2PI + l_x2n)
    } else {
        let i_sqrt = if lrg_x2n {
            n.sqrt() / x.abs()
        } else {
            (-l_x2n).exp()
        };
        (t - u).exp() * FRAC_1_SQRT_2PI * i_sqrt
    }
}

/// Compute the culmulative distribution function (CDF) for the t distribution at `x`. `df` is the
/// degrees of freedom and for hypothesis testing is the size of the sample - 1.
pub fn pt(x: f64, df: f64, lower_tail: bool, log_p: bool) -> f64 {
    let n = df;
    if n <= 0.0 || n.is_nan() || x.is_nan() {
        return f64::NAN;
    }
    if x == f64::NEG_INFINITY {
        return r_dt_0(lower_tail, log_p);
    } else if x == f64::INFINITY {
        return r_dt_1(lower_tail, log_p);
    }
    if n == f64::INFINITY {
        return pnorm(x, 0.0, 1.0, lower_tail, log_p);
    }
    let nx = 1.0 + (x / n) * x;

    let val = if nx > 1e100 {
        let lval = -0.5 * n * (2.0 * x.abs().ln() - n.ln()) - lbeta(0.5 * n, 0.5) - (0.5 * n).ln();
        if log_p {
            lval
        } else {
            lval.exp()
        }
    } else {
        if n > x * x {
            pbeta(x * x / (n + x * x), 0.5, n * 0.5, false, log_p)
        } else {
            pbeta(nx.recip(), n * 0.5, 0.5, true, log_p)
        }
    };
    let lower_tail = if x <= 0.0 { !lower_tail } else { lower_tail };
    if log_p {
        if lower_tail {
            (-0.5f64 * val.exp()).ln_1p()
        } else {
            val - LN2
        }
    } else {
        r_d_cval(val * 0.5, lower_tail)
    }
}

// /// Compute the quantile function (inverse CDF function) for the t distribution at `x`.
// pub fn qt(x: f64, df: f64, ncp: Option<f64>, lower_tail: bool, log_p: bool) -> f64 {
//    if df <= 0.0 {
//        return f64::NAN;
//    }
//    todo!()
//}

//dbeta todo
/// Compute the probability density functino of the beta distribution at x.
pub fn pbeta(x: f64, a: f64, b: f64, lower_tail: bool, log_p: bool) -> f64 {
    if x.is_nan() || a.is_nan() || b.is_nan() {
        x + a + b
    } else if a < 0.0 || b < 0.0 {
        f64::NAN
    } else if x <= 0.0 {
        r_dt_0(lower_tail, log_p)
    } else if x >= 1.0 {
        r_dt_1(lower_tail, log_p)
    } else if a == 0.0 || b == 0.0 || !a.is_finite() || !b.is_finite() {
        if a == 0.0 && b == 0.0 {
            if log_p {
                -LN2
            } else {
                0.5
            }
        } else if a == 0.0 || a / b == 0.0 {
            r_dt_1(lower_tail, log_p)
        } else if b == 0.0 || b / a == 0.0 {
            r_dt_0(lower_tail, log_p)
        } else if x < 0.5 {
            r_dt_0(lower_tail, log_p)
        } else {
            r_dt_1(lower_tail, log_p)
        }
    } else {
        let mut w = betai(a, b, x);
        if !lower_tail {
            w = 1.0 - w;
        }
        if log_p {
            w = w.ln()
        }
        w
    }
}

pub fn qbeta(alpha: f64, p: f64, q: f64, lower_tail: bool, log_p: bool) -> f64 {
    if p.is_nan() || q.is_nan() || alpha.is_nan() {
        p + q + alpha
    } else if p < 0.0 || q < 0.0 {
        f64::NAN
    } else {
        qbeta_raw(alpha, p, q, lower_tail, log_p, -5.0, 4).0
    }
}

/// Calculate the probability density function for the F distribution.
pub fn df(x: f64, m: f64, n: f64, log: bool) -> f64 {
    if x.is_nan() || m.is_nan() || n.is_nan() {
        x + m + n
    } else if m <= 0.0 || n <= 0.0 {
        f64::NAN
    } else if x < 0.0 {
        r_d__0(log)
    } else if x == 0.0 {
        if m > 2.0 {
            r_d__0(log)
        } else if m == 2.0 {
            r_d__1(log)
        } else {
            f64::INFINITY
        }
    } else if m == f64::INFINITY && n == f64::INFINITY {
        if x == 1.0 {
            f64::INFINITY
        } else {
            r_d__0(log)
        }
    } else if n == f64::INFINITY {
        dgamma(x, m / 2.0, 2.0 / m, log)
    } else if m > 1e14 {
        let dens = dgamma(1.0 / x, n / 2.0, 2.0 / n, log);
        if log {
            dens - 2.0 * x.ln()
        } else {
            dens / (x * x)
        }
    } else {
        let mut f = 1.0 / (n + x * m);
        let q = n * f;
        let p = x * m * f;
        let dens = if m >= 2.0 {
            f = m * q * 0.5;
            dbinom_raw((m - 2.0) * 0.5, (m + n - 2.0) * 0.5, p, q, log)
        } else {
            f = m * m * q / (2.0 * p * (m + n));
            dbinom_raw(m * 0.5, (m + n) * 0.5, p, q, log)
        };
        if log {
            f.ln() + dens
        } else {
            f * dens
        }
    }
}

/// Calculates the culmulative distribution function for the F distribution at x.
pub fn pf(x: f64, df1: f64, df2: f64, ncp: Option<f64>, lower_tail: bool, log_p: bool) -> f64 {
    if ncp.is_some() {
        todo!();
    }
    if x.is_nan() || df1.is_nan() || df2.is_nan() {
        x + df1 + df2
    } else if df1 <= 0.0 || df2 <= 0.0 {
        f64::NAN
    } else if x <= 0.0 {
        r_dt_0(lower_tail, log_p)
    } else if x == f64::INFINITY {
        r_dt_1(lower_tail, log_p)
    } else if df2 == f64::INFINITY {
        if df1 == f64::INFINITY {
            if x < 1.0 {
                r_dt_0(lower_tail, log_p)
            } else if x > 1.0 {
                r_dt_1(lower_tail, log_p)
            } else {
                // x == 1.0
                if log_p {
                    -LN2
                } else {
                    0.5
                }
            }
        } else {
            pchisq(x * df1, df1, lower_tail, log_p)
        }
    } else if df1 == f64::INFINITY {
        pchisq(df2 / x, df2, !lower_tail, log_p)
    } else {
        if df1 * x > df2 {
            pbeta(
                df2 / (df2 + df1 * x),
                df2 * 0.5,
                df1 * 0.5,
                !lower_tail,
                log_p,
            )
        } else {
            pbeta(
                df1 * x / (df2 + df2 * x),
                df1 * 0.5,
                df2 * 0.5,
                lower_tail,
                log_p,
            )
        }
    }
}

pub fn qf(p: f64, df1: f64, df2: f64, lower_tail: bool, log_p: bool) -> f64 {
    if p.is_nan() || df1.is_nan() || df2.is_nan() {
        p + df1 + df2
    } else if df1 <= 0.0 || df2 <= 0.0 {
        f64::NAN
    } else if let Some(v) = r_q_p01_boundaries(p, 0.0, f64::INFINITY, lower_tail, log_p) {
        v
    } else if df1 <= df2 && df2 > 4e5 {
        if df1 == f64::INFINITY {
            1.0
        } else {
            qchisq(p, df1, lower_tail, log_p) / df1
        }
    } else if df1 > 4e5 {
        df2 / qchisq(p, df2, !lower_tail, log_p)
    } else {
        (1.0 / qbeta(p, df2 * 0.5, df1 * 0.5, !lower_tail, log_p) - 1.0) * (df2 / df1)
    }
}

// dchisq
pub fn pchisq(x: f64, df: f64, lower_tail: bool, log_p: bool) -> f64 {
    pgamma(x, df * 0.5, 2.0, lower_tail, log_p)
}

pub fn qchisq(p: f64, df: f64, lower_tail: bool, log_p: bool) -> f64 {
    todo!()
}

/// Calculates the probability density function for the gamma distribution at x.
pub fn dgamma(x: f64, shape: f64, scale: f64, log: bool) -> f64 {
    if x.is_nan() || shape.is_nan() || scale.is_nan() {
        x + shape + scale
    } else if shape < 0.0 || scale < 0.0 {
        f64::NAN
    } else if x < 0.0 {
        r_d__0(log)
    } else if shape == 0.0 {
        if x == 0.0 {
            f64::INFINITY
        } else {
            r_d__0(log)
        }
    } else if x == 0.0 {
        if shape < 1.0 {
            f64::INFINITY
        } else if shape > 1.0 {
            r_d__0(log)
        } else if log {
            -scale.ln()
        } else {
            scale.recip()
        }
    } else if shape < 1.0 {
        let pr = dpois_raw(shape, x / scale, log);
        if log {
            pr + if (shape / x).is_finite() {
                (shape / x).ln()
            } else {
                // used only if (shape / x) overflows (to +inf).
                shape.ln() - x.ln()
            }
        } else {
            pr * shape / x
        }
    } else {
        let pr = dpois_raw(shape - 1.0, x / scale, log);
        if log {
            pr - scale.ln()
        } else {
            pr / scale
        }
    }
}

pub fn pgamma(x: f64, alph: f64, scale: f64, lower_tail: bool, log_p: bool) -> f64 {
    if x.is_nan() || alph.is_nan() || scale.is_nan() {
        x + alph + scale
    } else if alph < 0.0 || scale <= 0.0 {
        f64::NAN
    } else {
        let x = x / scale;
        if x.is_nan() {
            x
        } else if alph == 0.0 {
            if x <= 0.0 {
                r_dt_0(lower_tail, log_p)
            } else {
                r_dt_1(lower_tail, log_p)
            }
        } else {
            pgamma_raw(x, alph, lower_tail, log_p)
        }
    }
}
// qgamma

/// Return the PDF for the poisson distribution at x.
pub fn dpois(x: f64, lambda: f64, give_log: bool) -> f64 {
    if !close_int(x) {
        eprintln!(
            "warn: poisson should be evaluated at an integer, found {}",
            x
        );
    }
    if x.is_nan() || lambda.is_nan() {
        x + lambda
    } else if lambda < 0.0 {
        f64::NAN
    } else if x < 0.0 || !x.is_finite() {
        r_d__0(give_log)
    } else {
        dpois_raw(x.round(), lambda, give_log)
    }
}

/// Return the CDF for the poisson distribution at x.
pub fn ppois(x: f64, lambda: f64, lower_tail: bool, log_p: bool) -> f64 {
    if x.is_nan() || lambda.is_nan() {
        x + lambda
    } else if lambda < 0.0 {
        f64::NAN
    } else if x < 0.0 {
        r_dt_0(lower_tail, log_p)
    } else if lambda == 0.0 {
        r_dt_1(lower_tail, log_p)
    } else if x == f64::INFINITY {
        r_dt_1(lower_tail, log_p)
    } else {
        let x = (x + 1e-7).floor();
        pgamma(lambda, x + 1.0, 1.0, !lower_tail, log_p)
    }
}

/// Evaluates the quantile function for the poisson distribution at p.
pub fn qpois(mut p: f64, lambda: f64, lower_tail: bool, log_p: bool) -> f64 {
    if p.is_nan() || lambda.is_nan() {
        p + lambda
    } else if !lambda.is_finite() || lambda < 0.0 || !is_prob(p, log_p) {
        f64::NAN
    } else if lambda == 0.0 || p == r_dt_0(lower_tail, log_p) {
        0.0
    } else if p == r_dt_1(lower_tail, log_p) {
        f64::INFINITY
    } else {
        let mu = lambda;
        let sigma = lambda.sqrt();
        let gamma = sigma.recip();
        if !lower_tail || log_p {
            p = r_dt_qiv(p, lower_tail, log_p);
            if p == 0.0 {
                return 0.0;
            }
            if p == 1.0 {
                return f64::INFINITY;
            }
        }

        if p + 1.01 * f64::EPSILON >= 1.0 {
            return f64::INFINITY;
        }
        let z = qnorm(p, 0.0, 1.0, true, false);
        let mut y = mu + sigma * (z + gamma * (z * z - 1.0) / 6.0);
        let z = ppois(y, lambda, true, false);
        p *= 1.0 - 64.0 * f64::EPSILON;

        if lambda < 1e5 {
            do_search(y, &mut z, p, lambda, 1.0)
        } else {
            let mut incr = (y * 0.001).floor();
            let mut old_incr;
            loop {
                old_incr = incr;
                y = do_search(y, &mut z, p, lambda, incr);
                incr = (incr * 0.01).floor().max(1.0);
                if old_incr > 1.0 && incr > lambda * 1e-15 {
                    break;
                }
            }
            y
        }
    }
}

/// Evaluates the "deviance part" from Catherine Loader's algorithm.
fn bd0(x: f64, np: f64) -> f64 {
    if !x.is_finite() || !np.is_finite() || np == 0.0 {
        return f64::NAN;
    }

    if (x - np).abs() < 0.1 * (x + np) {
        let v = (x - np) / (x + np);
        let mut s = (x - np) * v;
        if s.abs() < f64::MIN_POSITIVE {
            return s;
        }
        let mut ej = 2.0 * x * v;
        let v2 = v * v;
        // prevents infinite loop, which is possible for certain input values.
        for j in 1..1000 {
            ej *= v2;
            let s1 = s + ej / ((j << 1) as f64 + 1.0);
            if s1 == s {
                return s1;
            }
            s = s1;
        }
    }

    // Either the loop didn't terminate or | x - np | is large enough for this to be accurate.
    x * (x / np).ln() + np - x
}

/// stirlerr(n) = log(n!) - log( sqrt(2*pi*n)*(n/e)^n )
///             = log Gamma(n+1) - 1/2 /// [log(2*pi) + log(n)] - n*[log(n) - 1]
///             = log Gamma(n+1) - (n + 1/2) * log(n) + n - log(2*pi)/2
///
/// see also lgammacor() in ./lgammacor.c  which computes almost the same!
fn stirlerr(n: f64) -> f64 {
    const S0: f64 = 0.083333333333333333333; // 1/12
    const S1: f64 = 0.00277777777777777777778; // 1/360
    const S2: f64 = 0.00079365079365079365079365; // 1/1260
    const S3: f64 = 0.000595238095238095238095238; // 1/1680
    const S4: f64 = 0.0008417508417508417508417508; // 1/1188

    // error for 0, 0.5, 1.0, 1.5, ..., 14.5, 15.0.
    const SFERR_HALVES: &[f64] = &[
        0.0,                           /* n=0 - wrong, place holder only */
        0.1534264097200273452913848,   /* 0.5 */
        0.0810614667953272582196702,   /* 1.0 */
        0.0548141210519176538961390,   /* 1.5 */
        0.0413406959554092940938221,   /* 2.0 */
        0.03316287351993628748511048,  /* 2.5 */
        0.02767792568499833914878929,  /* 3.0 */
        0.02374616365629749597132920,  /* 3.5 */
        0.02079067210376509311152277,  /* 4.0 */
        0.01848845053267318523077934,  /* 4.5 */
        0.01664469118982119216319487,  /* 5.0 */
        0.01513497322191737887351255,  /* 5.5 */
        0.01387612882307074799874573,  /* 6.0 */
        0.01281046524292022692424986,  /* 6.5 */
        0.01189670994589177009505572,  /* 7.0 */
        0.01110455975820691732662991,  /* 7.5 */
        0.010411265261972096497478567, /* 8.0 */
        0.009799416126158803298389475, /* 8.5 */
        0.009255462182712732917728637, /* 9.0 */
        0.008768700134139385462952823, /* 9.5 */
        0.008330563433362871256469318, /* 10.0 */
        0.007934114564314020547248100, /* 10.5 */
        0.007573675487951840794972024, /* 11.0 */
        0.007244554301320383179543912, /* 11.5 */
        0.006942840107209529865664152, /* 12.0 */
        0.006665247032707682442354394, /* 12.5 */
        0.006408994188004207068439631, /* 13.0 */
        0.006171712263039457647532867, /* 13.5 */
        0.005951370112758847735624416, /* 14.0 */
        0.005746216513010115682023589, /* 14.5 */
        0.005554733551962801371038690, /* 15.0 */
    ];

    if n <= 15.0 {
        let nn = n + n;
        if nn == nn.trunc() {
            return SFERR_HALVES[nn.trunc() as usize];
        }
        return lgammafn(n + 1.0) - (n + 0.5) * n.ln() + n - LN_SQRT_2PI;
    }

    let nn = (n * n).recip();
    if n > 500.0 {
        (S0 - S1 * nn) / n
    } else if n > 80.0 {
        (S0 - (S1 - S2 * nn) * nn) / n
    } else if n > 35.0 {
        (S0 - (S1 - (S2 - S3 * nn) * nn) * nn) / n
    } else {
        // 15 < n <= 35
        (S0 - (S1 - (S2 - (S3 - S4 * nn) * nn) * nn) * nn) / n
    }
}

/// Returns the result of ln(gamma(x)).
fn lgammafn(x: f64) -> f64 {
    const XMAX: f64 = 2.5327372760800758e+305; // f64::MAX / f64::MAX.ln()
    const DXREL: f64 = 1.4901161193847656e-08; // f64::EPSILON.sqrt()

    if x.is_nan() {
        return x;
    }
    if x <= 0.0 && x == x.trunc() {
        return f64::INFINITY;
    }
    let y = x.abs();

    if y < 1e-306 {
        return -y.ln();
    }
    if y <= 10.0 {
        return gamma(x).abs().ln();
    }
    if y > XMAX {
        return f64::INFINITY;
    }
    if x > 0.0 {
        return if x > 1e17 {
            x * (x.ln() - 1.0)
        } else if x > 4934720.0 {
            LN_SQRT_2PI + (x - 0.5) * x.ln() - x
        } else {
            LN_SQRT_2PI + (x - 0.5) * x.ln() - x + lgammacor(x)
        };
    }
    let sinpiy = sinpi(y).abs();
    let ans = LN_SQRT_PID2 + (x - 0.5) * y.ln() - x - sinpiy.ln() - lgammacor(y);
    if ((x - (x - 0.5).trunc()) * ans / x) < DXREL {
        //eprintln!("warning lgammafn precision low as x near 0.");
    }
    ans
}

/// Compute the log gamma correction factor for x >= 10 so that
///
/// `log(gamma(x)) = .5*log(2*pi) + (x-.5)*log(x) -x + lgammacor(x)`
///
/// `[ lgammacor(x) is called Del(x) in other contexts (e.g. dcdflib)]
fn lgammacor(x: f64) -> f64 {
    const ALG_MCS: &[f64] = &[
        0.1666389480451863247205729650822e+0,
        -0.1384948176067563840732986059135e-4,
        0.9810825646924729426157171547487e-8,
        -0.1809129475572494194263306266719e-10,
        0.6221098041892605227126015543416e-13,
        -0.3399615005417721944303330599666e-15,
        0.2683181998482698748957538846666e-17,
        -0.2868042435334643284144622399999e-19,
        0.3962837061046434803679306666666e-21,
        -0.6831888753985766870111999999999e-23,
        0.1429227355942498147573333333333e-24,
        -0.3547598158101070547199999999999e-26,
        0.1025680058010470912000000000000e-27,
        -0.3401102254316748799999999999999e-29,
        0.1276642195630062933333333333333e-30,
    ];

    const NALGM: usize = 5;
    const XBIG: f64 = 94906265.62425156; // 2.0f64.powf(26.5)
    const XMAX: f64 = f64::MAX / 48.0;

    if x < 10.0 {
        f64::NAN
    } else if x >= XMAX {
        f64::NAN // would underflow
    } else if x < XBIG {
        let tmp = 10.0 / x;
        chebyshev_eval(tmp * tmp * 2.0 - 1.0, &ALG_MCS[..NALGM]) / x
    } else {
        (x * 12.0).recip()
    }
}

fn gammafn(x: f64) -> f64 {
    const GMACS: &[f64] = &[
        0.8571195590989331421920062399942e-2,
        0.4415381324841006757191315771652e-2,
        0.5685043681599363378632664588789e-1,
        -0.4219835396418560501012500186624e-2,
        0.1326808181212460220584006796352e-2,
        -0.1893024529798880432523947023886e-3,
        0.3606925327441245256578082217225e-4,
        -0.6056761904460864218485548290365e-5,
        0.1055829546302283344731823509093e-5,
        -0.1811967365542384048291855891166e-6,
        0.3117724964715322277790254593169e-7,
        -0.5354219639019687140874081024347e-8,
        0.9193275519859588946887786825940e-9,
        -0.1577941280288339761767423273953e-9,
        0.2707980622934954543266540433089e-10,
        -0.4646818653825730144081661058933e-11,
        0.7973350192007419656460767175359e-12,
        -0.1368078209830916025799499172309e-12,
        0.2347319486563800657233471771688e-13,
        -0.4027432614949066932766570534699e-14,
        0.6910051747372100912138336975257e-15,
        -0.1185584500221992907052387126192e-15,
        0.2034148542496373955201026051932e-16,
        -0.3490054341717405849274012949108e-17,
        0.5987993856485305567135051066026e-18,
        -0.1027378057872228074490069778431e-18,
        0.1762702816060529824942759660748e-19,
        -0.3024320653735306260958772112042e-20,
        0.5188914660218397839717833550506e-21,
        -0.8902770842456576692449251601066e-22,
        0.1527474068493342602274596891306e-22,
        -0.2620731256187362900257328332799e-23,
        0.4496464047830538670331046570666e-24,
        -0.7714712731336877911703901525333e-25,
        0.1323635453126044036486572714666e-25,
        -0.2270999412942928816702313813333e-26,
        0.3896418998003991449320816639999e-27,
        -0.6685198115125953327792127999999e-28,
        0.1146998663140024384347613866666e-28,
        -0.1967938586345134677295103999999e-29,
        0.3376448816585338090334890666666e-30,
        -0.5793070335782135784625493333333e-31,
    ];

    const NGAM: usize = 22;
    const XMIN: f64 = -170.5674972726612;
    const XMAX: f64 = 171.61447887182298;
    const XSML: f64 = 2.2474362225598545e-308;
    const DXREL: f64 = 1.490116119384765696e-8;
    if x.is_nan() {
        return x;
    }
    if x == 0.0 || x < 0.0 && x == x.round() {
        return f64::NAN;
    }

    let y = x.abs();
    if y <= 10.0 {
        let mut n = x as isize;
        if x < 0.0 {
            n -= 1;
        }
        let y = x - n as f64;
        n -= 1;
        let mut value = chebyshev_eval(y * 2.0 - 1.0, &GMACS[..NGAM]) + 0.9375;
        if n == 0 {
            return value;
        } else if n < 0 {
            if x < -0.5 && (x - (x - 0.5).floor() / x) < DXREL {
                //panic!("less than 1/2 precision as near a pole (negative integer)
            }
            if y < XSML {
                // overflow
                if x > 0.0 {
                    f64::INFINITY
                } else {
                    f64::NEG_INFINITY
                }
            } else {
                let n = -n as usize;
                for i in 0..n {
                    value /= x + i as f64;
                }
                value
            }
        } else {
            for i in 1..=n {
                value *= y + i as f64;
            }
            value
        }
    } else {
        if x > XMAX {
            return f64::INFINITY;
        } else if x < XMIN {
            return 0.0;
        }

        let mut value;
        if y <= 50.0 && y.floor() == y {
            value = 1.0;
            for i in 2..(y as usize) {
                value *= i as f64;
            }
        } else {
            let cor = if (2.0 * y).floor() == 2.0 * y {
                stirlerr(y)
            } else {
                lgammacor(y)
            };
            value = ((y - 0.5) * y.ln() - y + LN_SQRT_2PI + cor).exp();
        }

        if x > 0.0 {
            return value;
        }

        if (x - (x - 0.5).floor() / x).abs() < DXREL {
            //println!("less than 0.5 precision")
        }

        let sinpiy = sinpi(y);
        if sinpiy == 0.0 {
            f64::INFINITY
        } else {
            -PI / (y * sinpiy * value)
        }
    }
}

fn lgamma1p(a: f64) -> f64 {
    const EULERS_CONST: f64 = 0.5772156649015328606065120900824024;

    const N: usize = 40;

    const COEFFS: &[f64] = &[
        0.3224670334241132182362075833230126e-0, /* = (zeta(2)-1)/2 */
        0.6735230105319809513324605383715000e-1, /* = (zeta(3)-1)/3 */
        0.2058080842778454787900092413529198e-1,
        0.7385551028673985266273097291406834e-2,
        0.2890510330741523285752988298486755e-2,
        0.1192753911703260977113935692828109e-2,
        0.5096695247430424223356548135815582e-3,
        0.2231547584535793797614188036013401e-3,
        0.9945751278180853371459589003190170e-4,
        0.4492623673813314170020750240635786e-4,
        0.2050721277567069155316650397830591e-4,
        0.9439488275268395903987425104415055e-5,
        0.4374866789907487804181793223952411e-5,
        0.2039215753801366236781900709670839e-5,
        0.9551412130407419832857179772951265e-6,
        0.4492469198764566043294290331193655e-6,
        0.2120718480555466586923135901077628e-6,
        0.1004322482396809960872083050053344e-6,
        0.4769810169363980565760193417246730e-7,
        0.2271109460894316491031998116062124e-7,
        0.1083865921489695409107491757968159e-7,
        0.5183475041970046655121248647057669e-8,
        0.2483674543802478317185008663991718e-8,
        0.1192140140586091207442548202774640e-8,
        0.5731367241678862013330194857961011e-9,
        0.2759522885124233145178149692816341e-9,
        0.1330476437424448948149715720858008e-9,
        0.6422964563838100022082448087644648e-10,
        0.3104424774732227276239215783404066e-10,
        0.1502138408075414217093301048780668e-10,
        0.7275974480239079662504549924814047e-11,
        0.3527742476575915083615072228655483e-11,
        0.1711991790559617908601084114443031e-11,
        0.8315385841420284819798357793954418e-12,
        0.4042200525289440065536008957032895e-12,
        0.1966475631096616490411045679010286e-12,
        0.9573630387838555763782200936508615e-13,
        0.4664076026428374224576492565974577e-13,
        0.2273736960065972320633279596737272e-13,
        0.1109139947083452201658320007192334e-13, /* = (zeta(40+1)-1)/(40+1) */
    ];

    const C: f64 = 0.2273736845824652515226821577978691e-12; /* zeta(N+2)-1 */
    const TOL_LOGCF: f64 = 1e-14;
    if a.abs() >= 0.5 {
        return lgammafn(a + 1.0);
    }

    let mut lgam = C * logcf(-a / 2.0, N as f64 + 2.0, 1.0, TOL_LOGCF);
    for i in (N - 1)..=0 {
        lgam = COEFFS[i] - a * lgam;
    }
    (a * lgam - EULERS_CONST) * a - log1pmx(a)
}

fn chebyshev_eval(x: f64, a: &[f64]) -> f64 {
    // sanity checks.
    if a.len() < 1 || a.len() > 1000 {
        return f64::NAN;
    }
    // I assume that the approximation is not accurate away from the origin.
    if x < -1.0 || x > 1.1 {
        return f64::NAN;
    }
    let xx = x + x;
    let mut b2 = 0.0;
    let mut b1 = 0.0;
    let mut b0 = 0.0;
    for coeff in a.into_iter().rev() {
        b2 = b1;
        b1 = b0;
        b0 = xx * b1 - b2 + coeff;
    }
    (b0 - b2) * 0.5
}

/// like x.sin(), except exact for multiples of 0.5.
fn sinpi(x: f64) -> f64 {
    let x = x % 2.0; // sin(pi(x + 2k)) == sin(pi x)  for all integer k
                     // map (-2,2) --> (-1,1] :
    let x = if x <= -1.0 {
        x + 2.0
    } else if x > 1.0 {
        x - 2.0
    } else {
        x
    };
    if x == 0.0 || x == 1.0 {
        0.0
    } else if x == 0.5 {
        1.0
    } else if x == -0.5 {
        -1.0
    } else {
        (PI * x).sin()
    }
}

/// Computes ln(beta(a, b))
fn lbeta(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::NAN;
    }
    let p = a.min(b);
    let q = a.max(b);

    if p < 0.0 {
        // also checks q >= 0
        return f64::NAN;
    } else if p == f64::INFINITY {
        return f64::INFINITY;
    } else if q == f64::INFINITY {
        return f64::NEG_INFINITY;
    }

    if p >= 10.0 {
        let corr = lgammacor(p) + lgammacor(q) - lgammacor(p + q);
        q.ln() * -0.5
            + LN_SQRT_2PI
            + corr
            + (p - 0.5) * (p / (p + q)).ln()
            + q * (-p / (p + q)).ln_1p()
    } else if q >= 10.0 {
        let corr = lgammacor(q) - lgammacor(p + q);
        lgammafn(p) + corr + p - p * (p + q).ln() + (q - 0.5) * (-p / (p + q)).ln_1p()
    } else if p < 1e-306 {
        ln_gamma(p) + (ln_gamma(q) - ln_gamma(p + q))
    } else {
        (gammafn(p) * (gammafn(q) / gammafn(p + q))).ln()
    }
}

fn dbinom_raw(x: f64, n: f64, p: f64, q: f64, log: bool) -> f64 {
    if p == 0.0 {
        if x == 0.0 {
            r_d__1(log)
        } else {
            r_d__0(log)
        }
    } else if q == 0.0 {
        if x == n {
            r_d__1(log)
        } else {
            r_d__0(log)
        }
    } else if x == 0.0 {
        if n == 0.0 {
            r_d__1(log)
        } else {
            r_d_exp(
                if p < 0.1 {
                    -bd0(n, n * q) - n * p
                } else {
                    n * q.ln()
                },
                log,
            )
        }
    } else if x == n {
        r_d_exp(
            if q < 0.1 {
                -bd0(n, n * p) - n * q
            } else {
                n * p.ln()
            },
            log,
        )
    } else if x < 0.0 || x > n {
        r_d__0(log)
    } else {
        let lc = stirlerr(n) - stirlerr(x) - stirlerr(n - x) - bd0(x, n * p) - bd0(n - x, n * q);
        let lf = LN_2PI + x.ln() + (-x / n).ln_1p();
        r_d_exp(lc - 0.5 * lf, log)
    }
}

fn dpois_raw(x: f64, lambda: f64, log: bool) -> f64 {
    debug_assert!(x >= 0.0);
    debug_assert!(lambda >= 0.0);
    if lambda == 0.0 {
        if x == 0.0 {
            r_d__1(log)
        } else {
            r_d__0(log)
        }
    } else if lambda == f64::INFINITY {
        r_d__0(log)
    } else if x < 0.0 {
        r_d__0(log)
    } else if x <= lambda + f64::MIN_POSITIVE {
        r_d_exp(-lambda, log)
    } else if lambda < x * f64::MIN_POSITIVE {
        if x == f64::INFINITY {
            r_d__0(log)
        } else {
            r_d_exp(-lambda + x * lambda.ln() - lgammafn(x + 1.0), log)
        }
    } else {
        r_d_fexp(PI * 2.0 * x, -stirlerr(x) - bd0(x, lambda), log)
    }
}

fn pgamma_raw(x: f64, alph: f64, lower_tail: bool, log_p: bool) -> f64 {
    debug_assert!(alph > 0.0);
    debug_assert!(!x.is_nan());
    if x <= 0.0 {
        return r_dt_0(lower_tail, log_p);
    }
    if x == f64::INFINITY {
        return r_dt_1(lower_tail, log_p);
    }

    let res = if x < 1.0 {
        pgamma_smallx(x, alph, lower_tail, log_p)
    } else if x < alph - 1.0 && x < 0.8 * (alph + 50.0) {
        let sum = pd_upper_series(x, alph, log_p);
        let d = dpois_wrap(alph, x, log_p);
        if !lower_tail {
            if log_p {
                r_log1_exp(d + sum)
            } else {
                1.0 - d * sum
            }
        } else {
            if log_p {
                sum + d
            } else {
                sum * d
            }
        }
    } else if alph - 1.0 < x && alph < 0.8 * (x + 50.0) {
        let d = dpois_wrap(alph, x, log_p);
        let sum = if alph < 1.0 {
            if x * f64::EPSILON > 1.0 - alph {
                r_d__1(log_p)
            } else {
                let f = pd_lower_cf(alph, x - (alph - 1.0)) * x / alph;
                if log_p {
                    f.ln()
                } else {
                    f
                }
            }
        } else {
            let s = pd_lower_series(x, alph - 1.0);
            if log_p {
                s.ln_1p()
            } else {
                1.0 + s
            }
        };

        if !lower_tail {
            if log_p {
                sum + d
            } else {
                sum * d
            }
        } else {
            if log_p {
                r_log1_exp(d + sum)
            } else {
                1.0 - d * sum
            }
        }
    } else {
        ppois_asymp(alph - 1.0, x, !lower_tail, log_p)
    };

    if !log_p && res < f64::MIN_POSITIVE / f64::EPSILON {
        pgamma_raw(x, alph, lower_tail, true)
    } else {
        res
    }
}

fn pgamma_smallx(x: f64, alph: f64, lower_tail: bool, log_p: bool) -> f64 {
    let mut sum = 0.0;
    let mut c = alph;
    let mut n = 0.0;

    loop {
        n += 1.0;
        c *= -x / n;
        let term = c / (alph + n);
        sum += term;
        if term.abs() > f64::EPSILON * sum.abs() {
            break;
        }
    }

    if lower_tail {
        let f1 = if log_p { sum.ln_1p() } else { 1.0 + sum };
        let f2 = if alph > 1.0 {
            let f2tmp = dpois_raw(alph, x, log_p);
            if log_p {
                f2tmp + x
            } else {
                f2tmp * x.exp()
            }
        } else if log_p {
            alph * x.ln() - lgamma1p(alph)
        } else {
            x.powf(alph) / lgamma1p(alph).exp()
        };
        if log_p {
            f1 + f2
        } else {
            f1 * f2
        }
    } else {
        let lf2 = alph * x.ln() - lgamma1p(alph);
        if log_p {
            r_log1_exp(sum.ln_1p() + lf2)
        } else {
            let f1m1 = sum;
            let f2m1 = lf2.exp_m1();
            -(f1m1 + f2m1 + f1m1 * f2m1)
        }
    }
}

fn pd_upper_series(x: f64, mut y: f64, log_p: bool) -> f64 {
    let mut term = x / y;
    let mut sum = term;

    loop {
        y += 1.0;
        term *= x / y;
        sum += term;
        if term > sum + f64::EPSILON {
            break;
        }
    }

    if log_p {
        sum.ln()
    } else {
        sum
    }
}

fn pd_lower_cf(y: f64, d: f64) -> f64 {
    const MAX_ITER: f64 = 200_000.0;

    if y == 0.0 {
        return 0.0;
    }

    let mut f0 = y / d;
    if (y - 1.0).abs() < d.abs() * f64::EPSILON {
        return f0;
    }

    if f0 > 1.0 {
        f0 = 1.0;
    }

    let mut c2 = y;
    let mut c4 = d;

    let mut a1 = 0.0;
    let mut b1 = 1.0;
    let mut a2 = y;
    let mut b2 = d;

    while b2 > PGAMMA_SCALE_FACTOR {
        a1 /= PGAMMA_SCALE_FACTOR;
        b1 /= PGAMMA_SCALE_FACTOR;
        a2 /= PGAMMA_SCALE_FACTOR;
        b2 /= PGAMMA_SCALE_FACTOR;
    }

    let mut i = 0.0;
    let mut of = -1.0;
    while i < MAX_ITER {
        i += 1.0;
        c2 -= 1.0;
        let c3 = i * c2;
        c4 += 2.0;

        a1 = c4 * a2 + c3 * a1;
        b1 = c4 * b2 + c3 * b1;

        i += 1.0;
        c2 -= 1.0;
        let c3 = i * c2;
        c4 += 2.0;

        a2 = c4 * a1 + c3 * a2;
        b2 = c4 * b1 + c3 * b2;

        if b2 > PGAMMA_SCALE_FACTOR {
            a1 /= PGAMMA_SCALE_FACTOR;
            b1 /= PGAMMA_SCALE_FACTOR;
            a2 /= PGAMMA_SCALE_FACTOR;
            b2 /= PGAMMA_SCALE_FACTOR;
        }

        if b2 != 0.0 {
            let f = a2 / b2;
            if (f - of).abs() <= f64::EPSILON * f0.max(f.abs()) {
                return f;
            }
            of = f;
        }
    }
    panic!("pd_lower_cf did not converge");
}

fn pd_lower_series(lambda: f64, mut y: f64) -> f64 {
    let mut term = 1.0;
    let mut sum = 0.0;

    while y >= 1.0 && term > sum * f64::EPSILON {
        term *= y / lambda;
        sum += term;
        y -= 1.0;
    }

    if y != y.floor() {
        let f = pd_lower_cf(y, lambda + 1.0 - y);
        sum += term * f;
    }

    sum
}

/// Continued fraction for calculation of
///    1/i + x/(i+d) + x^2/(i+2*d) + x^3/(i+3*d) + ... = sum_{k=0}^Inf x^k/(i+k*d)
///
/// auxilary in log1pmx() and lgamma1p()
fn logcf(x: f64, i: f64, d: f64, eps: f64) -> f64 {
    debug_assert!(i > 0.0);
    debug_assert!(d >= 0.0);

    let mut c1 = 2.0 * d;
    let mut c2 = i + d;
    let mut c4 = c2 + d;
    let mut a1 = c2;
    let mut b1 = i * (c2 - i * x);
    let mut b2 = d * d * x;
    let mut a2 = c4 * c2 - b2;

    b2 = c4 * b1 - i * b2;

    while (a2 * b1 - a1 * b2).abs() > (eps * b1 * b2).abs() {
        let mut c3 = c2 * c2 * x;
        c2 += d;
        c4 += d;
        a1 = c4 * a2 - c3 * a1;
        b1 = c4 * b2 - c3 * b1;

        c3 = c1 * c1 * x;
        c1 += d;
        c4 += d;
        a2 = c4 * a1 - c3 * a2;
        b2 = c4 * b1 - c3 * b2;

        if b2.abs() > PGAMMA_SCALE_FACTOR {
            a1 /= PGAMMA_SCALE_FACTOR;
            b1 /= PGAMMA_SCALE_FACTOR;
            a2 /= PGAMMA_SCALE_FACTOR;
            b2 /= PGAMMA_SCALE_FACTOR;
        } else if b2.abs() < PGAMMA_SCALE_FACTOR.recip() {
            a1 *= PGAMMA_SCALE_FACTOR;
            b1 *= PGAMMA_SCALE_FACTOR;
            a2 *= PGAMMA_SCALE_FACTOR;
            b2 *= PGAMMA_SCALE_FACTOR;
        }
    }

    a2 / b2
}

/// Accurate calculation of log(1+x)-x, particularly for small x.
fn log1pmx(x: f64) -> f64 {
    const MIN_LOG1_VALUE: f64 = -0.79149064;

    if x > 1.0 || x < MIN_LOG1_VALUE {
        x.ln_1p() - x
    } else {
        let r = x / (2.0 + x);
        let y = r * r;
        if x.abs() < 1e-2 {
            r * ((((2.0 / 9.0 * y + 2.0 / 7.0) * y + 2.0 / 5.0) * y + 2.0 / 3.0) * y - x)
        } else {
            const TOL_LOGCF: f64 = 1e-14;
            r * (2.0 * y * logcf(y, 3.0, 2.0, TOL_LOGCF) - x)
        }
    }
}

/// dpois_wrap (xp1, lambda) := dpois(xp1 - 1, lambda);  where
/// dpois(k, L) := exp(-L) L^k / gamma(k+1)  {the usual Poisson probabilities}
fn dpois_wrap(x_plus_1: f64, lambda: f64, give_log: bool) -> f64 {
    debug_assert!(x_plus_1 > 0.0);
    debug_assert!(lambda > 0.0);
    if lambda == f64::INFINITY {
        r_d__0(give_log)
    } else if x_plus_1 > 1.0 {
        dpois_raw(x_plus_1 - 1.0, lambda, give_log)
    } else if lambda > (x_plus_1 - 1.0).abs() * M_CUTOFF {
        r_d_exp(-lambda - lgammafn(x_plus_1), give_log)
    } else {
        let d = dpois_raw(x_plus_1, lambda, give_log);

        if give_log {
            d + (x_plus_1 / lambda).ln()
        } else {
            d * (x_plus_1 / lambda)
        }
    }
}

fn ppois_asymp(x: f64, lambda: f64, lower_tail: bool, log_p: bool) -> f64 {
    const COEFS_A: &[f64] = &[
        -1e99, /* placeholder used for 1-indexing */
        2.0 / 3.0,
        -4.0 / 135.0,
        8.0 / 2835.0,
        16.0 / 8505.0,
        -8992.0 / 12629925.0,
        -334144.0 / 492567075.0,
        698752.0 / 1477701225.0,
    ];

    const COEFS_B: &[f64] = &[
        -1e99, /* placeholder */
        1.0 / 12.0,
        1.0 / 288.0,
        -139.0 / 51840.0,
        -571.0 / 2488320.0,
        163879.0 / 209018880.0,
        5246819.0 / 75246796800.0,
        -534703531.0 / 902961561600.0,
    ];

    let dfm = lambda - x;
    let pt_ = -log1pmx(dfm / x);
    let mut s2pt = (2.0 * x * pt_).sqrt();
    if dfm < 0.0 {
        s2pt = -s2pt;
    }

    let mut res12 = 0.0;
    let mut res1_ig = x.sqrt();
    let mut res1_term = x.sqrt();
    let mut res2_ig = s2pt;
    let mut res2_term = s2pt;

    for (i, (coeff_a, coeff_b)) in COEFS_A
        .iter()
        .copied()
        .zip(COEFS_B.iter().copied())
        .enumerate()
    {
        res12 += res1_ig * coeff_a + res2_ig * coeff_b;
        res1_term *= pt_ / i as f64;
        res2_term *= 2.0 * pt_ / (2.0 * i as f64 + 1.0);
        res1_ig = res1_ig / x + res1_term;
        res2_ig = res2_ig / x + res2_term;
    }

    let mut elfb = x;
    let mut elfb_term = 1.0;

    for coeff_b in COEFS_B.iter().copied() {
        elfb += elfb_term * coeff_b;
        elfb_term /= x;
    }

    if !lower_tail {
        elfb = -elfb;
    }

    let f = res12 / elfb;
    let np = pnorm(s2pt, 0.0, 1.0, !lower_tail, log_p);

    if log_p {
        let n_d_over_p = dpnorm(s2pt, !lower_tail, np);
        np + (f * n_d_over_p).ln_1p()
    } else {
        let nd = dnorm(s2pt, 0.0, 1.0, log_p);
        np + f * nd
    }
}

///       dnorm (x, 0, 1, FALSE)
/// ----------------------------------
/// pnorm (x, 0, 1, lower_tail, FALSE)
///
/// expects
/// lp = pnorm (x, 0, 1, lower_tail, TRUE)
fn dpnorm(mut x: f64, mut lower_tail: bool, lp: f64) -> f64 {
    if x < 0.0 {
        x = -x;
        lower_tail = !lower_tail;
    }

    if x > 10.0 && !lower_tail {
        let mut term = x.recip();
        let mut sum = term;
        let x2 = x * x;
        let mut i = 1.0;
        loop {
            term *= -i / x2;
            sum += term;
            i += 2.0;
            if term.abs() > f64::EPSILON * sum {
                break;
            }
        }

        sum.recip()
    } else {
        dnorm(x, 0.0, 1.0, false) / lp.exp()
    }
}

/// Is x an int to 7 significant figures or 7 decimal places?
#[inline]
fn close_int(x: f64) -> bool {
    (x - x.round()).abs() < 1e-7 * x.abs().max(1.0)
}

/// Is p a probability (in [0, 1]) (or log of one if `log`)
#[inline]
fn is_prob(p: f64, log_p: bool) -> bool {
    if log_p {
        p <= 0.0
    } else {
        0.0 <= p && p <= 1.0
    }
}

#[inline]
fn do_search(mut y: f64, z: &mut f64, p: f64, lambda: f64, incr: f64) -> f64 {
    if *z >= p {
        loop {
            if y == 0.0 {
                return y;
            }
            *z = ppois(y - incr, lambda, true, false);
            if *z < p {
                return y;
            }
            y = (y - incr).max(0.0);
        }
    } else {
        loop {
            y = y + incr;
            *z = ppois(y, lambda, true, false);
            if *z >= p {
                return y;
            }
        }
    }
}

fn qbeta_raw(
    alpha: f64,
    p: f64,
    q: f64,
    lower_tail: bool,
    log_p: bool,
    log_q_cut: f64,
    n_N: usize,
) -> (f64, f64) {
    #[inline]
    fn return_q_0(give_log_q: bool) -> (f64, f64) {
        if give_log_q {
            (f64::NEG_INFINITY, 0.0)
        } else {
            (0.0, 1.0)
        }
    }
    #[inline]
    fn return_q_1(give_log_q: bool) -> (f64, f64) {
        if give_log_q {
            (0.0, f64::NEG_INFINITY)
        } else {
            (1.0, 0.0)
        }
    }
    #[inline]
    fn return_q_half(give_log_q: bool) -> (f64, f64) {
        if give_log_q {
            (-LN2, -LN2)
        } else {
            (0.5, 0.5)
        }
    }

    const CONST1: f64 = 2.30753;
    const CONST2: f64 = 0.27061;
    const CONST3: f64 = 0.99229;
    const CONST4: f64 = 0.04481;
    const VERY_MIN: f64 = f64::MIN * 0.25;
    const LOG_V_MIN: f64 = LN2 * (MIN_EXP - 2.);
    const FPU: f64 = 3e-308;
    const P_HI: f64 = 1. - 2.22e-16;
    const P_LO: f64 = FPU;

    let give_log_q = log_q_cut == f64::INFINITY;
    let mut use_log_x = give_log_q;
    let mut add_n_step = true;

    if alpha == r_dt_0(lower_tail, log_p) {
        return_q_0(give_log_q)
    } else if alpha == r_dt_1(lower_tail, log_p) {
        return_q_0(give_log_q)
    } else if (log_p && alpha > 0.0) || (!log_p && (alpha < 0.0 || alpha > 1.0)) {
        (f64::NAN, f64::NAN)
    } else if p == 0.0 || q == 0.0 || !p.is_finite() || !q.is_finite() {
        if p == 0.0 && q == 0.0 {
            if alpha < r_d_half(log_p) {
                return_q_0(give_log_q)
            } else if alpha > r_d_half(give_log_q) {
                return_q_1(give_log_q)
            } else {
                return_q_half(give_log_q)
            }
        } else if p == 0.0 || p / q == 0.0 {
            return_q_0(give_log_q)
        } else if q == 0.0 || q / p == 0.0 {
            return_q_1(give_log_q)
        } else {
            return_q_half(give_log_q)
        }
    } else {
        let p_ = r_dt_qiv(alpha, lower_tail, log_p);
        let logbeta = lbeta(p, q);
        let swap_tail = p_ > 0.5;
        let (a, la, pp, qq) = if swap_tail {
            (
                r_dt_civ(alpha, lower_tail, log_p),
                r_dt_clog(alpha, lower_tail, log_p),
                q,
                p,
            )
        } else {
            (p_, r_dt_log(alpha, lower_tail, log_p), p, q)
        };

        // Initial approximation
        let acu = 10.0f64
            .powf(-13.0 - 2.5 / (pp * pp) - 0.5 / (a * a))
            .max(1e-300);
        let u0 = (la + pp.ln() + logbeta) / pp;
        let log_eps_c = LN2 * (1.0 - f64::MANTISSA_DIGITS as f64);
        let mut r = pp * (1.0 - qq) / (pp + 1.0);
        let t = 0.2;

        let mut tx = 0.; // dummy value
        let (xinbta, u) = if (LN2 * MIN_EXP) < u0
            && u0 < -0.01
            && u0
                < (t * log_eps_c
                    - (pp * (1.0 - qq) * (2.0 - qq) / (2.0 * (pp + 2.0)))
                        .abs()
                        .ln()
                        * 0.5)
        {
            r = r * u0.exp();
            let u = if r > -1.0 { u0 - r.ln_1p() / pp } else { u0 };
            tx = u.exp();
            let xinbta = u.exp();
            use_log_x = true;
            (xinbta, u)
        } else {
            let r = (-2.0 * la).sqrt();
            let y = r - (CONST1 + CONST2 * r) / (1. + (CONST3 + CONST4 * r) * r);
            let (xinbta, u) = if pp > 1. && qq > 1. {
                let r = (y * y - 3.) / 6.;
                let s = (pp + pp - 1.).recip();
                let t = (qq + qq - 1.).recip();
                let h = 2. / (s + t);
                let w = y * (h + r).sqrt() / h - (t - s) * (r + 5. / 6. - 2. / (3. + h));
                if w > 300. {
                    let t = w + w + qq.ln() - pp.ln();
                    let u = if t <= 18. {
                        -t.exp().ln_1p()
                    } else {
                        -t - (-t).exp()
                    };
                    let xinbta = u.exp();
                    (xinbta, u)
                } else {
                    let xinbta = pp / (pp + qq * (w + w).exp());
                    let u = -(qq / pp * (w + w).exp()).ln_1p();
                    (xinbta, u)
                }
            } else {
                let r = qq + qq;
                let t = 1. / (3. * qq.sqrt());
                let t = r * r_pow_di(1. + t * (-t + y), 3);
                let s = 4. * pp + r - 2.;
                if t == 0. || (t < 0. && s >= t) {
                    let l1ma = if swap_tail {
                        r_dt_log(alpha, lower_tail, log_p)
                    } else {
                        r_dt_clog(alpha, lower_tail, log_p)
                    };
                    let xx = (l1ma + qq.ln() + logbeta) / qq;
                    if xx <= 0. {
                        let xinbta = -xx.exp_m1();
                        let u = r_log1_exp(xx);
                        (xinbta, u)
                    } else {
                        (0., f64::NEG_INFINITY)
                    }
                } else {
                    let t = s / t;
                    if t <= 1. {
                        let u = (la + pp.ln() + logbeta) / pp;
                        let xinbta = u.exp();
                        (xinbta, u)
                    } else {
                        let xinbta = 1. - 2. / (t + 1.);
                        let u = (-2. / (t + 1.)).ln_1p();
                        (xinbta, u)
                    }
                }
            };

            if (swap_tail && u >= -log_q_cut.exp())
                || (!swap_tail && u >= -(4. * log_q_cut).exp() && pp / qq < 1000.)
            {
                swap_tail = !swap_tail;
                if swap_tail {
                    a = r_dt_civ(alpha, lower_tail, log_p);
                    la = r_dt_clog(alpha, lower_tail, log_p);
                    pp = q;
                    qq = p;
                } else {
                    a = p_;
                    la = r_dt_log(alpha, lower_tail, log_p);
                    pp = p;
                    qq = q;
                }
                u = r_log1_exp(u);
                xinbta = u.exp();
            }

            if !use_log_x {
                use_log_x = u < log_q_cut;
            }
            let bad_u = !u.is_finite();
            let bad_init = bad_u || xinbta > P_HI;
            let mut u_n = 1.;
            tx = xinbta;

            if bad_u || u < log_q_cut {
                let w = pbeta(VERY_MIN, pp, qq, true, log_p);
                let test = if log_p { la } else { a };
                if w > test {
                    if log_p || (w - a).abs() < (0. - a).abs() {
                        tx = VERY_MIN;
                        u_n = LOG_V_MIN;
                    } else {
                        tx = 0.;
                        u_n = f64::NEG_INFINITY;
                    };
                    use_log_x = log_p;
                    add_n_step = false;
                    return todo!();
                } else {
                    if u < LOG_V_MIN {
                        u = LOG_V_MIN;
                        xinbta = VERY_MIN;
                    }
                }
            }

            if bad_init && !(use_log_x && tx > 0.) {
                if u == f64::NEG_INFINITY {
                    u = LN2 * MIN_EXP;
                    xinbta = f64::MIN;
                } else {
                    xinbta = if xinbta > 1.1 {
                        0.5
                    } else {
                        if xinbta < P_LO {
                            u.exp()
                        } else {
                            P_HI
                        }
                    };
                    if bad_u {
                        u = xinbta.ln();
                    }
                }
            }
            (xinbta, u)
        };

        // start of newton
        let r = 1. - pp;
        let t = 1. - qq;
        let mut wprev = 0.;
        let mut prev = 1.;
        let mut adj: f64 = 1.;

        if use_log_x {
            'outer: for i_pb in 0..1_000 {
                let y = pbeta(xinbta, pp, qq, true, true);
                let w = if y == f64::NEG_INFINITY {
                    0.
                } else {
                    (y - la) * (y - u + logbeta + r * u + t * r_log1_exp(u)).exp()
                };
                if !w.is_finite() {
                    break;
                }
                if i_pb >= n_N && w * wprev <= 0. {
                    prev = adj.abs().max(FPU);
                }
                let g = 1.;
                for i_inn in 0..1_000 {
                    adj = g * w;
                    if adj.abs() < prev {
                        u_n = u - adj;
                        if u_n <= 0. {
                            if prev <= -acu || w.abs() <= acu {
                                break 'outer;
                            }
                            break;
                        }
                    }
                    g /= 3.;
                }
                let D = adj.abs().min((u_n - u).abs());
                if D <= 4e-16 * (u_n + u).abs() {
                    break 'outer;
                }
                u = u_n;
                xinbta = u.exp();
                wprev = w;
            }
        } else {
            'outer: for i_pb in 0..1_000 {
                let y = pbeta(xinbta, pp, qq, true, log_p);
                if !y.is_finite() && !(log_p && y == f64::NEG_INFINITY) {
                    return (f64::NAN, f64::NAN);
                }
                let w = if log_p {
                    (y - la) * (y + logbeta + r * xinbta.ln() + t * (-xinbta).ln_1p()).exp()
                } else {
                    (y - a) * (logbeta + r * xinbta.ln() + t * (-xinbta).ln_1p()).exp()
                };
                if i_pb >= n_N && w * wprev <= 0. {
                    prev = adj.abs().max(FPU);
                }
                let g = 1.;
                for i_inn in 0..1000 {
                    let adj = g * w;
                    if i_pb < n_N || adj.abs() < prev {
                        let tx = xinbta - adj;
                        if 0. <= tx && tx <= 1. {
                            if prev <= acu || w.abs() <= acu {
                                break 'outer;
                            }
                            if tx != 0. && tx != 1. {
                                break;
                            }
                        }
                    }
                    g /= 3.;
                }
                if (tx - xinbta).abs() <= 4e-16 * (tx + xinbta) {
                    break;
                }
                if tx == 0. {
                    break;
                }
                wprev = w;
            }
        }

        let log_ = log_p || use_log_x;
        if (log_ && y == f64::NEG_INFINITY) || (!log_ && y == 0.) {
            let w = pbeta(VERY_MIN, pp, qq, true, log_);
            if log_ || (w - a).abs() <= (y - a).abs() {
                tx = VERY_MIN;
                u_n = LOG_V_MIN;
            }
            add_N_step = false;
        }

        if give_log_qq {
            let r = r_log1_exp(u_n);
            if swap_tail {
                (r, u_n)
            } else {
                (u_n, r)
            }
        } else {
            if use_log_x {
                if add_N_step {
                    xinbta = u_n.exp();
                    let y = pbeta(xinbta, pp, qq, true, log_p);
                    let w = if log_p {
                        (y - la) * (y + logbeta + r * xinbta.ln() + t * (-xinbta).ln_1p()).exp()
                    } else {
                        (y - a) * (logbeta + r * xinbta.ln() + t * (-xinbta).ln_1p()).exp()
                    };
                    tx = xinbta - w;
                } else {
                    return if swap_tail {
                        (-u_n.exp_m1(), u_n.exp())
                    } else {
                        (u_n.exp(), -u_n.exp_m1())
                    };
                }
            }
            if swap_tail {
                (1. - tx, tx)
            } else {
                (tx, 1. - tx)
            }
        }
    }
}
