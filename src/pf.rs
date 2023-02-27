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
    fn pbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pchisq(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pf(
    mut x: libc::c_double,
    mut df1: libc::c_double,
    mut df2: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || df1.is_nan() as i32 != 0 as libc::c_int
        || df2.is_nan() as i32 != 0 as libc::c_int
    {
        return x + df2 + df1;
    }
    if df1 <= 0.0f64 || df2 <= 0.0f64 {
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
    if x <= 0.0f64 {
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
    if x >= 1.0f64 / 0.0f64 {
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
    if df2 == 1.0f64 / 0.0f64 {
        if df1 == 1.0f64 / 0.0f64 {
            if x < 1.0f64 {
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
            if x == 1.0f64 {
                return if log_p != 0 {
                    -0.693147180559945309417232121458f64
                } else {
                    0.5f64
                };
            }
            if x > 1.0f64 {
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
        return pchisq(x * df1, df1, lower_tail, log_p);
    }
    if df1 == 1.0f64 / 0.0f64 {
        return pchisq(df2 / x, df2, (lower_tail == 0) as libc::c_int, log_p);
    }
    if df1 * x > df2 {
        x = pbeta(
            df2 / (df2 + df1 * x),
            df2 / 2.0f64,
            df1 / 2.0f64,
            (lower_tail == 0) as libc::c_int,
            log_p,
        );
    } else {
        x = pbeta(
            df1 * x / (df2 + df1 * x),
            df1 / 2.0f64,
            df2 / 2.0f64,
            lower_tail,
            log_p,
        );
    }
    return if !(x.is_nan() as i32 != 0 as libc::c_int) {
        x
    } else {
        0.0f64 / 0.0f64
    };
}
