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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn lgammafn_sign(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    fn lbeta(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_lfastchoose(
    mut n: libc::c_double,
    mut k: libc::c_double,
) -> libc::c_double {
    return -log(n + 1.0f64) - lbeta(n - k + 1.0f64, k + 1.0f64);
}
unsafe extern "C" fn lfastchoose2(
    mut n: libc::c_double,
    mut k: libc::c_double,
    mut s_choose: *mut libc::c_int,
) -> libc::c_double {
    let mut r: libc::c_double = 0.;
    r = lgammafn_sign(n - k + 1.0f64, s_choose);
    return lgammafn(n + 1.0f64) - lgammafn(k + 1.0f64) - r;
}
#[no_mangle]
pub unsafe extern "C" fn lchoose(mut n: libc::c_double, mut k: libc::c_double) -> libc::c_double {
    let mut k0: libc::c_double = k;
    k = round(k);
    if n.is_nan() as i32 != 0 as libc::c_int || k.is_nan() as i32 != 0 as libc::c_int {
        return n + k;
    }
    if fabs(k - k0) > 1e-7f64 {
        printf(
            b"'k' (%.2f) must be integer, rounded to %.0f\0" as *const u8 as *const libc::c_char,
            k0,
            k,
        );
    }
    if k < 2 as libc::c_int as libc::c_double {
        if k < 0 as libc::c_int as libc::c_double {
            return -1.0f64 / 0.0f64;
        }
        if k == 0 as libc::c_int as libc::c_double {
            return 0.0f64;
        }
        return log(fabs(n));
    }
    if n < 0 as libc::c_int as libc::c_double {
        return lchoose(-n + k - 1 as libc::c_int as libc::c_double, k);
    } else {
        if !(fabs(n - round(n)) > 1e-7f64 * fmax2(1.0f64, fabs(n))) {
            n = round(n);
            if n < k {
                return -1.0f64 / 0.0f64;
            }
            if n - k < 2 as libc::c_int as libc::c_double {
                return lchoose(n, n - k);
            }
            return Rf_lfastchoose(n, k);
        }
    }
    if n < k - 1 as libc::c_int as libc::c_double {
        let mut s: libc::c_int = 0;
        return lfastchoose2(n, k, &mut s);
    }
    return Rf_lfastchoose(n, k);
}
#[no_mangle]
pub unsafe extern "C" fn choose(mut n: libc::c_double, mut k: libc::c_double) -> libc::c_double {
    let mut r: libc::c_double = 0.;
    let mut k0: libc::c_double = k;
    k = round(k);
    if n.is_nan() as i32 != 0 as libc::c_int || k.is_nan() as i32 != 0 as libc::c_int {
        return n + k;
    }
    if fabs(k - k0) > 1e-7f64 {
        printf(
            b"'k' (%.2f) must be integer, rounded to %.0f\0" as *const u8 as *const libc::c_char,
            k0,
            k,
        );
    }
    if k < 30 as libc::c_int as libc::c_double {
        let mut j: libc::c_int = 0;
        if n - k < k
            && n >= 0 as libc::c_int as libc::c_double
            && !(fabs(n - round(n)) > 1e-7f64 * fmax2(1.0f64, fabs(n)))
        {
            k = round(n - k);
        }
        if k < 0 as libc::c_int as libc::c_double {
            return 0.0f64;
        }
        if k == 0 as libc::c_int as libc::c_double {
            return 1.0f64;
        }
        r = n;
        j = 2 as libc::c_int;
        while j as libc::c_double <= k {
            r *= (n - j as libc::c_double + 1 as libc::c_int as libc::c_double)
                / j as libc::c_double;
            j += 1;
        }
        return if !(fabs(n - round(n)) > 1e-7f64 * fmax2(1.0f64, fabs(n))) {
            round(r)
        } else {
            r
        };
    }
    if n < 0 as libc::c_int as libc::c_double {
        r = choose(-n + k - 1 as libc::c_int as libc::c_double, k);
        if k != 2 as libc::c_int as libc::c_double * floor(k / 2.0f64) {
            r = -r;
        }
        return r;
    } else {
        if !(fabs(n - round(n)) > 1e-7f64 * fmax2(1.0f64, fabs(n))) {
            n = round(n);
            if n < k {
                return 0.0f64;
            }
            if n - k < 30 as libc::c_int as libc::c_double {
                return choose(n, n - k);
            }
            return round(exp(Rf_lfastchoose(n, k)));
        }
    }
    if n < k - 1 as libc::c_int as libc::c_double {
        let mut s_choose: libc::c_int = 0;
        r = lfastchoose2(n, k, &mut s_choose);
        return s_choose as libc::c_double * exp(r);
    }
    return exp(Rf_lfastchoose(n, k));
}
