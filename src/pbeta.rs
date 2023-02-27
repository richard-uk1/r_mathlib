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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_bratio(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        w: *mut libc::c_double,
        w1: *mut libc::c_double,
        ierr: *mut libc::c_int,
        log_p: libc::c_int,
    );
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pbeta_raw(
    mut x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if a == 0 as libc::c_int as libc::c_double
        || b == 0 as libc::c_int as libc::c_double
        || R_finite(a) == 0
        || R_finite(b) == 0
    {
        if a == 0 as libc::c_int as libc::c_double && b == 0 as libc::c_int as libc::c_double {
            return if log_p != 0 {
                -0.693147180559945309417232121458f64
            } else {
                0.5f64
            };
        }
        if a == 0 as libc::c_int as libc::c_double || a / b == 0 as libc::c_int as libc::c_double {
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
        if b == 0 as libc::c_int as libc::c_double || b / a == 0 as libc::c_int as libc::c_double {
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
        if x < 0.5f64 {
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
        } else {
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
    }
    let mut x1: libc::c_double = 0.5f64 - x + 0.5f64;
    let mut w: libc::c_double = 0.;
    let mut wc: libc::c_double = 0.;
    let mut ierr: libc::c_int = 0;
    Rf_bratio(a, b, x, x1, &mut w, &mut wc, &mut ierr, log_p);
    if ierr != 0 && ierr != 11 as libc::c_int && ierr != 14 as libc::c_int {
        printf(
            b"pbeta_raw(%g, a=%g, b=%g, ..) -> bratio() gave error code %d\0" as *const u8
                as *const libc::c_char,
            x,
            a,
            b,
            ierr,
        );
    }
    return if lower_tail != 0 { w } else { wc };
}
#[no_mangle]
pub unsafe extern "C" fn pbeta(
    mut x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || a.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
    {
        return x + a + b;
    }
    if a < 0 as libc::c_int as libc::c_double || b < 0 as libc::c_int as libc::c_double {
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
    if x <= 0 as libc::c_int as libc::c_double {
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
    if x >= 1 as libc::c_int as libc::c_double {
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
    return pbeta_raw(x, a, b, lower_tail, log_p);
}
