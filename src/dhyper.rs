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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn dbinom_raw(
        x: libc::c_double,
        n: libc::c_double,
        p: libc::c_double,
        q: libc::c_double,
        give_log_0: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn dhyper(
    mut x: libc::c_double,
    mut r: libc::c_double,
    mut b: libc::c_double,
    mut n: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut p1: libc::c_double = 0.;
    let mut p2: libc::c_double = 0.;
    let mut p3: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || r.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return x + r + b + n;
    }
    if r < 0.0f64
        || fabs(r - round(r)) > 1e-7f64 * fmax2(1.0f64, fabs(r))
        || (b < 0.0f64 || fabs(b - round(b)) > 1e-7f64 * fmax2(1.0f64, fabs(b)))
        || (n < 0.0f64 || fabs(n - round(n)) > 1e-7f64 * fmax2(1.0f64, fabs(n)))
        || n > r + b
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
    if x < 0 as libc::c_int as libc::c_double {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if fabs(x - round(x)) > 1e-7f64 * fmax2(1.0f64, fabs(x)) {
        printf(
            b"non-integer x = %f\0" as *const u8 as *const libc::c_char,
            x,
        );
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    x = round(x);
    r = round(r);
    b = round(b);
    n = round(n);
    if n < x || r < x || n - x > b {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if n == 0 as libc::c_int as libc::c_double {
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
    p = n / (r + b);
    q = (r + b - n) / (r + b);
    p1 = dbinom_raw(x, r, p, q, log_p);
    p2 = dbinom_raw(n - x, b, p, q, log_p);
    p3 = dbinom_raw(n, r + b, p, q, log_p);
    return if log_p != 0 {
        p1 + p2 - p3
    } else {
        p1 * p2 / p3
    };
}
