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
    fn dnorm4(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn dt(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn pnt(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dnt(
    mut x: libc::c_double,
    mut df: libc::c_double,
    mut ncp: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut u: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int || df.is_nan() as i32 != 0 as libc::c_int {
        return x + df;
    }
    if df <= 0.0f64 {
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
    if ncp == 0.0f64 {
        return dt(x, df, log_p);
    }
    if R_finite(x) == 0 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if R_finite(df) == 0 || df > 1e8f64 {
        return dnorm4(x, ncp, 1.0f64, log_p);
    }
    if fabs(x) > sqrt(df * 2.2204460492503131e-16f64) {
        u = log(df) - log(fabs(x))
            + log(fabs(
                pnt(
                    x * sqrt((df + 2 as libc::c_int as libc::c_double) / df),
                    df + 2 as libc::c_int as libc::c_double,
                    ncp,
                    1 as libc::c_int,
                    0 as libc::c_int,
                ) - pnt(x, df, ncp, 1 as libc::c_int, 0 as libc::c_int),
            ));
    } else {
        u = lgammafn(
            (df + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double,
        ) - lgammafn(df / 2 as libc::c_int as libc::c_double)
            - (0.572364942924700087071713675677f64 + 0.5f64 * (log(df) + ncp * ncp));
    }
    return if log_p != 0 { u } else { exp(u) };
}
