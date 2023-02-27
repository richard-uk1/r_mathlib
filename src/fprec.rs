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
    fn log10(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn nearbyint(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn R_pow_di(_: libc::c_double, _: libc::c_int) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fprec(
    mut x: libc::c_double,
    mut digits: libc::c_double,
) -> libc::c_double {
    let mut l10: libc::c_double = 0.;
    let mut pow10: libc::c_double = 0.;
    let mut sgn: libc::c_double = 0.;
    let mut p10: libc::c_double = 0.;
    let mut P10: libc::c_double = 0.;
    let mut e10: libc::c_int = 0;
    let mut e2: libc::c_int = 0;
    let mut do_round: libc::c_int = 0;
    let mut dig: libc::c_int = 0;
    static mut max10e: libc::c_int = (1024 as libc::c_int as libc::c_double
        * 0.301029995663981195213738894724f64) as libc::c_int;
    if x.is_nan() as i32 != 0 as libc::c_int || digits.is_nan() as i32 != 0 as libc::c_int {
        return x + digits;
    }
    if x.is_finite() as i32 == 0 {
        return x;
    }
    if digits.is_finite() as i32 == 0 {
        if digits > 0.0f64 {
            return x;
        } else {
            digits = 1.0f64;
        }
    }
    if x == 0 as libc::c_int as libc::c_double {
        return x;
    }
    dig = round(digits) as libc::c_int;
    if dig > 22 as libc::c_int {
        return x;
    } else {
        if dig < 1 as libc::c_int {
            dig = 1 as libc::c_int;
        }
    }
    sgn = 1.0f64;
    if x < 0.0f64 {
        sgn = -sgn;
        x = -x;
    }
    l10 = log10(x);
    e10 = ((dig - 1 as libc::c_int) as libc::c_double - floor(l10)) as libc::c_int;
    if fabs(l10) < (max10e - 2 as libc::c_int) as libc::c_double {
        p10 = 1.0f64;
        if e10 > max10e {
            p10 = R_pow_di(10.0f64, e10 - max10e);
            e10 = max10e;
        }
        if e10 > 0 as libc::c_int {
            pow10 = R_pow_di(10.0f64, e10);
            return sgn * (nearbyint(x * pow10 * p10) / pow10) / p10;
        } else {
            pow10 = R_pow_di(10.0f64, -e10);
            return sgn * (nearbyint(x / pow10) * pow10);
        }
    } else {
        do_round = (max10e as libc::c_double - l10 >= R_pow_di(10.0f64, -dig)) as libc::c_int;
        e2 = dig
            + (if e10 > 0 as libc::c_int {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            }) * 22 as libc::c_int;
        p10 = R_pow_di(10.0f64, e2);
        x *= p10;
        P10 = R_pow_di(10.0f64, e10 - e2);
        x *= P10;
        if do_round != 0 {
            x += 0.5f64;
        }
        x = floor(x) / p10;
        return sgn * x / P10;
    };
}
