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
    fn log(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn dbinom_raw(
        x: libc::c_double,
        n: libc::c_double,
        p: libc::c_double,
        q: libc::c_double,
        give_log: libc::c_int,
    ) -> libc::c_double;
    fn lbeta(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn dbeta(
    mut x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || a.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
    {
        return x + a + b;
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
    }
    if x < 0 as libc::c_int as libc::c_double || x > 1 as libc::c_int as libc::c_double {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if a == 0 as libc::c_int as libc::c_double
        || b == 0 as libc::c_int as libc::c_double
        || R_finite(a) == 0
        || R_finite(b) == 0
    {
        if a == 0 as libc::c_int as libc::c_double && b == 0 as libc::c_int as libc::c_double {
            if x == 0 as libc::c_int as libc::c_double || x == 1 as libc::c_int as libc::c_double {
                return 1.0f64 / 0.0f64;
            } else {
                return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
            }
        }
        if a == 0 as libc::c_int as libc::c_double || a / b == 0 as libc::c_int as libc::c_double {
            if x == 0 as libc::c_int as libc::c_double {
                return 1.0f64 / 0.0f64;
            } else {
                return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
            }
        }
        if b == 0 as libc::c_int as libc::c_double || b / a == 0 as libc::c_int as libc::c_double {
            if x == 1 as libc::c_int as libc::c_double {
                return 1.0f64 / 0.0f64;
            } else {
                return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
            }
        }
        if x == 0.5f64 {
            return 1.0f64 / 0.0f64;
        } else {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
    }
    if x == 0 as libc::c_int as libc::c_double {
        if a > 1 as libc::c_int as libc::c_double {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
        if a < 1 as libc::c_int as libc::c_double {
            return 1.0f64 / 0.0f64;
        }
        return if log_p != 0 { log(b) } else { b };
    }
    if x == 1 as libc::c_int as libc::c_double {
        if b > 1 as libc::c_int as libc::c_double {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
        if b < 1 as libc::c_int as libc::c_double {
            return 1.0f64 / 0.0f64;
        }
        return if log_p != 0 { log(a) } else { a };
    }
    let mut lval: libc::c_double = 0.;
    if a <= 2 as libc::c_int as libc::c_double || b <= 2 as libc::c_int as libc::c_double {
        lval = (a - 1 as libc::c_int as libc::c_double) * log(x)
            + (b - 1 as libc::c_int as libc::c_double) * Rlog1p(-x)
            - lbeta(a, b);
    } else {
        lval = log(a + b - 1 as libc::c_int as libc::c_double)
            + dbinom_raw(
                a - 1 as libc::c_int as libc::c_double,
                a + b - 2 as libc::c_int as libc::c_double,
                x,
                1 as libc::c_int as libc::c_double - x,
                TRUE as libc::c_int,
            );
    }
    return if log_p != 0 { lval } else { exp(lval) };
}
