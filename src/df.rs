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
    fn dgamma(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn dbinom_raw(
        x: libc::c_double,
        n: libc::c_double,
        p: libc::c_double,
        q: libc::c_double,
        give_log_0: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn df(
    mut x: libc::c_double,
    mut m: libc::c_double,
    mut n: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut dens: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || m.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return x + m + n;
    }
    if m <= 0 as libc::c_int as libc::c_double || n <= 0 as libc::c_int as libc::c_double {
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
    if x < 0.0f64 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x == 0.0f64 {
        return if m > 2 as libc::c_int as libc::c_double {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if m == 2 as libc::c_int as libc::c_double {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else {
            1.0f64 / 0.0f64
        };
    }
    if R_finite(m) == 0 && R_finite(n) == 0 {
        if x == 1.0f64 {
            return 1.0f64 / 0.0f64;
        } else {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
    }
    if R_finite(n) == 0 {
        return dgamma(x, m / 2 as libc::c_int as libc::c_double, 2.0f64 / m, log_p);
    }
    if m > 1e14f64 {
        dens = dgamma(
            1.0f64 / x,
            n / 2 as libc::c_int as libc::c_double,
            2.0f64 / n,
            log_p,
        );
        return if log_p != 0 {
            dens - 2 as libc::c_int as libc::c_double * log(x)
        } else {
            dens / (x * x)
        };
    }
    f = 1.0f64 / (n + x * m);
    q = n * f;
    p = x * m * f;
    if m >= 2 as libc::c_int as libc::c_double {
        f = m * q / 2 as libc::c_int as libc::c_double;
        dens = dbinom_raw(
            (m - 2 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double,
            (m + n - 2 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double,
            p,
            q,
            log_p,
        );
    } else {
        f = m * m * q / (2 as libc::c_int as libc::c_double * p * (m + n));
        dens = dbinom_raw(
            m / 2 as libc::c_int as libc::c_double,
            (m + n) / 2 as libc::c_int as libc::c_double,
            p,
            q,
            log_p,
        );
    }
    return if log_p != 0 { log(f) + dens } else { f * dens };
}
