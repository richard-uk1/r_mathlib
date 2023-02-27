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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qnorm5(
    mut p: libc::c_double,
    mut mu: libc::c_double,
    mut sigma: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut p_: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut val: libc::c_double = 0.;
    if p.is_nan() as i32 != 0 as libc::c_int
        || mu.is_nan() as i32 != 0 as libc::c_int
        || sigma.is_nan() as i32 != 0 as libc::c_int
    {
        return p + mu + sigma;
    }
    if log_p != 0 {
        if p > 0 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
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
                printf(msg, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
        if p == -1.0f64 / 0.0f64 {
            return if lower_tail != 0 {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
    } else {
        if p < 0 as libc::c_int as libc::c_double || p > 1 as libc::c_int as libc::c_double {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_0: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_0 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_0 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_0 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_0 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_0 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_0, b"\0" as *const u8 as *const libc::c_char);
            }
            return 0.0f64 / 0.0f64;
        }
        if p == 0 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                -1.0f64 / 0.0f64
            } else {
                1.0f64 / 0.0f64
            };
        }
        if p == 1 as libc::c_int as libc::c_double {
            return if lower_tail != 0 {
                1.0f64 / 0.0f64
            } else {
                -1.0f64 / 0.0f64
            };
        }
    }
    if sigma < 0 as libc::c_int as libc::c_double {
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
    if sigma == 0 as libc::c_int as libc::c_double {
        return mu;
    }
    p_ = if log_p != 0 {
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
    q = p_ - 0.5f64;
    if fabs(q) <= 0.425f64 {
        r = 0.180625f64 - q * q;
        val = q
            * (((((((r * 2509.0809287301226727f64 + 33430.575583588128105f64) * r
                + 67265.770927008700853f64)
                * r
                + 45921.953931549871457f64)
                * r
                + 13731.693765509461125f64)
                * r
                + 1971.5909503065514427f64)
                * r
                + 133.14166789178437745f64)
                * r
                + 3.387132872796366608f64)
            / (((((((r * 5226.495278852854561f64 + 28729.085735721942674f64) * r
                + 39307.89580009271061f64)
                * r
                + 21213.794301586595867f64)
                * r
                + 5394.1960214247511077f64)
                * r
                + 687.1870074920579083f64)
                * r
                + 42.313330701600911252f64)
                * r
                + 1.0f64);
    } else {
        if q > 0 as libc::c_int as libc::c_double {
            r = if log_p != 0 {
                if lower_tail != 0 {
                    -expm1(p)
                } else {
                    exp(p)
                }
            } else if lower_tail != 0 {
                0.5f64 - p + 0.5f64
            } else {
                p
            };
        } else {
            r = p_;
        }
        r = sqrt(-if log_p != 0
            && (lower_tail != 0 && q <= 0 as libc::c_int as libc::c_double
                || lower_tail == 0 && q > 0 as libc::c_int as libc::c_double)
        {
            p
        } else {
            log(r)
        });
        if r <= 5.0f64 {
            r += -1.6f64;
            val = (((((((r * 7.7454501427834140764e-4f64 + 0.0227238449892691845833f64) * r
                + 0.24178072517745061177f64)
                * r
                + 1.27045825245236838258f64)
                * r
                + 3.64784832476320460504f64)
                * r
                + 5.7694972214606914055f64)
                * r
                + 4.6303378461565452959f64)
                * r
                + 1.42343711074968357734f64)
                / (((((((r * 1.05075007164441684324e-9f64 + 5.475938084995344946e-4f64) * r
                    + 0.0151986665636164571966f64)
                    * r
                    + 0.14810397642748007459f64)
                    * r
                    + 0.68976733498510000455f64)
                    * r
                    + 1.6763848301838038494f64)
                    * r
                    + 2.05319162663775882187f64)
                    * r
                    + 1.0f64);
        } else {
            r += -5.0f64;
            val = (((((((r * 2.01033439929228813265e-7f64 + 2.71155556874348757815e-5f64) * r
                + 0.0012426609473880784386f64)
                * r
                + 0.026532189526576123093f64)
                * r
                + 0.29656057182850489123f64)
                * r
                + 1.7848265399172913358f64)
                * r
                + 5.4637849111641143699f64)
                * r
                + 6.6579046435011037772f64)
                / (((((((r * 2.04426310338993978564e-15f64 + 1.4215117583164458887e-7f64) * r
                    + 1.8463183175100546818e-5f64)
                    * r
                    + 7.868691311456132591e-4f64)
                    * r
                    + 0.0148753612908506148525f64)
                    * r
                    + 0.13692988092273580531f64)
                    * r
                    + 0.59983220655588793769f64)
                    * r
                    + 1.0f64);
        }
        if q < 0.0f64 {
            val = -val;
        }
    }
    return mu + sigma * val;
}
