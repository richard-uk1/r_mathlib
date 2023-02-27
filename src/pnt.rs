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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn pnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pbeta(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn pt(_: libc::c_double, _: libc::c_double, _: libc::c_int, _: libc::c_int) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn pnt(
    mut t: libc::c_double,
    mut df: libc::c_double,
    mut ncp: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut current_block: u64;
    let mut albeta: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut del: libc::c_double = 0.;
    let mut errbd: libc::c_double = 0.;
    let mut lambda: libc::c_double = 0.;
    let mut rxb: libc::c_double = 0.;
    let mut tt: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut geven: libc::c_double = 0.;
    let mut godd: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut tnc: libc::c_double = 0.;
    let mut xeven: libc::c_double = 0.;
    let mut xodd: libc::c_double = 0.;
    let mut it: libc::c_int = 0;
    let mut negdel: libc::c_int = 0;
    let itrmax: libc::c_int = 1000 as libc::c_int;
    static mut errmax: libc::c_double = 1.0e-12f64;
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
        return pt(t, df, lower_tail, log_p);
    }
    if R_finite(t) == 0 {
        return if t < 0 as libc::c_int as libc::c_double {
            if lower_tail != 0 {
                if log_p != 0 {
                    -1.0f64 / 0.0f64
                } else {
                    0.0f64
                }
            } else if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if lower_tail != 0 {
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
    if t >= 0.0f64 {
        negdel = FALSE as libc::c_int;
        tt = t;
        del = ncp;
    } else {
        if ncp > 40 as libc::c_int as libc::c_double && (log_p == 0 || lower_tail == 0) {
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
        negdel = TRUE as libc::c_int;
        tt = -t;
        del = -ncp;
    }
    if df > 4e5f64
        || del * del
            > 2 as libc::c_int as libc::c_double
                * 0.693147180559945309417232121458f64
                * --(1021 as libc::c_int) as libc::c_double
    {
        s = 1.0f64 / (4.0f64 * df);
        return pnorm5(
            tt * (1.0f64 - s),
            del,
            sqrt(1.0f64 + tt * tt * 2.0f64 * s),
            (lower_tail != negdel) as libc::c_int,
            log_p,
        );
    }
    x = t * t;
    rxb = df / (x + df);
    x = x / (x + df);
    if x > 0.0f64 {
        lambda = del * del;
        p = 0.5f64 * exp(-0.5f64 * lambda);
        if p == 0.0f64 {
            if 16 as libc::c_int > 1 as libc::c_int {
                let mut msg_0: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 16 as libc::c_int {
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
                printf(msg_0, b"pnt\0" as *const u8 as *const libc::c_char);
            }
            if 2 as libc::c_int > 1 as libc::c_int {
                let mut msg_1: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 2 as libc::c_int {
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
                printf(msg_1, b"pnt\0" as *const u8 as *const libc::c_char);
            }
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
        q = 0.797884560802865355879892119869f64 * p * del;
        s = 0.5f64 - p;
        if s < 1e-7f64 {
            s = -0.5f64 * expm1(-0.5f64 * lambda);
        }
        a = 0.5f64;
        b = 0.5f64 * df;
        rxb = pow(rxb, b);
        albeta = 0.572364942924700087071713675677f64 + lgammafn(b) - lgammafn(0.5f64 + b);
        xodd = pbeta(x, a, b, TRUE as libc::c_int, FALSE as libc::c_int);
        godd = 2.0f64 * rxb * exp(a * log(x) - albeta);
        tnc = b * x;
        xeven = if tnc < 2.2204460492503131e-16f64 {
            tnc
        } else {
            1.0f64 - rxb
        };
        geven = tnc * rxb;
        tnc = p * xodd + q * xeven;
        it = 1 as libc::c_int;
        loop {
            if !(it <= itrmax) {
                current_block = 5023038348526654800;
                break;
            }
            a += 1.0f64;
            xodd -= godd;
            xeven -= geven;
            godd *= x * (a + b - 1.0f64) / a;
            geven *= x * (a + b - 0.5f64) / (a + 0.5f64);
            p *= lambda / (2 as libc::c_int * it) as libc::c_double;
            q *= lambda / (2 as libc::c_int * it + 1 as libc::c_int) as libc::c_double;
            tnc += p * xodd + q * xeven;
            s -= p;
            if s < -1.0e-10f64 {
                if 8 as libc::c_int > 1 as libc::c_int {
                    let mut msg_2: *mut libc::c_char =
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    match 8 as libc::c_int {
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
                                as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        16 => {
                            msg_2 = b"underflow occurred in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        _ => {}
                    }
                    printf(msg_2, b"pnt\0" as *const u8 as *const libc::c_char);
                }
                current_block = 4638389250046733374;
                break;
            } else {
                if s <= 0 as libc::c_int as libc::c_double && it > 1 as libc::c_int {
                    current_block = 4638389250046733374;
                    break;
                }
                errbd = 2.0f64 * s * (xodd - godd);
                if fabs(errbd) < errmax {
                    current_block = 4638389250046733374;
                    break;
                }
                it += 1;
            }
        }
        match current_block {
            4638389250046733374 => {}
            _ => {
                if 4 as libc::c_int > 1 as libc::c_int {
                    let mut msg_3: *mut libc::c_char =
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    match 4 as libc::c_int {
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
                                as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        16 => {
                            msg_3 = b"underflow occurred in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        _ => {}
                    }
                    printf(msg_3, b"pnt\0" as *const u8 as *const libc::c_char);
                }
            }
        }
    } else {
        tnc = 0.0f64;
    }
    tnc += pnorm5(
        -del,
        0.0f64,
        1.0f64,
        TRUE as libc::c_int,
        FALSE as libc::c_int,
    );
    lower_tail = (lower_tail != negdel) as libc::c_int;
    if tnc > 1 as libc::c_int as libc::c_double - 1e-10f64 && lower_tail != 0 {
        if 8 as libc::c_int > 1 as libc::c_int {
            let mut msg_4: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 8 as libc::c_int {
                1 => {
                    msg_4 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_4 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_4 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_4 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_4 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_4, b"pnt{final}\0" as *const u8 as *const libc::c_char);
        }
    }
    return if lower_tail != 0 {
        if log_p != 0 {
            log(fmin2(tnc, 1.0f64))
        } else {
            fmin2(tnc, 1.0f64)
        }
    } else if log_p != 0 {
        Rlog1p(-fmin2(tnc, 1.0f64))
    } else {
        0.5f64 - fmin2(tnc, 1.0f64) + 0.5f64
    };
}
