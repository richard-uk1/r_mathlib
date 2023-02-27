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
    fn qnchisq(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn qnbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qnf(
    mut p: libc::c_double,
    mut df1: libc::c_double,
    mut df2: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int
        || df1.is_nan() as i32 != 0 as libc::c_int
        || df2.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return p + df1 + df2 + ncp;
    }
    if df1 <= 0.0f64 || df2 <= 0.0f64 || ncp < 0 as libc::c_int as libc::c_double {
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
    if R_finite(ncp) == 0 {
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
    if R_finite(df1) == 0 && R_finite(df2) == 0 {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg_1: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
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
            printf(msg_1, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
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
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                0 as libc::c_int as libc::c_double
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                0 as libc::c_int as libc::c_double
            } else {
                1.0f64 / 0.0f64
            };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_3: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_3 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_3 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_3 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_3 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_3 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_3, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                0 as libc::c_int as libc::c_double
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                0 as libc::c_int as libc::c_double
            };
        }
    }
    if df2 > 1e8f64 {
        return qnchisq(p, df1, ncp, lower_tail, log_p) / df1;
    }
    y = qnbeta(p, df1 / 2.0f64, df2 / 2.0f64, ncp, lower_tail, log_p);
    return y / (1 as libc::c_int as libc::c_double - y) * (df2 / df1);
}
