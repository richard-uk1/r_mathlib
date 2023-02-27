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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn dbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn dpois_raw(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn dnbeta(
    mut x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut ncp: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut eps: libc::c_double = 1.0e-15f64;
    let mut kMax: libc::c_int = 0;
    let mut k: libc::c_double = 0.;
    let mut ncp2: libc::c_double = 0.;
    let mut dx2: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut D: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    let mut p_k: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || a.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return x + a + b + ncp;
    }
    if ncp < 0 as libc::c_int as libc::c_double
        || a <= 0 as libc::c_int as libc::c_double
        || b <= 0 as libc::c_int as libc::c_double
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
    if R_finite(a) == 0 || R_finite(b) == 0 || R_finite(ncp) == 0 {
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
    if x < 0 as libc::c_int as libc::c_double || x > 1 as libc::c_int as libc::c_double {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if ncp == 0 as libc::c_int as libc::c_double {
        return dbeta(x, a, b, log_p);
    }
    ncp2 = 0.5f64 * ncp;
    dx2 = ncp2 * x;
    d = (dx2 - a - 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double;
    D = d * d + dx2 * (a + b) - a;
    if D <= 0 as libc::c_int as libc::c_double {
        kMax = 0 as libc::c_int;
    } else {
        D = ceil(d + sqrt(D));
        kMax = if D > 0 as libc::c_int as libc::c_double {
            D as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    term = dbeta(x, a + kMax as libc::c_double, b, TRUE as libc::c_int);
    p_k = dpois_raw(kMax as libc::c_double, ncp2, TRUE as libc::c_int);
    if x == 0.0f64 || R_finite(term) == 0 || R_finite(p_k) == 0 {
        return if log_p != 0 {
            p_k + term
        } else {
            exp(p_k + term)
        };
    }
    p_k += term;
    term = 1.0f64;
    sum = term;
    k = kMax as libc::c_double;
    while k > 0 as libc::c_int as libc::c_double && term > sum * eps {
        k -= 1.;
        q = (k + 1 as libc::c_int as libc::c_double) * (k + a) / (k + a + b) / dx2;
        term *= q;
        sum += term;
    }
    term = 1.0f64;
    k = kMax as libc::c_double;
    loop {
        q = dx2 * (k + a + b) / (k + a) / (k + 1 as libc::c_int as libc::c_double);
        k += 1.;
        term *= q;
        sum += term;
        if !(term > sum * eps) {
            break;
        }
    }
    return if log_p != 0 {
        p_k + log(sum)
    } else {
        exp(p_k + log(sum))
    };
}
