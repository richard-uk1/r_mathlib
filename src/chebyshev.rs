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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_chebyshev_init(
    mut dos: *mut libc::c_double,
    mut nos: libc::c_int,
    mut eta: libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut err: libc::c_double = 0.;
    if nos < 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    err = 0.0f64;
    i = 0 as libc::c_int;
    ii = 1 as libc::c_int;
    while ii <= nos {
        i = nos - ii;
        err += fabs(*dos.offset(i as isize));
        if err > eta {
            return i;
        }
        ii += 1;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_chebyshev_eval(
    mut x: libc::c_double,
    mut a: *const libc::c_double,
    n: libc::c_int,
) -> libc::c_double {
    let mut b0: libc::c_double = 0.;
    let mut b1: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    let mut twox: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if n < 1 as libc::c_int || n > 1000 as libc::c_int {
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
    if x < -1.1f64 || x > 1.1f64 {
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
    twox = x * 2 as libc::c_int as libc::c_double;
    b1 = 0 as libc::c_int as libc::c_double;
    b2 = b1;
    b0 = 0 as libc::c_int as libc::c_double;
    i = 1 as libc::c_int;
    while i <= n {
        b2 = b1;
        b1 = b0;
        b0 = twox * b1 - b2 + *a.offset((n - i) as isize);
        i += 1;
    }
    return (b0 - b2) * 0.5f64;
}
