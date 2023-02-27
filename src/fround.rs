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
    fn logb(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn nearbyint(_: libc::c_double) -> libc::c_double;
    fn R_pow_di(_: libc::c_double, _: libc::c_int) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fround(
    mut x: libc::c_double,
    mut digits: libc::c_double,
) -> libc::c_double {
    static mut max10e: libc::c_int = 308 as libc::c_int;
    if x.is_nan() as i32 != 0 as libc::c_int || digits.is_nan() as i32 != 0 as libc::c_int {
        return x + digits;
    }
    if x.is_finite() as i32 == 0 {
        return x;
    }
    if digits > (308 as libc::c_int + 15 as libc::c_int) as libc::c_double || x == 0.0f64 {
        return x;
    } else {
        if digits < -max10e as libc::c_double {
            return 0.0f64;
        } else {
            if digits == 0.0f64 {
                return nearbyint(x);
            }
        }
    }
    let mut dig: libc::c_int = floor(digits + 0.5f64) as libc::c_int;
    let mut sgn: libc::c_double = 1.0f64;
    if x < 0.0f64 {
        sgn = -1.0f64;
        x = -x;
    }
    let mut l10x: libc::c_double = 0.301029995663981195213738894724f64 * (0.5f64 + logb(x));
    if l10x + dig as libc::c_double > 15 as libc::c_int as libc::c_double {
        return sgn * x;
    } else {
        let mut pow10: libc::c_double = 0.;
        let mut x10: libc::c_double = 0.;
        let mut i10: libc::c_double = 0.;
        let mut xd: libc::c_double = 0.;
        let mut xu: libc::c_double = 0.;
        if dig <= max10e {
            pow10 = R_pow_di(10.0f64, dig);
            x10 = x * pow10;
            i10 = floor(x10);
            xd = i10 / pow10;
            xu = ceil(x10) / pow10;
        } else {
            let mut e10: libc::c_int = dig - max10e;
            let mut p10: libc::c_double = R_pow_di(10.0f64, e10);
            pow10 = R_pow_di(10.0f64, max10e);
            x10 = x * pow10 * p10;
            i10 = floor(x10);
            xd = i10 / pow10 / p10;
            xu = ceil(x10) / pow10 / p10;
        }
        let mut du: libc::c_double = xu - x;
        let mut dd: libc::c_double = x - xd;
        return sgn
            * (if du < dd || fmod(i10, 2.0f64) == 1 as libc::c_int as libc::c_double && du == dd {
                xu
            } else {
                xd
            });
    };
}
