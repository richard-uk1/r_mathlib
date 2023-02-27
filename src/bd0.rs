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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_bd0(mut x: libc::c_double, mut np: libc::c_double) -> libc::c_double {
    let mut ej: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut s1: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    if R_finite(x) == 0 || R_finite(np) == 0 || np == 0.0f64 {
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
    if fabs(x - np) < 0.1f64 * (x + np) {
        v = (x - np) / (x + np);
        s = (x - np) * v;
        if fabs(s) < 2.2250738585072014e-308f64 {
            return s;
        }
        ej = 2 as libc::c_int as libc::c_double * x * v;
        v = v * v;
        j = 1 as libc::c_int;
        while j < 1000 as libc::c_int {
            ej *= v;
            s1 = s + ej / ((j << 1 as libc::c_int) + 1 as libc::c_int) as libc::c_double;
            if s1 == s {
                return s1;
            }
            s = s1;
            j += 1;
        }
    }
    return x * log(x / np) + np - x;
}
