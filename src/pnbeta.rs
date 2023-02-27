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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn Rf_bratio(
        a: libc::c_double,
        b: libc::c_double,
        x: libc::c_double,
        y: libc::c_double,
        w: *mut libc::c_double,
        w1: *mut libc::c_double,
        ierr: *mut libc::c_int,
        log_p: libc::c_int,
    );
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn Rf_pnbeta_raw(
    mut x: libc::c_double,
    mut o_x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut ncp: libc::c_double,
) -> libc::c_double {
    static mut errmax: libc::c_double = 1.0e-9f64;
    let itrmax: libc::c_int = 10000 as libc::c_int;
    let mut a0: libc::c_double = 0.;
    let mut lbeta: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut errbd: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut tmp_c: libc::c_double = 0.;
    let mut ierr: libc::c_int = 0;
    let mut ans: libc::c_double = 0.;
    let mut ax: libc::c_double = 0.;
    let mut gx: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut sumq: libc::c_double = 0.;
    if ncp < 0.0f64 || a <= 0.0f64 || b <= 0.0f64 {
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
    if x < 0.0f64 || o_x > 1.0f64 || x == 0.0f64 && o_x == 1.0f64 {
        return 0.0f64;
    }
    if x > 1.0f64 || o_x < 0.0f64 || x == 1.0f64 && o_x == 0.0f64 {
        return 1.0f64;
    }
    c = ncp / 2.0f64;
    x0 = floor(fmax2(c - 7.0f64 * sqrt(c), 0.0f64));
    a0 = a + x0;
    lbeta = lgammafn(a0) + lgammafn(b) - lgammafn(a0 + b);
    Rf_bratio(
        a0,
        b,
        x,
        o_x,
        &mut temp,
        &mut tmp_c,
        &mut ierr,
        FALSE as libc::c_int,
    );
    gx = exp(a0 * log(x) + b * (if x < 0.5f64 { Rlog1p(-x) } else { log(o_x) }) - lbeta - log(a0));
    if a0 > a {
        q = exp(-c + x0 * log(c) - lgammafn(x0 + 1.0f64));
    } else {
        q = exp(-c);
    }
    sumq = 1.0f64 - q;
    ax = q * temp;
    ans = ax;
    let mut j: libc::c_double = floor(x0);
    loop {
        j += 1.;
        temp -= gx;
        gx *= x * (a + b + j - 1.0f64) / (a + j);
        q *= c / j;
        sumq -= q;
        ax = temp * q;
        ans += ax;
        errbd = (temp - gx) * sumq;
        if !(errbd > errmax && j < itrmax as libc::c_double + x0) {
            break;
        }
    }
    if errbd > errmax {
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
            printf(msg_0, b"pnbeta\0" as *const u8 as *const libc::c_char);
        }
    }
    if j >= itrmax as libc::c_double + x0 {
        if 4 as libc::c_int > 1 as libc::c_int {
            let mut msg_1: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 4 as libc::c_int {
                1 => {
                    msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_1 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_1 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_1 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_1 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_1, b"pnbeta\0" as *const u8 as *const libc::c_char);
        }
    }
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_pnbeta2(
    mut x: libc::c_double,
    mut o_x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut ans: libc::c_double = Rf_pnbeta_raw(x, o_x, a, b, ncp);
    if lower_tail != 0 {
        return if log_p != 0 { log(ans) } else { ans };
    } else {
        if ans > 1.0f64 - 1e-10f64 {
            if 8 as libc::c_int > 1 as libc::c_int {
                let mut msg: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 8 as libc::c_int {
                    1 => {
                        msg = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
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
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg, b"pnbeta\0" as *const u8 as *const libc::c_char);
            }
        }
        if ans > 1.0f64 {
            ans = 1.0f64;
        }
        return if log_p != 0 {
            Rlog1p(-ans)
        } else {
            1.0f64 - ans
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnbeta(
    mut x: libc::c_double,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int
        || a.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return x + a + b + ncp;
    }
    if x <= 0.0f64 {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    if x >= 1.0f64 {
        return if lower_tail != 0 {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    return Rf_pnbeta2(
        x,
        1 as libc::c_int as libc::c_double - x,
        a,
        b,
        ncp,
        lower_tail,
        log_p,
    );
}
