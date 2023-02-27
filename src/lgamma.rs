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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn gammafn(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_lgammacor(_: libc::c_double) -> libc::c_double;
    fn sinpi(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn lgammafn_sign(
    mut x: libc::c_double,
    mut sgn: *mut libc::c_int,
) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut sinpiy: libc::c_double = 0.;
    if !sgn.is_null() {
        *sgn = 1 as libc::c_int;
    }
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if !sgn.is_null()
        && x < 0 as libc::c_int as libc::c_double
        && fmod(floor(-x), 2.0f64) == 0 as libc::c_int as libc::c_double
    {
        *sgn = -(1 as libc::c_int);
    }
    if x <= 0 as libc::c_int as libc::c_double && x == trunc(x) {
        return 1.0f64 / 0.0f64;
    }
    y = fabs(x);
    if y < 1e-306f64 {
        return -log(y);
    }
    if y <= 10 as libc::c_int as libc::c_double {
        return log(fabs(gammafn(x)));
    }
    if y > 2.5327372760800758e+305f64 {
        return 1.0f64 / 0.0f64;
    }
    if x > 0 as libc::c_int as libc::c_double {
        if x > 1e17f64 {
            return x * (log(x) - 1.0f64);
        } else if x > 4934720.0f64 {
            return 0.918938533204672741780329736406f64 + (x - 0.5f64) * log(x) - x;
        } else {
            return 0.918938533204672741780329736406f64 + (x - 0.5f64) * log(x) - x
                + Rf_lgammacor(x);
        }
    }
    sinpiy = fabs(sinpi(y));
    if sinpiy == 0 as libc::c_int as libc::c_double {
        printf(
            b" ** should NEVER happen! *** [lgamma.c: Neg.int, y=%g]\n\0" as *const u8
                as *const libc::c_char,
            y,
        );
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
    ans = 0.225791352644727432363097614947f64 + (x - 0.5f64) * log(y)
        - x
        - log(sinpiy)
        - Rf_lgammacor(y);
    if fabs((x - trunc(x - 0.5f64)) * ans / x) < 1.490116119384765625e-8f64 {
        if 8 as libc::c_int > 1 as libc::c_int {
            let mut msg_0: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 8 as libc::c_int {
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
            printf(msg_0, b"lgamma\0" as *const u8 as *const libc::c_char);
        }
    }
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn lgammafn(mut x: libc::c_double) -> libc::c_double {
    return lgammafn_sign(x, 0 as *mut libc::c_int);
}
