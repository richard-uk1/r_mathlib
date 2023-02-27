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
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn pnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn lbeta(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn pt(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut val: libc::c_double = 0.;
    let mut nx: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int || n.is_nan() as i32 != 0 as libc::c_int {
        return x + n;
    }
    if n <= 0.0f64 {
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
    if R_finite(x) == 0 {
        return if x < 0 as libc::c_int as libc::c_double {
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
    if R_finite(n) == 0 {
        return pnorm5(x, 0.0f64, 1.0f64, lower_tail, log_p);
    }
    nx = 1 as libc::c_int as libc::c_double + x / n * x;
    if nx > 1e100f64 {
        let mut lval: libc::c_double = 0.;
        lval = -0.5f64 * n * (2 as libc::c_int as libc::c_double * log(fabs(x)) - log(n))
            - lbeta(0.5f64 * n, 0.5f64)
            - log(0.5f64 * n);
        val = if log_p != 0 { lval } else { exp(lval) };
    } else {
        val = if n > x * x {
            pbeta(
                x * x / (n + x * x),
                0.5f64,
                n / 2.0f64,
                0 as libc::c_int,
                log_p,
            )
        } else {
            pbeta(1.0f64 / nx, n / 2.0f64, 0.5f64, 1 as libc::c_int, log_p)
        };
    }
    if x <= 0.0f64 {
        lower_tail = (lower_tail == 0) as libc::c_int;
    }
    if log_p != 0 {
        if lower_tail != 0 {
            return Rlog1p(-0.5f64 * exp(val));
        } else {
            return val - 0.693147180559945309417232121458f64;
        }
    } else {
        val /= 2.0f64;
        return if lower_tail != 0 {
            0.5f64 - val + 0.5f64
        } else {
            val
        };
    };
}
