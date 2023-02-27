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
    fn log(_: libc::c_double) -> libc::c_double;
    fn lgamma(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn gammafn(_: libc::c_double) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn Rf_lgammacor(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lbeta(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    let mut corr: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    if a.is_nan() as i32 != 0 as libc::c_int || b.is_nan() as i32 != 0 as libc::c_int {
        return a + b;
    }
    q = a;
    p = q;
    if b < p {
        p = b;
    }
    if b > q {
        q = b;
    }
    if p < 0 as libc::c_int as libc::c_double {
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
    } else {
        if p == 0 as libc::c_int as libc::c_double {
            return 1.0f64 / 0.0f64;
        } else {
            if R_finite(q) == 0 {
                return -1.0f64 / 0.0f64;
            }
        }
    }
    if p >= 10 as libc::c_int as libc::c_double {
        corr = Rf_lgammacor(p) + Rf_lgammacor(q) - Rf_lgammacor(p + q);
        return log(q) * -0.5f64
            + 0.918938533204672741780329736406f64
            + corr
            + (p - 0.5f64) * log(p / (p + q))
            + q * Rlog1p(-p / (p + q));
    } else if q >= 10 as libc::c_int as libc::c_double {
        corr = Rf_lgammacor(q) - Rf_lgammacor(p + q);
        return lgammafn(p) + corr + p - p * log(p + q) + (q - 0.5f64) * Rlog1p(-p / (p + q));
    } else if p < 1e-306f64 {
        return lgamma(p) + (lgamma(q) - lgamma(p + q));
    } else {
        return log(gammafn(p) * (gammafn(q) / gammafn(p + q)));
    };
}
