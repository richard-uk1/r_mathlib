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
    fn exp(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn expm1(_: f64) -> f64;
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn floor(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn Rlog1p(_: f64) -> f64;
    fn R_finite(_: f64) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dnorm4(_: f64, _: f64, _: f64, _: libc::c_int) -> f64;
    fn pnorm5(_: f64, _: f64, _: f64, _: libc::c_int, _: libc::c_int) -> f64;
    fn dpois_raw(_: f64, _: f64, _: libc::c_int) -> f64;
    fn lgammafn(_: f64) -> f64;
    fn fmax2(_: f64, _: f64) -> f64;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
const scalefactor: f64 = 4294967296.0
    * 4294967296.0
    * (4294967296.0 * 4294967296.0)
    * (4294967296.0 * 4294967296.0 * (4294967296.0 * 4294967296.0));
const M_cutoff: f64 = 0.693147180559945309417232121458 * 1024. / 2.2204460492503131e-16;

unsafe extern "C" fn logcf(mut x: f64, mut i: f64, mut d: f64, mut eps: f64) -> f64 {
    let mut c1: f64 = 2 as libc::c_int as f64 * d;
    let mut c2: f64 = i + d;
    let mut c4: f64 = c2 + d;
    let mut a1: f64 = c2;
    let mut b1: f64 = i * (c2 - i * x);
    let mut b2: f64 = d * d * x;
    let mut a2: f64 = c4 * c2 - b2;
    b2 = c4 * b1 - i * b2;
    while fabs(a2 * b1 - a1 * b2) > fabs(eps * b1 * b2) {
        let mut c3: f64 = c2 * c2 * x;
        c2 += d;
        c4 += d;
        a1 = c4 * a2 - c3 * a1;
        b1 = c4 * b2 - c3 * b1;
        c3 = c1 * c1 * x;
        c1 += d;
        c4 += d;
        a2 = c4 * a1 - c3 * a2;
        b2 = c4 * b1 - c3 * b2;
        if fabs(b2) > scalefactor {
            a1 /= scalefactor;
            b1 /= scalefactor;
            a2 /= scalefactor;
            b2 /= scalefactor;
        } else if fabs(b2) < 1 as libc::c_int as f64 / scalefactor {
            a1 *= scalefactor;
            b1 *= scalefactor;
            a2 *= scalefactor;
            b2 *= scalefactor;
        }
    }
    return a2 / b2;
}

#[no_mangle]
pub unsafe extern "C" fn log1pmx(mut x: f64) -> f64 {
    static mut minLog1Value: f64 = -0.79149064;
    if x > 1 as libc::c_int as f64 || x < minLog1Value {
        return Rlog1p(x) - x;
    } else {
        let mut r: f64 = x / (2 as libc::c_int as f64 + x);
        let mut y: f64 = r * r;
        if fabs(x) < 1e-2 {
            static mut two: f64 = 2 as libc::c_int as f64;
            return r
                * ((((two / 9 as libc::c_int as f64 * y + two / 7 as libc::c_int as f64) * y
                    + two / 5 as libc::c_int as f64)
                    * y
                    + two / 3 as libc::c_int as f64)
                    * y
                    - x);
        } else {
            static mut tol_logcf: f64 = 1e-14;
            return r
                * (2 as libc::c_int as f64
                    * y
                    * logcf(
                        y,
                        3 as libc::c_int as f64,
                        2 as libc::c_int as f64,
                        tol_logcf,
                    )
                    - x);
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn lgamma1p(mut a: f64) -> f64 {
    let eulers_const: f64 = 0.5772156649015328606065120900824024;
    let N: libc::c_int = 40 as libc::c_int;
    static mut coeffs: [f64; 40] = [
        0.3224670334241132182362075833230126e-0,
        0.6735230105319809513324605383715000e-1,
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
        0.1109139947083452201658320007192334e-13,
    ];
    let c: f64 = 0.2273736845824652515226821577978691e-12;
    let tol_logcf: f64 = 1e-14;
    let mut lgam: f64 = 0.;
    let mut i: libc::c_int = 0;
    if fabs(a) >= 0.5 {
        return lgammafn(a + 1 as libc::c_int as f64);
    }
    lgam = c * logcf(
        -a / 2 as libc::c_int as f64,
        (N + 2 as libc::c_int) as f64,
        1 as libc::c_int as f64,
        tol_logcf,
    );
    i = N - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        lgam = coeffs[i as usize] - a * lgam;
        i -= 1;
    }
    return (a * lgam - eulers_const) * a - log1pmx(a);
}

pub fn logspace_add(mut logx: f64, mut logy: f64) -> f64 {
    logx.max(logy) + (-(logx - logy).abs()).exp().ln_1p()
}

#[no_mangle]
pub unsafe extern "C" fn logspace_sub(mut logx: f64, mut logy: f64) -> f64 {
    return logx
        + (if logy - logx > -0.693147180559945309417232121458 {
            log(-expm1(logy - logx))
        } else {
            Rlog1p(-exp(logy - logx))
        });
}

#[no_mangle]
pub unsafe extern "C" fn logspace_sum(mut logx: *const f64, mut n: libc::c_int) -> f64 {
    if n == 0 as libc::c_int {
        return -1.0 / 0.0;
    }
    if n == 1 as libc::c_int {
        return *logx.offset(0 as libc::c_int as isize);
    }
    if n == 2 as libc::c_int {
        return logspace_add(
            *logx.offset(0 as libc::c_int as isize),
            *logx.offset(1 as libc::c_int as isize),
        );
    }
    let mut i: libc::c_int = 0;
    let mut Mx: f64 = *logx.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < n {
        if Mx < *logx.offset(i as isize) {
            Mx = *logx.offset(i as isize);
        }
        i += 1;
    }
    let mut s: f64 = 0.0;
    i = 0 as libc::c_int;
    while i < n {
        s += exp(*logx.offset(i as isize) - Mx);
        i += 1;
    }
    return Mx + log(s);
}
unsafe extern "C" fn dpois_wrap(mut x_plus_1: f64, mut lambda: f64, mut log_p: libc::c_int) -> f64 {
    if R_finite(lambda) == 0 {
        return if log_p != 0 { -1.0 / 0.0 } else { 0.0 };
    }
    if x_plus_1 > 1 as libc::c_int as f64 {
        return dpois_raw(x_plus_1 - 1 as libc::c_int as f64, lambda, log_p);
    }
    if lambda > fabs(x_plus_1 - 1 as libc::c_int as f64) * M_cutoff {
        return if log_p != 0 {
            -lambda - lgammafn(x_plus_1)
        } else {
            exp(-lambda - lgammafn(x_plus_1))
        };
    } else {
        let mut d: f64 = dpois_raw(x_plus_1, lambda, log_p);
        return if log_p != 0 {
            d + log(x_plus_1 / lambda)
        } else {
            d * (x_plus_1 / lambda)
        };
    };
}
unsafe extern "C" fn pgamma_smallx(
    mut x: f64,
    mut alph: f64,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> f64 {
    let mut sum: f64 = 0 as libc::c_int as f64;
    let mut c: f64 = alph;
    let mut n: f64 = 0 as libc::c_int as f64;
    let mut term: f64 = 0.;
    loop {
        n += 1.;
        c *= -x / n;
        term = c / (alph + n);
        sum += term;
        if !(fabs(term) > 2.2204460492503131e-16 * fabs(sum)) {
            break;
        }
    }
    if lower_tail != 0 {
        let mut f1: f64 = if log_p != 0 {
            Rlog1p(sum)
        } else {
            1 as libc::c_int as f64 + sum
        };
        let mut f2: f64 = 0.;
        if alph > 1 as libc::c_int as f64 {
            f2 = dpois_raw(alph, x, log_p);
            f2 = if log_p != 0 { f2 + x } else { f2 * exp(x) };
        } else if log_p != 0 {
            f2 = alph * log(x) - lgamma1p(alph);
        } else {
            f2 = pow(x, alph) / exp(lgamma1p(alph));
        }
        return if log_p != 0 { f1 + f2 } else { f1 * f2 };
    } else {
        let mut lf2: f64 = alph * log(x) - lgamma1p(alph);
        if log_p != 0 {
            return if Rlog1p(sum) + lf2 > -0.693147180559945309417232121458 {
                log(-expm1(Rlog1p(sum) + lf2))
            } else {
                Rlog1p(-exp(Rlog1p(sum) + lf2))
            };
        } else {
            let mut f1m1: f64 = sum;
            let mut f2m1: f64 = expm1(lf2);
            return -(f1m1 + f2m1 + f1m1 * f2m1);
        }
    };
}
unsafe extern "C" fn pd_upper_series(mut x: f64, mut y: f64, mut log_p: libc::c_int) -> f64 {
    let mut term: f64 = x / y;
    let mut sum: f64 = term;
    loop {
        y += 1.;
        term *= x / y;
        sum += term;
        if !(term > sum * 2.2204460492503131e-16) {
            break;
        }
    }
    return if log_p != 0 { log(sum) } else { sum };
}
unsafe extern "C" fn pd_lower_cf(mut y: f64, mut d: f64) -> f64 {
    let mut f: f64 = 0.0;
    let mut of: f64 = 0.;
    let mut f0: f64 = 0.;
    let mut i: f64 = 0.;
    let mut c2: f64 = 0.;
    let mut c3: f64 = 0.;
    let mut c4: f64 = 0.;
    let mut a1: f64 = 0.;
    let mut b1: f64 = 0.;
    let mut a2: f64 = 0.;
    let mut b2: f64 = 0.;
    if y == 0 as libc::c_int as f64 {
        return 0 as libc::c_int as f64;
    }
    f0 = y / d;
    if fabs(y - 1 as libc::c_int as f64) < fabs(d) * 2.2204460492503131e-16 {
        return f0;
    }
    if f0 > 1.0 {
        f0 = 1.0;
    }
    c2 = y;
    c4 = d;
    a1 = 0 as libc::c_int as f64;
    b1 = 1 as libc::c_int as f64;
    a2 = y;
    b2 = d;
    while b2 > scalefactor {
        a1 /= scalefactor;
        b1 /= scalefactor;
        a2 /= scalefactor;
        b2 /= scalefactor;
    }
    i = 0 as libc::c_int as f64;
    of = -1.0;
    while i < 200000 as libc::c_int as f64 {
        i += 1.;
        c2 -= 1.;
        c3 = i * c2;
        c4 += 2 as libc::c_int as f64;
        a1 = c4 * a2 + c3 * a1;
        b1 = c4 * b2 + c3 * b1;
        i += 1.;
        c2 -= 1.;
        c3 = i * c2;
        c4 += 2 as libc::c_int as f64;
        a2 = c4 * a1 + c3 * a2;
        b2 = c4 * b1 + c3 * b2;
        if b2 > scalefactor {
            a1 /= scalefactor;
            b1 /= scalefactor;
            a2 /= scalefactor;
            b2 /= scalefactor;
        }
        if b2 != 0 as libc::c_int as f64 {
            f = a2 / b2;
            if fabs(f - of) <= 2.2204460492503131e-16 * fmax2(f0, fabs(f)) {
                return f;
            }
            of = f;
        }
    }
    printf(
        b" ** NON-convergence in pgamma()'s pd_lower_cf() f= %g.\n\0" as *const u8
            as *const libc::c_char,
        f,
    );
    return f;
}
unsafe extern "C" fn pd_lower_series(mut lambda: f64, mut y: f64) -> f64 {
    let mut term: f64 = 1 as libc::c_int as f64;
    let mut sum: f64 = 0 as libc::c_int as f64;
    while y >= 1 as libc::c_int as f64 && term > sum * 2.2204460492503131e-16 {
        term *= y / lambda;
        sum += term;
        y -= 1.;
    }
    if y != floor(y) {
        let mut f: f64 = 0.;
        f = pd_lower_cf(y, lambda + 1 as libc::c_int as f64 - y);
        sum += term * f;
    }
    return sum;
}
unsafe extern "C" fn dpnorm(mut x: f64, mut lower_tail: libc::c_int, mut lp: f64) -> f64 {
    if x < 0 as libc::c_int as f64 {
        x = -x;
        lower_tail = (lower_tail == 0) as libc::c_int;
    }
    if x > 10 as libc::c_int as f64 && lower_tail == 0 {
        let mut term: f64 = 1 as libc::c_int as f64 / x;
        let mut sum: f64 = term;
        let mut x2: f64 = x * x;
        let mut i: f64 = 1 as libc::c_int as f64;
        loop {
            term *= -i / x2;
            sum += term;
            i += 2 as libc::c_int as f64;
            if !(fabs(term) > 2.2204460492503131e-16 * sum) {
                break;
            }
        }
        return 1 as libc::c_int as f64 / sum;
    } else {
        let mut d: f64 = dnorm4(x, 0.0, 1.0, FALSE as libc::c_int);
        return d / exp(lp);
    };
}
unsafe extern "C" fn ppois_asymp(
    mut x: f64,
    mut lambda: f64,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> f64 {
    static mut coefs_a: [f64; 8] = [
        -1e99,
        2 as libc::c_int as f64 / 3.0,
        -(4 as libc::c_int) as f64 / 135.0,
        8 as libc::c_int as f64 / 2835.0,
        16 as libc::c_int as f64 / 8505.0,
        -(8992 as libc::c_int) as f64 / 12629925.0,
        -(334144 as libc::c_int) as f64 / 492567075.0,
        698752 as libc::c_int as f64 / 1477701225.0,
    ];
    static mut coefs_b: [f64; 8] = [
        -1e99,
        1 as libc::c_int as f64 / 12.0,
        1 as libc::c_int as f64 / 288.0,
        -(139 as libc::c_int) as f64 / 51840.0,
        -(571 as libc::c_int) as f64 / 2488320.0,
        163879 as libc::c_int as f64 / 209018880.0,
        5246819 as libc::c_int as f64 / 75246796800.0,
        -(534703531 as libc::c_int) as f64 / 902961561600.0,
    ];
    let mut elfb: f64 = 0.;
    let mut elfb_term: f64 = 0.;
    let mut res12: f64 = 0.;
    let mut res1_term: f64 = 0.;
    let mut res1_ig: f64 = 0.;
    let mut res2_term: f64 = 0.;
    let mut res2_ig: f64 = 0.;
    let mut dfm: f64 = 0.;
    let mut pt_: f64 = 0.;
    let mut s2pt: f64 = 0.;
    let mut f: f64 = 0.;
    let mut np: f64 = 0.;
    let mut i: libc::c_int = 0;
    dfm = lambda - x;
    pt_ = -log1pmx(dfm / x);
    s2pt = sqrt(2 as libc::c_int as f64 * x * pt_);
    if dfm < 0 as libc::c_int as f64 {
        s2pt = -s2pt;
    }
    res12 = 0 as libc::c_int as f64;
    res1_term = sqrt(x);
    res1_ig = res1_term;
    res2_term = s2pt;
    res2_ig = res2_term;
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        res12 += res1_ig * coefs_a[i as usize];
        res12 += res2_ig * coefs_b[i as usize];
        res1_term *= pt_ / i as f64;
        res2_term *=
            2 as libc::c_int as f64 * pt_ / (2 as libc::c_int * i + 1 as libc::c_int) as f64;
        res1_ig = res1_ig / x + res1_term;
        res2_ig = res2_ig / x + res2_term;
        i += 1;
    }
    elfb = x;
    elfb_term = 1 as libc::c_int as f64;
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        elfb += elfb_term * coefs_b[i as usize];
        elfb_term /= x;
        i += 1;
    }
    if lower_tail == 0 {
        elfb = -elfb;
    }
    f = res12 / elfb;
    np = pnorm5(s2pt, 0.0, 1.0, (lower_tail == 0) as libc::c_int, log_p);
    if log_p != 0 {
        let mut n_d_over_p: f64 = dpnorm(s2pt, (lower_tail == 0) as libc::c_int, np);
        return np + Rlog1p(f * n_d_over_p);
    } else {
        let mut nd: f64 = dnorm4(s2pt, 0.0, 1.0, log_p);
        return np + f * nd;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Rf_pgamma_raw(
    mut x: f64,
    mut alph: f64,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> f64 {
    let mut res: f64 = 0.;
    if x <= 0.0 {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0 / 0.0
            } else {
                0.0
            }
        } else if log_p != 0 {
            0.0
        } else {
            1.0
        };
    }
    if x >= 1.0 / 0.0 {
        return if lower_tail != 0 {
            if log_p != 0 {
                0.0
            } else {
                1.0
            }
        } else if log_p != 0 {
            -1.0 / 0.0
        } else {
            0.0
        };
    }
    if x < 1 as libc::c_int as f64 {
        res = pgamma_smallx(x, alph, lower_tail, log_p);
    } else if x <= alph - 1 as libc::c_int as f64 && x < 0.8 * (alph + 50 as libc::c_int as f64) {
        let mut sum: f64 = pd_upper_series(x, alph, log_p);
        let mut d: f64 = dpois_wrap(alph, x, log_p);
        if lower_tail == 0 {
            res = if log_p != 0 {
                if d + sum > -0.693147180559945309417232121458 {
                    log(-expm1(d + sum))
                } else {
                    Rlog1p(-exp(d + sum))
                }
            } else {
                1 as libc::c_int as f64 - d * sum
            };
        } else {
            res = if log_p != 0 { sum + d } else { sum * d };
        }
    } else if (alph - 1 as libc::c_int as f64) < x && alph < 0.8 * (x + 50 as libc::c_int as f64) {
        let mut sum_0: f64 = 0.;
        let mut d_0: f64 = dpois_wrap(alph, x, log_p);
        if alph < 1 as libc::c_int as f64 {
            if x * 2.2204460492503131e-16 > 1 as libc::c_int as f64 - alph {
                sum_0 = if log_p != 0 { 0.0 } else { 1.0 };
            } else {
                let mut f: f64 = pd_lower_cf(alph, x - (alph - 1 as libc::c_int as f64)) * x / alph;
                sum_0 = if log_p != 0 { log(f) } else { f };
            }
        } else {
            sum_0 = pd_lower_series(x, alph - 1 as libc::c_int as f64);
            sum_0 = if log_p != 0 {
                Rlog1p(sum_0)
            } else {
                1 as libc::c_int as f64 + sum_0
            };
        }
        if lower_tail == 0 {
            res = if log_p != 0 { sum_0 + d_0 } else { sum_0 * d_0 };
        } else {
            res = if log_p != 0 {
                if d_0 + sum_0 > -0.693147180559945309417232121458 {
                    log(-expm1(d_0 + sum_0))
                } else {
                    Rlog1p(-exp(d_0 + sum_0))
                }
            } else {
                1 as libc::c_int as f64 - d_0 * sum_0
            };
        }
    } else {
        res = ppois_asymp(
            alph - 1 as libc::c_int as f64,
            x,
            (lower_tail == 0) as libc::c_int,
            log_p,
        );
    }
    if log_p == 0 && res < 2.2250738585072014e-308 / 2.2204460492503131e-16 {
        return exp(Rf_pgamma_raw(x, alph, lower_tail, 1 as libc::c_int));
    } else {
        return res;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pgamma(
    mut x: f64,
    mut alph: f64,
    mut scale: f64,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> f64 {
    if x.is_nan() as i32 != 0 as libc::c_int
        || alph.is_nan() as i32 != 0 as libc::c_int
        || scale.is_nan() as i32 != 0 as libc::c_int
    {
        return x + alph + scale;
    }
    if alph < 0.0 || scale <= 0.0 {
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
        return 0.0 / 0.0;
    }
    x /= scale;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if alph == 0.0 {
        return if x <= 0 as libc::c_int as f64 {
            if lower_tail != 0 {
                if log_p != 0 {
                    -1.0 / 0.0
                } else {
                    0.0
                }
            } else if log_p != 0 {
                0.0
            } else {
                1.0
            }
        } else if lower_tail != 0 {
            if log_p != 0 {
                0.0
            } else {
                1.0
            }
        } else if log_p != 0 {
            -1.0 / 0.0
        } else {
            0.0
        };
    }
    return Rf_pgamma_raw(x, alph, lower_tail, log_p);
}
