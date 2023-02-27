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
    fn round(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn Rf_bd0(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn Rf_stirlerr(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn dbinom_raw(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut p: libc::c_double,
    mut q: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut lf: libc::c_double = 0.;
    let mut lc: libc::c_double = 0.;
    if p == 0 as libc::c_int as libc::c_double {
        return if x == 0 as libc::c_int as libc::c_double {
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
    if q == 0 as libc::c_int as libc::c_double {
        return if x == n {
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
    if x == 0 as libc::c_int as libc::c_double {
        if n == 0 as libc::c_int as libc::c_double {
            return if log_p != 0 { 0.0f64 } else { 1.0f64 };
        }
        lc = if p < 0.1f64 {
            -Rf_bd0(n, n * q) - n * p
        } else {
            n * log(q)
        };
        return if log_p != 0 { lc } else { exp(lc) };
    }
    if x == n {
        lc = if q < 0.1f64 {
            -Rf_bd0(n, n * p) - n * q
        } else {
            n * log(p)
        };
        return if log_p != 0 { lc } else { exp(lc) };
    }
    if x < 0 as libc::c_int as libc::c_double || x > n {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    lc = Rf_stirlerr(n)
        - Rf_stirlerr(x)
        - Rf_stirlerr(n - x)
        - Rf_bd0(x, n * p)
        - Rf_bd0(n - x, n * q);
    lf = 1.837877066409345483560659472811f64 + log(x) + Rlog1p(-x / n);
    return if log_p != 0 {
        lc - 0.5f64 * lf
    } else {
        exp(lc - 0.5f64 * lf)
    };
}
#[no_mangle]
pub unsafe extern "C" fn dbinom(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut p: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
        || p.is_nan() as i32 != 0 as libc::c_int
    {
        return x + n + p;
    }
    if p < 0 as libc::c_int as libc::c_double
        || p > 1 as libc::c_int as libc::c_double
        || (n < 0.0f64 || fabs(n - round(n)) > 1e-7f64 * fmax2(1.0f64, fabs(n)))
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
    if fabs(x - round(x)) > 1e-7f64 * fmax2(1.0f64, fabs(x)) {
        printf(
            b"non-integer x = %f\0" as *const u8 as *const libc::c_char,
            x,
        );
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x < 0 as libc::c_int as libc::c_double || R_finite(x) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    n = round(n);
    x = round(x);
    return dbinom_raw(x, n, p, 1 as libc::c_int as libc::c_double - p, log_p);
}
