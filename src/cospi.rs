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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cospi(mut x: libc::c_double) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if R_finite(x) == 0 {
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
    x = fmod(fabs(x), 2.0f64);
    if fmod(x, 1.0f64) == 0.5f64 {
        return 0.0f64;
    }
    if x == 1.0f64 {
        return -1.0f64;
    }
    if x == 0.0f64 {
        return 1.0f64;
    }
    return cos(3.141592653589793238462643383280f64 * x);
}
#[no_mangle]
pub unsafe extern "C" fn sinpi(mut x: libc::c_double) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if R_finite(x) == 0 {
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
    x = fmod(x, 2.0f64);
    if x <= -(1 as libc::c_int) as libc::c_double {
        x += 2.0f64;
    } else if x > 1.0f64 {
        x -= 2.0f64;
    }
    if x == 0.0f64 || x == 1.0f64 {
        return 0.0f64;
    }
    if x == 0.5f64 {
        return 1.0f64;
    }
    if x == -0.5f64 {
        return -1.0f64;
    }
    return sin(3.141592653589793238462643383280f64 * x);
}
#[no_mangle]
pub unsafe extern "C" fn tanpi(mut x: libc::c_double) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if R_finite(x) == 0 {
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
    x = fmod(x, 1.0f64);
    if x <= -0.5f64 {
        x += 1.;
    } else if x > 0.5f64 {
        x -= 1.;
    }
    return if x == 0.0f64 {
        0.0f64
    } else if x == 0.5f64 {
        0.0f64 / 0.0f64
    } else {
        tan(3.141592653589793238462643383280f64 * x)
    };
}
