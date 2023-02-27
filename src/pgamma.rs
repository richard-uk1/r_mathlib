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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn dnorm4(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn dpois_raw(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
static mut scalefactor: libc::c_double = 4294967296.0f64
    * 4294967296.0f64
    * (4294967296.0f64 * 4294967296.0f64)
    * (4294967296.0f64 * 4294967296.0f64 * (4294967296.0f64 * 4294967296.0f64));
static mut M_cutoff: libc::c_double = 0.693147180559945309417232121458f64
    * 1024 as libc::c_int as libc::c_double
    / 2.2204460492503131e-16f64;
unsafe extern "C" fn logcf(
    mut x: libc::c_double,
    mut i: libc::c_double,
    mut d: libc::c_double,
    mut eps: libc::c_double,
) -> libc::c_double {
    let mut c1: libc::c_double = 2 as libc::c_int as libc::c_double * d;
    let mut c2: libc::c_double = i + d;
    let mut c4: libc::c_double = c2 + d;
    let mut a1: libc::c_double = c2;
    let mut b1: libc::c_double = i * (c2 - i * x);
    let mut b2: libc::c_double = d * d * x;
    let mut a2: libc::c_double = c4 * c2 - b2;
    b2 = c4 * b1 - i * b2;
    while fabs(a2 * b1 - a1 * b2) > fabs(eps * b1 * b2) {
        let mut c3: libc::c_double = c2 * c2 * x;
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
        } else if fabs(b2) < 1 as libc::c_int as libc::c_double / scalefactor {
            a1 *= scalefactor;
            b1 *= scalefactor;
            a2 *= scalefactor;
            b2 *= scalefactor;
        }
    }
    return a2 / b2;
}
#[no_mangle]
pub unsafe extern "C" fn log1pmx(mut x: libc::c_double) -> libc::c_double {
    static mut minLog1Value: libc::c_double = -0.79149064f64;
    if x > 1 as libc::c_int as libc::c_double || x < minLog1Value {
        return Rlog1p(x) - x;
    } else {
        let mut r: libc::c_double = x / (2 as libc::c_int as libc::c_double + x);
        let mut y: libc::c_double = r * r;
        if fabs(x) < 1e-2f64 {
            static mut two: libc::c_double = 2 as libc::c_int as libc::c_double;
            return r
                * ((((two / 9 as libc::c_int as libc::c_double * y
                    + two / 7 as libc::c_int as libc::c_double)
                    * y
                    + two / 5 as libc::c_int as libc::c_double)
                    * y
                    + two / 3 as libc::c_int as libc::c_double)
                    * y
                    - x);
        } else {
            static mut tol_logcf: libc::c_double = 1e-14f64;
            return r
                * (2 as libc::c_int as libc::c_double
                    * y
                    * logcf(
                        y,
                        3 as libc::c_int as libc::c_double,
                        2 as libc::c_int as libc::c_double,
                        tol_logcf,
                    )
                    - x);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lgamma1p(mut a: libc::c_double) -> libc::c_double {
    let eulers_const: libc::c_double = 0.5772156649015328606065120900824024f64;
    let N: libc::c_int = 40 as libc::c_int;
    static mut coeffs: [libc::c_double; 40] = [
        0.3224670334241132182362075833230126e-0f64,
        0.6735230105319809513324605383715000e-1f64,
        0.2058080842778454787900092413529198e-1f64,
        0.7385551028673985266273097291406834e-2f64,
        0.2890510330741523285752988298486755e-2f64,
        0.1192753911703260977113935692828109e-2f64,
        0.5096695247430424223356548135815582e-3f64,
        0.2231547584535793797614188036013401e-3f64,
        0.9945751278180853371459589003190170e-4f64,
        0.4492623673813314170020750240635786e-4f64,
        0.2050721277567069155316650397830591e-4f64,
        0.9439488275268395903987425104415055e-5f64,
        0.4374866789907487804181793223952411e-5f64,
        0.2039215753801366236781900709670839e-5f64,
        0.9551412130407419832857179772951265e-6f64,
        0.4492469198764566043294290331193655e-6f64,
        0.2120718480555466586923135901077628e-6f64,
        0.1004322482396809960872083050053344e-6f64,
        0.4769810169363980565760193417246730e-7f64,
        0.2271109460894316491031998116062124e-7f64,
        0.1083865921489695409107491757968159e-7f64,
        0.5183475041970046655121248647057669e-8f64,
        0.2483674543802478317185008663991718e-8f64,
        0.1192140140586091207442548202774640e-8f64,
        0.5731367241678862013330194857961011e-9f64,
        0.2759522885124233145178149692816341e-9f64,
        0.1330476437424448948149715720858008e-9f64,
        0.6422964563838100022082448087644648e-10f64,
        0.3104424774732227276239215783404066e-10f64,
        0.1502138408075414217093301048780668e-10f64,
        0.7275974480239079662504549924814047e-11f64,
        0.3527742476575915083615072228655483e-11f64,
        0.1711991790559617908601084114443031e-11f64,
        0.8315385841420284819798357793954418e-12f64,
        0.4042200525289440065536008957032895e-12f64,
        0.1966475631096616490411045679010286e-12f64,
        0.9573630387838555763782200936508615e-13f64,
        0.4664076026428374224576492565974577e-13f64,
        0.2273736960065972320633279596737272e-13f64,
        0.1109139947083452201658320007192334e-13f64,
    ];
    let c: libc::c_double = 0.2273736845824652515226821577978691e-12f64;
    let tol_logcf: libc::c_double = 1e-14f64;
    let mut lgam: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if fabs(a) >= 0.5f64 {
        return lgammafn(a + 1 as libc::c_int as libc::c_double);
    }
    lgam = c * logcf(
        -a / 2 as libc::c_int as libc::c_double,
        (N + 2 as libc::c_int) as libc::c_double,
        1 as libc::c_int as libc::c_double,
        tol_logcf,
    );
    i = N - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        lgam = coeffs[i as usize] - a * lgam;
        i -= 1;
    }
    return (a * lgam - eulers_const) * a - log1pmx(a);
}
#[no_mangle]
pub unsafe extern "C" fn logspace_add(
    mut logx: libc::c_double,
    mut logy: libc::c_double,
) -> libc::c_double {
    return fmax2(logx, logy) + Rlog1p(exp(-fabs(logx - logy)));
}
#[no_mangle]
pub unsafe extern "C" fn logspace_sub(
    mut logx: libc::c_double,
    mut logy: libc::c_double,
) -> libc::c_double {
    return logx
        + (if logy - logx > -0.693147180559945309417232121458f64 {
            log(-expm1(logy - logx))
        } else {
            Rlog1p(-exp(logy - logx))
        });
}
#[no_mangle]
pub unsafe extern "C" fn logspace_sum(
    mut logx: *const libc::c_double,
    mut n: libc::c_int,
) -> libc::c_double {
    if n == 0 as libc::c_int {
        return -1.0f64 / 0.0f64;
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
    let mut Mx: libc::c_double = *logx.offset(0 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < n {
        if Mx < *logx.offset(i as isize) {
            Mx = *logx.offset(i as isize);
        }
        i += 1;
    }
    let mut s: libc::c_double = 0.0f64;
    i = 0 as libc::c_int;
    while i < n {
        s += exp(*logx.offset(i as isize) - Mx);
        i += 1;
    }
    return Mx + log(s);
}
unsafe extern "C" fn dpois_wrap(
    mut x_plus_1: libc::c_double,
    mut lambda: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if R_finite(lambda) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x_plus_1 > 1 as libc::c_int as libc::c_double {
        return dpois_raw(x_plus_1 - 1 as libc::c_int as libc::c_double, lambda, log_p);
    }
    if lambda > fabs(x_plus_1 - 1 as libc::c_int as libc::c_double) * M_cutoff {
        return if log_p != 0 {
            -lambda - lgammafn(x_plus_1)
        } else {
            exp(-lambda - lgammafn(x_plus_1))
        };
    } else {
        let mut d: libc::c_double = dpois_raw(x_plus_1, lambda, log_p);
        return if log_p != 0 {
            d + log(x_plus_1 / lambda)
        } else {
            d * (x_plus_1 / lambda)
        };
    };
}
unsafe extern "C" fn pgamma_smallx(
    mut x: libc::c_double,
    mut alph: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut c: libc::c_double = alph;
    let mut n: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut term: libc::c_double = 0.;
    loop {
        n += 1.;
        c *= -x / n;
        term = c / (alph + n);
        sum += term;
        if !(fabs(term) > 2.2204460492503131e-16f64 * fabs(sum)) {
            break;
        }
    }
    if lower_tail != 0 {
        let mut f1: libc::c_double = if log_p != 0 {
            Rlog1p(sum)
        } else {
            1 as libc::c_int as libc::c_double + sum
        };
        let mut f2: libc::c_double = 0.;
        if alph > 1 as libc::c_int as libc::c_double {
            f2 = dpois_raw(alph, x, log_p);
            f2 = if log_p != 0 { f2 + x } else { f2 * exp(x) };
        } else if log_p != 0 {
            f2 = alph * log(x) - lgamma1p(alph);
        } else {
            f2 = pow(x, alph) / exp(lgamma1p(alph));
        }
        return if log_p != 0 { f1 + f2 } else { f1 * f2 };
    } else {
        let mut lf2: libc::c_double = alph * log(x) - lgamma1p(alph);
        if log_p != 0 {
            return if Rlog1p(sum) + lf2 > -0.693147180559945309417232121458f64 {
                log(-expm1(Rlog1p(sum) + lf2))
            } else {
                Rlog1p(-exp(Rlog1p(sum) + lf2))
            };
        } else {
            let mut f1m1: libc::c_double = sum;
            let mut f2m1: libc::c_double = expm1(lf2);
            return -(f1m1 + f2m1 + f1m1 * f2m1);
        }
    };
}
unsafe extern "C" fn pd_upper_series(
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut term: libc::c_double = x / y;
    let mut sum: libc::c_double = term;
    loop {
        y += 1.;
        term *= x / y;
        sum += term;
        if !(term > sum * 2.2204460492503131e-16f64) {
            break;
        }
    }
    return if log_p != 0 { log(sum) } else { sum };
}
unsafe extern "C" fn pd_lower_cf(mut y: libc::c_double, mut d: libc::c_double) -> libc::c_double {
    let mut f: libc::c_double = 0.0f64;
    let mut of: libc::c_double = 0.;
    let mut f0: libc::c_double = 0.;
    let mut i: libc::c_double = 0.;
    let mut c2: libc::c_double = 0.;
    let mut c3: libc::c_double = 0.;
    let mut c4: libc::c_double = 0.;
    let mut a1: libc::c_double = 0.;
    let mut b1: libc::c_double = 0.;
    let mut a2: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    if y == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    f0 = y / d;
    if fabs(y - 1 as libc::c_int as libc::c_double) < fabs(d) * 2.2204460492503131e-16f64 {
        return f0;
    }
    if f0 > 1.0f64 {
        f0 = 1.0f64;
    }
    c2 = y;
    c4 = d;
    a1 = 0 as libc::c_int as libc::c_double;
    b1 = 1 as libc::c_int as libc::c_double;
    a2 = y;
    b2 = d;
    while b2 > scalefactor {
        a1 /= scalefactor;
        b1 /= scalefactor;
        a2 /= scalefactor;
        b2 /= scalefactor;
    }
    i = 0 as libc::c_int as libc::c_double;
    of = -1.0f64;
    while i < 200000 as libc::c_int as libc::c_double {
        i += 1.;
        c2 -= 1.;
        c3 = i * c2;
        c4 += 2 as libc::c_int as libc::c_double;
        a1 = c4 * a2 + c3 * a1;
        b1 = c4 * b2 + c3 * b1;
        i += 1.;
        c2 -= 1.;
        c3 = i * c2;
        c4 += 2 as libc::c_int as libc::c_double;
        a2 = c4 * a1 + c3 * a2;
        b2 = c4 * b1 + c3 * b2;
        if b2 > scalefactor {
            a1 /= scalefactor;
            b1 /= scalefactor;
            a2 /= scalefactor;
            b2 /= scalefactor;
        }
        if b2 != 0 as libc::c_int as libc::c_double {
            f = a2 / b2;
            if fabs(f - of) <= 2.2204460492503131e-16f64 * fmax2(f0, fabs(f)) {
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
unsafe extern "C" fn pd_lower_series(
    mut lambda: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    let mut term: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    while y >= 1 as libc::c_int as libc::c_double && term > sum * 2.2204460492503131e-16f64 {
        term *= y / lambda;
        sum += term;
        y -= 1.;
    }
    if y != floor(y) {
        let mut f: libc::c_double = 0.;
        f = pd_lower_cf(y, lambda + 1 as libc::c_int as libc::c_double - y);
        sum += term * f;
    }
    return sum;
}
unsafe extern "C" fn dpnorm(
    mut x: libc::c_double,
    mut lower_tail: libc::c_int,
    mut lp: libc::c_double,
) -> libc::c_double {
    if x < 0 as libc::c_int as libc::c_double {
        x = -x;
        lower_tail = (lower_tail == 0) as libc::c_int;
    }
    if x > 10 as libc::c_int as libc::c_double && lower_tail == 0 {
        let mut term: libc::c_double = 1 as libc::c_int as libc::c_double / x;
        let mut sum: libc::c_double = term;
        let mut x2: libc::c_double = x * x;
        let mut i: libc::c_double = 1 as libc::c_int as libc::c_double;
        loop {
            term *= -i / x2;
            sum += term;
            i += 2 as libc::c_int as libc::c_double;
            if !(fabs(term) > 2.2204460492503131e-16f64 * sum) {
                break;
            }
        }
        return 1 as libc::c_int as libc::c_double / sum;
    } else {
        let mut d: libc::c_double = dnorm4(x, 0.0f64, 1.0f64, FALSE as libc::c_int);
        return d / exp(lp);
    };
}
unsafe extern "C" fn ppois_asymp(
    mut x: libc::c_double,
    mut lambda: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut coefs_a: [libc::c_double; 8] = [
        -1e99f64,
        2 as libc::c_int as libc::c_double / 3.0f64,
        -(4 as libc::c_int) as libc::c_double / 135.0f64,
        8 as libc::c_int as libc::c_double / 2835.0f64,
        16 as libc::c_int as libc::c_double / 8505.0f64,
        -(8992 as libc::c_int) as libc::c_double / 12629925.0f64,
        -(334144 as libc::c_int) as libc::c_double / 492567075.0f64,
        698752 as libc::c_int as libc::c_double / 1477701225.0f64,
    ];
    static mut coefs_b: [libc::c_double; 8] = [
        -1e99f64,
        1 as libc::c_int as libc::c_double / 12.0f64,
        1 as libc::c_int as libc::c_double / 288.0f64,
        -(139 as libc::c_int) as libc::c_double / 51840.0f64,
        -(571 as libc::c_int) as libc::c_double / 2488320.0f64,
        163879 as libc::c_int as libc::c_double / 209018880.0f64,
        5246819 as libc::c_int as libc::c_double / 75246796800.0f64,
        -(534703531 as libc::c_int) as libc::c_double / 902961561600.0f64,
    ];
    let mut elfb: libc::c_double = 0.;
    let mut elfb_term: libc::c_double = 0.;
    let mut res12: libc::c_double = 0.;
    let mut res1_term: libc::c_double = 0.;
    let mut res1_ig: libc::c_double = 0.;
    let mut res2_term: libc::c_double = 0.;
    let mut res2_ig: libc::c_double = 0.;
    let mut dfm: libc::c_double = 0.;
    let mut pt_: libc::c_double = 0.;
    let mut s2pt: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut np: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    dfm = lambda - x;
    pt_ = -log1pmx(dfm / x);
    s2pt = sqrt(2 as libc::c_int as libc::c_double * x * pt_);
    if dfm < 0 as libc::c_int as libc::c_double {
        s2pt = -s2pt;
    }
    res12 = 0 as libc::c_int as libc::c_double;
    res1_term = sqrt(x);
    res1_ig = res1_term;
    res2_term = s2pt;
    res2_ig = res2_term;
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        res12 += res1_ig * coefs_a[i as usize];
        res12 += res2_ig * coefs_b[i as usize];
        res1_term *= pt_ / i as libc::c_double;
        res2_term *= 2 as libc::c_int as libc::c_double * pt_
            / (2 as libc::c_int * i + 1 as libc::c_int) as libc::c_double;
        res1_ig = res1_ig / x + res1_term;
        res2_ig = res2_ig / x + res2_term;
        i += 1;
    }
    elfb = x;
    elfb_term = 1 as libc::c_int as libc::c_double;
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
    np = pnorm5(
        s2pt,
        0.0f64,
        1.0f64,
        (lower_tail == 0) as libc::c_int,
        log_p,
    );
    if log_p != 0 {
        let mut n_d_over_p: libc::c_double = dpnorm(s2pt, (lower_tail == 0) as libc::c_int, np);
        return np + Rlog1p(f * n_d_over_p);
    } else {
        let mut nd: libc::c_double = dnorm4(s2pt, 0.0f64, 1.0f64, log_p);
        return np + f * nd;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Rf_pgamma_raw(
    mut x: libc::c_double,
    mut alph: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut res: libc::c_double = 0.;
    if x <= 0.0f64 {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    if x >= 1.0f64 / 0.0f64 {
        return if lower_tail != 0 {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if x < 1 as libc::c_int as libc::c_double {
        res = pgamma_smallx(x, alph, lower_tail, log_p);
    } else if x <= alph - 1 as libc::c_int as libc::c_double
        && x < 0.8f64 * (alph + 50 as libc::c_int as libc::c_double)
    {
        let mut sum: libc::c_double = pd_upper_series(x, alph, log_p);
        let mut d: libc::c_double = dpois_wrap(alph, x, log_p);
        if lower_tail == 0 {
            res = if log_p != 0 {
                if d + sum > -0.693147180559945309417232121458f64 {
                    log(-expm1(d + sum))
                } else {
                    Rlog1p(-exp(d + sum))
                }
            } else {
                1 as libc::c_int as libc::c_double - d * sum
            };
        } else {
            res = if log_p != 0 { sum + d } else { sum * d };
        }
    } else if (alph - 1 as libc::c_int as libc::c_double) < x
        && alph < 0.8f64 * (x + 50 as libc::c_int as libc::c_double)
    {
        let mut sum_0: libc::c_double = 0.;
        let mut d_0: libc::c_double = dpois_wrap(alph, x, log_p);
        if alph < 1 as libc::c_int as libc::c_double {
            if x * 2.2204460492503131e-16f64 > 1 as libc::c_int as libc::c_double - alph {
                sum_0 = if log_p != 0 { 0.0f64 } else { 1.0f64 };
            } else {
                let mut f: libc::c_double =
                    pd_lower_cf(alph, x - (alph - 1 as libc::c_int as libc::c_double)) * x / alph;
                sum_0 = if log_p != 0 { log(f) } else { f };
            }
        } else {
            sum_0 = pd_lower_series(x, alph - 1 as libc::c_int as libc::c_double);
            sum_0 = if log_p != 0 {
                Rlog1p(sum_0)
            } else {
                1 as libc::c_int as libc::c_double + sum_0
            };
        }
        if lower_tail == 0 {
            res = if log_p != 0 { sum_0 + d_0 } else { sum_0 * d_0 };
        } else {
            res = if log_p != 0 {
                if d_0 + sum_0 > -0.693147180559945309417232121458f64 {
                    log(-expm1(d_0 + sum_0))
                } else {
                    Rlog1p(-exp(d_0 + sum_0))
                }
            } else {
                1 as libc::c_int as libc::c_double - d_0 * sum_0
            };
        }
    } else {
        res = ppois_asymp(
            alph - 1 as libc::c_int as libc::c_double,
            x,
            (lower_tail == 0) as libc::c_int,
            log_p,
        );
    }
    if log_p == 0 && res < 2.2250738585072014e-308f64 / 2.2204460492503131e-16f64 {
        return exp(Rf_pgamma_raw(x, alph, lower_tail, 1 as libc::c_int));
    } else {
        return res;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pgamma(
    mut x: libc::c_double,
    mut alph: libc::c_double,
    mut scale: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || alph.is_nan() as i32 != 0 as libc::c_int
        || scale.is_nan() as i32 != 0 as libc::c_int
    {
        return x + alph + scale;
    }
    if alph < 0.0f64 || scale <= 0.0f64 {
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
    x /= scale;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if alph == 0.0f64 {
        return if x <= 0 as libc::c_int as libc::c_double {
            if lower_tail != 0 {
                if log_p != 0 {
                    -1.0f64 / 0.0f64
                } else {
                    0.0f64
                }
            } else if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if lower_tail != 0 {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    return Rf_pgamma_raw(x, alph, lower_tail, log_p);
}
