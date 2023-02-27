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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn dnorm4(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn Rf_stirlerr(_: libc::c_double) -> libc::c_double;
    fn Rf_bd0(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
#[no_mangle]
pub unsafe extern "C" fn dt(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int || n.is_nan() as i32 != 0 as libc::c_int {
        return x + n;
    }
    if n <= 0 as libc::c_int as libc::c_double {
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
    if R_finite(x) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if R_finite(n) == 0 {
        return dnorm4(x, 0.0f64, 1.0f64, log_p);
    }
    let mut u: libc::c_double = 0.;
    let mut t: libc::c_double = -Rf_bd0(
        n / 2.0f64,
        (n + 1 as libc::c_int as libc::c_double) / 2.0f64,
    ) + Rf_stirlerr((n + 1 as libc::c_int as libc::c_double) / 2.0f64)
        - Rf_stirlerr(n / 2.0f64);
    let mut x2n: libc::c_double = x * x / n;
    let mut ax: libc::c_double = 0.0f64;
    let mut l_x2n: libc::c_double = 0.;
    let mut lrg_x2n: Rboolean =
        (x2n > 1.0f64 / 2.2204460492503131e-16f64) as libc::c_int as Rboolean;
    if lrg_x2n as u64 != 0 {
        ax = fabs(x);
        l_x2n = log(ax) - log(n) / 2.0f64;
        u = n * l_x2n;
    } else if x2n > 0.2f64 {
        l_x2n = log(1 as libc::c_int as libc::c_double + x2n) / 2.0f64;
        u = n * l_x2n;
    } else {
        l_x2n = Rlog1p(x2n) / 2.0f64;
        u = -Rf_bd0(n / 2.0f64, (n + x * x) / 2.0f64) + x * x / 2.0f64;
    }
    if log_p != 0 {
        return t - u - (0.918938533204672741780329736406f64 + l_x2n);
    }
    let mut I_sqrt_: libc::c_double = if lrg_x2n as libc::c_uint != 0 {
        sqrt(n) / ax
    } else {
        exp(-l_x2n)
    };
    return exp(t - u) * 0.398942280401432677939946059934f64 * I_sqrt_;
}
