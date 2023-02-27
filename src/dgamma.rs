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
    fn dpois_raw(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dgamma(
    mut x: libc::c_double,
    mut shape: libc::c_double,
    mut scale: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut pr: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || shape.is_nan() as i32 != 0 as libc::c_int
        || scale.is_nan() as i32 != 0 as libc::c_int
    {
        return x + shape + scale;
    }
    if shape < 0 as libc::c_int as libc::c_double || scale <= 0 as libc::c_int as libc::c_double {
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
    if shape == 0 as libc::c_int as libc::c_double {
        return if x == 0 as libc::c_int as libc::c_double {
            1.0f64 / 0.0f64
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if x == 0 as libc::c_int as libc::c_double {
        if shape < 1 as libc::c_int as libc::c_double {
            return 1.0f64 / 0.0f64;
        }
        if shape > 1 as libc::c_int as libc::c_double {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
        return if log_p != 0 {
            -log(scale)
        } else {
            1 as libc::c_int as libc::c_double / scale
        };
    }
    if shape < 1 as libc::c_int as libc::c_double {
        pr = dpois_raw(shape, x / scale, log_p);
        return if log_p != 0 {
            pr + (if R_finite(shape / x) != 0 {
                log(shape / x)
            } else {
                log(shape) - log(x)
            })
        } else {
            pr * shape / x
        };
    }
    pr = dpois_raw(shape - 1 as libc::c_int as libc::c_double, x / scale, log_p);
    return if log_p != 0 {
        pr - log(scale)
    } else {
        pr / scale
    };
}
