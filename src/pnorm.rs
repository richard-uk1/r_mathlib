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
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pnorm5(
    mut x: libc::c_double,
    mut mu: libc::c_double,
    mut sigma: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut p: libc::c_double = 0.;
    let mut cp: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || mu.is_nan() as i32 != 0 as libc::c_int
        || sigma.is_nan() as i32 != 0 as libc::c_int
    {
        return x + mu + sigma;
    }
    if R_finite(x) == 0 && mu == x {
        return 0.0f64 / 0.0f64;
    }
    if sigma <= 0 as libc::c_int as libc::c_double {
        if sigma < 0 as libc::c_int as libc::c_double {
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
        return if x < mu {
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
    p = (x - mu) / sigma;
    if R_finite(p) == 0 {
        return if x < mu {
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
    x = p;
    pnorm_both(
        x,
        &mut p,
        &mut cp,
        if lower_tail != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        },
        log_p,
    );
    return if lower_tail != 0 { p } else { cp };
}
#[no_mangle]
pub unsafe extern "C" fn pnorm_both(
    mut x: libc::c_double,
    mut cum: *mut libc::c_double,
    mut ccum: *mut libc::c_double,
    mut i_tail: libc::c_int,
    mut log_p: libc::c_int,
) {
    static mut a: [libc::c_double; 5] = [
        2.2352520354606839287f64,
        161.02823106855587881f64,
        1067.6894854603709582f64,
        18154.981253343561249f64,
        0.065682337918207449113f64,
    ];
    static mut b: [libc::c_double; 4] = [
        47.20258190468824187f64,
        976.09855173777669322f64,
        10260.932208618978205f64,
        45507.789335026729956f64,
    ];
    static mut c: [libc::c_double; 9] = [
        0.39894151208813466764f64,
        8.8831497943883759412f64,
        93.506656132177855979f64,
        597.27027639480026226f64,
        2494.5375852903726711f64,
        6848.1904505362823326f64,
        11602.651437647350124f64,
        9842.7148383839780218f64,
        1.0765576773720192317e-8f64,
    ];
    static mut d: [libc::c_double; 8] = [
        22.266688044328115691f64,
        235.38790178262499861f64,
        1519.377599407554805f64,
        6485.558298266760755f64,
        18615.571640885098091f64,
        34900.952721145977266f64,
        38912.003286093271411f64,
        19685.429676859990727f64,
    ];
    static mut p: [libc::c_double; 6] = [
        0.21589853405795699f64,
        0.1274011611602473639f64,
        0.022235277870649807f64,
        0.001421619193227893466f64,
        2.9112874951168792e-5f64,
        0.02307344176494017303f64,
    ];
    static mut q: [libc::c_double; 5] = [
        1.28426009614491121f64,
        0.468238212480865118f64,
        0.0659881378689285515f64,
        0.00378239633202758244f64,
        7.29751555083966205e-5f64,
    ];
    let mut xden: libc::c_double = 0.;
    let mut xnum: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut del: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut xsq: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut lower: libc::c_int = 0;
    let mut upper: libc::c_int = 0;
    if x.is_nan() as i32 != 0 as libc::c_int {
        *ccum = x;
        *cum = *ccum;
        return;
    }
    eps = 2.2204460492503131e-16f64 * 0.5f64;
    lower = (i_tail != 1 as libc::c_int) as libc::c_int;
    upper = (i_tail != 0 as libc::c_int) as libc::c_int;
    y = fabs(x);
    if y <= 0.67448975f64 {
        if y > eps {
            xsq = x * x;
            xnum = a[4 as libc::c_int as usize] * xsq;
            xden = xsq;
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                xnum = (xnum + a[i as usize]) * xsq;
                xden = (xden + b[i as usize]) * xsq;
                i += 1;
            }
        } else {
            xden = 0.0f64;
            xnum = xden;
        }
        temp = x * (xnum + a[3 as libc::c_int as usize]) / (xden + b[3 as libc::c_int as usize]);
        if lower != 0 {
            *cum = 0.5f64 + temp;
        }
        if upper != 0 {
            *ccum = 0.5f64 - temp;
        }
        if log_p != 0 {
            if lower != 0 {
                *cum = log(*cum);
            }
            if upper != 0 {
                *ccum = log(*ccum);
            }
        }
    } else if y <= 5.656854249492380195206754896838f64 {
        xnum = c[8 as libc::c_int as usize] * y;
        xden = y;
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int {
            xnum = (xnum + c[i as usize]) * y;
            xden = (xden + d[i as usize]) * y;
            i += 1;
        }
        temp = (xnum + c[7 as libc::c_int as usize]) / (xden + d[7 as libc::c_int as usize]);
        xsq = trunc(y * 16 as libc::c_int as libc::c_double) / 16 as libc::c_int as libc::c_double;
        del = (y - xsq) * (y + xsq);
        if log_p != 0 {
            *cum = -xsq * xsq * 0.5f64 + -del * 0.5f64 + log(temp);
            if lower != 0 && x > 0.0f64 || upper != 0 && x <= 0.0f64 {
                *ccum = Rlog1p(-exp(-xsq * xsq * 0.5f64) * exp(-del * 0.5f64) * temp);
            }
        } else {
            *cum = exp(-xsq * xsq * 0.5f64) * exp(-del * 0.5f64) * temp;
            *ccum = 1.0f64 - *cum;
        }
        if x > 0.0f64 {
            temp = *cum;
            if lower != 0 {
                *cum = *ccum;
            }
            *ccum = temp;
        }
    } else if log_p != 0 && y < 1e170f64
        || lower != 0 && -37.5193f64 < x && x < 8.2924f64
        || upper != 0 && -8.2924f64 < x && x < 37.5193f64
    {
        xsq = 1.0f64 / (x * x);
        xnum = p[5 as libc::c_int as usize] * xsq;
        xden = xsq;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            xnum = (xnum + p[i as usize]) * xsq;
            xden = (xden + q[i as usize]) * xsq;
            i += 1;
        }
        temp = xsq * (xnum + p[4 as libc::c_int as usize]) / (xden + q[4 as libc::c_int as usize]);
        temp = (0.398942280401432677939946059934f64 - temp) / y;
        xsq = trunc(x * 16 as libc::c_int as libc::c_double) / 16 as libc::c_int as libc::c_double;
        del = (x - xsq) * (x + xsq);
        if log_p != 0 {
            *cum = -xsq * xsq * 0.5f64 + -del * 0.5f64 + log(temp);
            if lower != 0 && x > 0.0f64 || upper != 0 && x <= 0.0f64 {
                *ccum = Rlog1p(-exp(-xsq * xsq * 0.5f64) * exp(-del * 0.5f64) * temp);
            }
        } else {
            *cum = exp(-xsq * xsq * 0.5f64) * exp(-del * 0.5f64) * temp;
            *ccum = 1.0f64 - *cum;
        }
        if x > 0.0f64 {
            temp = *cum;
            if lower != 0 {
                *cum = *ccum;
            }
            *ccum = temp;
        }
    } else if x > 0 as libc::c_int as libc::c_double {
        *cum = if log_p != 0 { 0.0f64 } else { 1.0f64 };
        *ccum = if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    } else {
        *cum = if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
        *ccum = if log_p != 0 { 0.0f64 } else { 1.0f64 };
    };
}
