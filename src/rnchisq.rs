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
    fn rchisq(_: libc::c_double) -> libc::c_double;
    fn rpois(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rnchisq(
    mut df: libc::c_double,
    mut lambda: libc::c_double,
) -> libc::c_double {
    if df.is_nan() as i32 != 0 as libc::c_int
        || R_finite(lambda) == 0
        || df < 0.0f64
        || lambda < 0.0f64
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
    if lambda == 0.0f64 {
        return if df == 0.0f64 {
            0.0f64
        } else {
            rgamma(df / 2.0f64, 2.0f64)
        };
    } else {
        let mut r: libc::c_double = rpois(lambda / 2.0f64);
        if r > 0.0f64 {
            r = rchisq(2.0f64 * r);
        }
        if df > 0.0f64 {
            r += rgamma(df / 2.0f64, 2.0f64);
        }
        return r;
    };
}
