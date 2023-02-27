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
    fn rgamma(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn rpois(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rnbinom(
    mut size: libc::c_double,
    mut prob: libc::c_double,
) -> libc::c_double {
    if R_finite(prob) == 0
        || size.is_nan() as i32 != 0 as libc::c_int
        || size <= 0 as libc::c_int as libc::c_double
        || prob <= 0 as libc::c_int as libc::c_double
        || prob > 1 as libc::c_int as libc::c_double
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
    if R_finite(size) == 0 {
        size = 1.7976931348623157e+308f64 / 2.0f64;
    }
    return if prob == 1 as libc::c_int as libc::c_double {
        0 as libc::c_int as libc::c_double
    } else {
        rpois(rgamma(
            size,
            (1 as libc::c_int as libc::c_double - prob) / prob,
        ))
    };
}
#[no_mangle]
pub unsafe extern "C" fn rnbinom_mu(
    mut size: libc::c_double,
    mut mu: libc::c_double,
) -> libc::c_double {
    if R_finite(mu) == 0
        || size.is_nan() as i32 != 0 as libc::c_int
        || size <= 0 as libc::c_int as libc::c_double
        || mu < 0 as libc::c_int as libc::c_double
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
    if R_finite(size) == 0 {
        size = 1.7976931348623157e+308f64 / 2.0f64;
    }
    return if mu == 0 as libc::c_int as libc::c_double {
        0 as libc::c_int as libc::c_double
    } else {
        rpois(rgamma(size, mu / size))
    };
}
