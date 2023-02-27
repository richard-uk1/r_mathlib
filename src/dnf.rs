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
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn dgamma(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn dnchisq(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn dnbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dnf(
    mut x: libc::c_double,
    mut df1: libc::c_double,
    mut df2: libc::c_double,
    mut ncp: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || df1.is_nan() as i32 != 0 as libc::c_int
        || df2.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return x + df2 + df1 + ncp;
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
    if x < 0.0f64 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
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
        if x == 1.0f64 {
            return 1.0f64 / 0.0f64;
        } else {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
    }
    if R_finite(df2) == 0 {
        return df1 * dnchisq(x * df1, df1, ncp, log_p);
    }
    if df1 > 1e14f64 && ncp < 1e7f64 {
        f = 1 as libc::c_int as libc::c_double + ncp / df1;
        z = dgamma(
            1.0f64 / x / f,
            df2 / 2 as libc::c_int as libc::c_double,
            2.0f64 / df2,
            log_p,
        );
        return if log_p != 0 {
            z - 2 as libc::c_int as libc::c_double * log(x) - log(f)
        } else {
            z / (x * x) / f
        };
    }
    y = df1 / df2 * x;
    z = dnbeta(
        y / (1 as libc::c_int as libc::c_double + y),
        df1 / 2.0f64,
        df2 / 2.0f64,
        ncp,
        log_p,
    );
    return if log_p != 0 {
        z + log(df1) - log(df2) - 2 as libc::c_int as libc::c_double * Rlog1p(y)
    } else {
        z * (df1 / df2)
            / (1 as libc::c_int as libc::c_double + y)
            / (1 as libc::c_int as libc::c_double + y)
    };
}
