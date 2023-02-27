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
    fn rchisq(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rf(mut n1: libc::c_double, mut n2: libc::c_double) -> libc::c_double {
    let mut v1: libc::c_double = 0.;
    let mut v2: libc::c_double = 0.;
    if n1.is_nan() as i32 != 0 as libc::c_int
        || n2.is_nan() as i32 != 0 as libc::c_int
        || n1 <= 0.0f64
        || n2 <= 0.0f64
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
    v1 = if R_finite(n1) != 0 {
        rchisq(n1) / n1
    } else {
        1 as libc::c_int as libc::c_double
    };
    v2 = if R_finite(n2) != 0 {
        rchisq(n2) / n2
    } else {
        1 as libc::c_int as libc::c_double
    };
    return v1 / v2;
}
