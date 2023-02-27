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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn dchisq(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn dpois_raw(_: libc::c_double, _: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn dnchisq(
    mut x: libc::c_double,
    mut df: libc::c_double,
    mut ncp: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut eps: libc::c_double = 5e-15f64;
    let mut i: libc::c_double = 0.;
    let mut ncp2: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut mid: libc::c_double = 0.;
    let mut dfmid: libc::c_double = 0.;
    let mut imax: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || df.is_nan() as i32 != 0 as libc::c_int
        || ncp.is_nan() as i32 != 0 as libc::c_int
    {
        return x + df + ncp;
    }
    if R_finite(df) == 0
        || R_finite(ncp) == 0
        || ncp < 0 as libc::c_int as libc::c_double
        || df < 0 as libc::c_int as libc::c_double
    {
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
    if x < 0 as libc::c_int as libc::c_double {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    if x == 0 as libc::c_int as libc::c_double && df < 2.0f64 {
        return 1.0f64 / 0.0f64;
    }
    if ncp == 0 as libc::c_int as libc::c_double {
        return if df > 0 as libc::c_int as libc::c_double {
            dchisq(x, df, log_p)
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if x == 1.0f64 / 0.0f64 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    ncp2 = 0.5f64 * ncp;
    imax = ceil(
        (-(2 as libc::c_int as libc::c_double + df)
            + sqrt(
                (2 as libc::c_int as libc::c_double - df)
                    * (2 as libc::c_int as libc::c_double - df)
                    + 4 as libc::c_int as libc::c_double * ncp * x,
            ))
            / 4 as libc::c_int as libc::c_double,
    );
    if imax < 0 as libc::c_int as libc::c_double {
        imax = 0 as libc::c_int as libc::c_double;
    }
    if R_finite(imax) != 0 {
        dfmid = df + 2 as libc::c_int as libc::c_double * imax;
        mid = dpois_raw(imax, ncp2, FALSE as libc::c_int) * dchisq(x, dfmid, FALSE as libc::c_int);
    } else {
        mid = 0 as libc::c_int as libc::c_double;
    }
    if mid == 0 as libc::c_int as libc::c_double {
        if log_p != 0 || ncp > 1000.0f64 {
            let mut nl: libc::c_double = df + ncp;
            let mut ic: libc::c_double = nl / (nl + ncp);
            return dchisq(x * ic, nl * ic, log_p);
        } else {
            return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        }
    }
    sum = mid;
    term = mid;
    df = dfmid;
    i = imax;
    let mut x2: libc::c_double = x * ncp2;
    loop {
        i += 1.;
        q = x2 / i / df;
        df += 2 as libc::c_int as libc::c_double;
        term *= q;
        sum += term;
        if !(q >= 1 as libc::c_int as libc::c_double
            || term * q > (1 as libc::c_int as libc::c_double - q) * eps
            || term > 1e-10f64 * sum)
        {
            break;
        }
    }
    term = mid;
    df = dfmid;
    i = imax;
    while i != 0 as libc::c_int as libc::c_double {
        df -= 2 as libc::c_int as libc::c_double;
        q = i * df / x2;
        i -= 1.;
        term *= q;
        sum += term;
        if q < 1 as libc::c_int as libc::c_double
            && term * q <= (1 as libc::c_int as libc::c_double - q) * eps
        {
            break;
        }
    }
    return if log_p != 0 { log(sum) } else { sum };
}
