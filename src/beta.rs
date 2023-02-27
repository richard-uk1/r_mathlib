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
    fn gammafn(_: libc::c_double) -> libc::c_double;
    fn lbeta(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn beta(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    if a.is_nan() as i32 != 0 as libc::c_int || b.is_nan() as i32 != 0 as libc::c_int {
        return a + b;
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
    } else {
        if a == 0 as libc::c_int as libc::c_double || b == 0 as libc::c_int as libc::c_double {
            return 1.0f64 / 0.0f64;
        } else {
            if R_finite(a) == 0 || R_finite(b) == 0 {
                return 0 as libc::c_int as libc::c_double;
            }
        }
    }
    if a + b < 171.61447887182298f64 {
        return 1 as libc::c_int as libc::c_double / gammafn(a + b) * gammafn(a) * gammafn(b);
    } else {
        let mut val: libc::c_double = lbeta(a, b);
        return exp(val);
    };
}
