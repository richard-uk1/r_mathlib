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
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn Rf_lfastchoose(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn qhyper(
    mut p: libc::c_double,
    mut NR: libc::c_double,
    mut NB: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut N: libc::c_double = 0.;
    let mut xstart: libc::c_double = 0.;
    let mut xend: libc::c_double = 0.;
    let mut xr: libc::c_double = 0.;
    let mut xb: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    let mut small_N: libc::c_int = 0;
    if p.is_nan() as i32 != 0 as libc::c_int
        || NR.is_nan() as i32 != 0 as libc::c_int
        || NB.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return p + NR + NB + n;
    }
    if R_finite(p) == 0 || R_finite(NR) == 0 || R_finite(NB) == 0 || R_finite(n) == 0 {
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
    NR = round(NR);
    NB = round(NB);
    N = NR + NB;
    n = round(n);
    if NR < 0 as libc::c_int as libc::c_double
        || NB < 0 as libc::c_int as libc::c_double
        || n < 0 as libc::c_int as libc::c_double
        || n > N
    {
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
    xstart = fmax2(0 as libc::c_int as libc::c_double, n - NB);
    xend = fmin2(n, NR);
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_1: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_1 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_1 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_1 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_1 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_1, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 { xend } else { xstart };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 { xstart } else { xend };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_2: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_2 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_2 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_2 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_2 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_2 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_2, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 { xstart } else { xend };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 { xend } else { xstart };
        }
    }
    xr = xstart;
    xb = n - xr;
    small_N = (N < 1000 as libc::c_int as libc::c_double) as libc::c_int;
    term = Rf_lfastchoose(NR, xr) + Rf_lfastchoose(NB, xb) - Rf_lfastchoose(N, n);
    if small_N != 0 {
        term = exp(term);
    }
    NR -= xr;
    NB -= xb;
    if lower_tail == 0 || log_p != 0 {
        p = if log_p != 0 {
            if lower_tail != 0 {
                exp(p)
            } else {
                -expm1(p)
            }
        } else if lower_tail != 0 {
            p
        } else {
            0.5f64 - p + 0.5f64
        };
    }
    p *= 1 as libc::c_int as libc::c_double
        - 1000 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
    sum = if small_N != 0 { term } else { exp(term) };
    while sum < p && xr < xend {
        xr += 1.;
        NB += 1.;
        if small_N != 0 {
            term *= NR / xr * (xb / NB);
        } else {
            term += log(NR / xr * (xb / NB));
        }
        sum += if small_N != 0 { term } else { exp(term) };
        xb -= 1.;
        NR -= 1.;
    }
    return xr;
}
