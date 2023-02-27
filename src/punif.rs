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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn punif(
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
    if b < a {
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
    if R_finite(a) == 0 || R_finite(b) == 0 {
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
    if x >= b {
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
    if x <= a {
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
    if lower_tail != 0 {
        return if log_p != 0 {
            log((x - a) / (b - a))
        } else {
            (x - a) / (b - a)
        };
    } else {
        return if log_p != 0 {
            log((b - x) / (b - a))
        } else {
            (b - x) / (b - a)
        };
    };
}
