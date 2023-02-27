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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn R_pow_di(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn unif_rand() -> libc::c_double;
    fn qbinom(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rbinom(mut nin: libc::c_double, mut pp: libc::c_double) -> libc::c_double {
    let mut current_block: u64;
    static mut c: libc::c_double = 0.;
    static mut fm: libc::c_double = 0.;
    static mut npq: libc::c_double = 0.;
    static mut p1: libc::c_double = 0.;
    static mut p2: libc::c_double = 0.;
    static mut p3: libc::c_double = 0.;
    static mut p4: libc::c_double = 0.;
    static mut qn: libc::c_double = 0.;
    static mut xl: libc::c_double = 0.;
    static mut xll: libc::c_double = 0.;
    static mut xlr: libc::c_double = 0.;
    static mut xm: libc::c_double = 0.;
    static mut xr: libc::c_double = 0.;
    static mut psave: libc::c_double = -1.0f64;
    static mut nsave: libc::c_int = -(1 as libc::c_int);
    static mut m: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut f1: libc::c_double = 0.;
    let mut f2: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut w2: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut z2: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut np: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut al: libc::c_double = 0.;
    let mut alv: libc::c_double = 0.;
    let mut amaxp: libc::c_double = 0.;
    let mut ffm: libc::c_double = 0.;
    let mut ynorm: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if R_finite(nin) == 0 {
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
    r = round(nin);
    if r != nin {
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
    if R_finite(pp) == 0 || r < 0 as libc::c_int as libc::c_double || pp < 0.0f64 || pp > 1.0f64 {
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
    if r == 0 as libc::c_int as libc::c_double || pp == 0.0f64 {
        return 0 as libc::c_int as libc::c_double;
    }
    if pp == 1.0f64 {
        return r;
    }
    if r >= 2147483647 as libc::c_int as libc::c_double {
        return qbinom(unif_rand(), r, pp, 0 as libc::c_int, 0 as libc::c_int);
    }
    n = r as libc::c_int;
    p = fmin2(pp, 1.0f64 - pp);
    q = 1.0f64 - p;
    np = n as libc::c_double * p;
    r = p / q;
    g = r * (n + 1 as libc::c_int) as libc::c_double;
    if pp != psave || n != nsave {
        psave = pp;
        nsave = n;
        if np < 30.0f64 {
            qn = R_pow_di(q, n);
            current_block = 18002345992382212654;
        } else {
            ffm = np + p;
            m = ffm as libc::c_int;
            fm = m as libc::c_double;
            npq = np * q;
            p1 = (2.195f64 * sqrt(npq) - 4.6f64 * q) as libc::c_int as libc::c_double + 0.5f64;
            xm = fm + 0.5f64;
            xl = xm - p1;
            xr = xm + p1;
            c = 0.134f64 + 20.5f64 / (15.3f64 + fm);
            al = (ffm - xl) / (ffm - xl * p);
            xll = al * (1.0f64 + 0.5f64 * al);
            al = (xr - ffm) / (xr * q);
            xlr = al * (1.0f64 + 0.5f64 * al);
            p2 = p1 * (1.0f64 + c + c);
            p3 = p2 + c / xll;
            p4 = p3 + c / xlr;
            current_block = 3736434875406665187;
        }
    } else if n == nsave {
        if np < 30.0f64 {
            current_block = 18002345992382212654;
        } else {
            current_block = 3736434875406665187;
        }
    } else {
        current_block = 3736434875406665187;
    }
    's_378: loop {
        match current_block {
            18002345992382212654 => {
                ix = 0 as libc::c_int;
                f = qn;
                u = unif_rand();
                loop {
                    if u < f {
                        break 's_378;
                    }
                    if ix > 110 as libc::c_int {
                        current_block = 18002345992382212654;
                        break;
                    }
                    u -= f;
                    ix += 1;
                    f *= g / ix as libc::c_double - r;
                }
            }
            _ => {
                u = unif_rand() * p4;
                v = unif_rand();
                if u <= p1 {
                    ix = (xm - p1 * v + u) as libc::c_int;
                    break;
                } else {
                    if u <= p2 {
                        x = xl + (u - p1) / c;
                        v = v * c + 1.0f64 - fabs(xm - x) / p1;
                        if v > 1.0f64 || v <= 0.0f64 {
                            current_block = 3736434875406665187;
                            continue;
                        }
                        ix = x as libc::c_int;
                    } else if u > p3 {
                        ix = (xr - log(v) / xlr) as libc::c_int;
                        if ix > n {
                            current_block = 3736434875406665187;
                            continue;
                        }
                        v = v * (u - p3) * xlr;
                    } else {
                        ix = (xl + log(v) / xll) as libc::c_int;
                        if ix < 0 as libc::c_int {
                            current_block = 3736434875406665187;
                            continue;
                        }
                        v = v * (u - p2) * xll;
                    }
                    k = abs(ix - m);
                    if k <= 20 as libc::c_int
                        || k as libc::c_double
                            >= npq / 2 as libc::c_int as libc::c_double
                                - 1 as libc::c_int as libc::c_double
                    {
                        f = 1.0f64;
                        if m < ix {
                            i = m + 1 as libc::c_int;
                            while i <= ix {
                                f *= g / i as libc::c_double - r;
                                i += 1;
                            }
                        } else if m != ix {
                            i = ix + 1 as libc::c_int;
                            while i <= m {
                                f /= g / i as libc::c_double - r;
                                i += 1;
                            }
                        }
                        if v <= f {
                            break;
                        } else {
                            current_block = 3736434875406665187;
                        }
                    } else {
                        amaxp = k as libc::c_double / npq
                            * ((k as libc::c_double * (k as libc::c_double / 3.0f64 + 0.625f64)
                                + 0.1666666666666f64)
                                / npq
                                + 0.5f64);
                        ynorm = (-k * k) as libc::c_double / (2.0f64 * npq);
                        alv = log(v);
                        if alv < ynorm - amaxp {
                            break;
                        }
                        if !(alv <= ynorm + amaxp) {
                            current_block = 3736434875406665187;
                            continue;
                        }
                        x1 = (ix + 1 as libc::c_int) as libc::c_double;
                        f1 = fm + 1.0f64;
                        z = (n + 1 as libc::c_int) as libc::c_double - fm;
                        w = (n - ix) as libc::c_double + 1.0f64;
                        z2 = z * z;
                        x2 = x1 * x1;
                        f2 = f1 * f1;
                        w2 = w * w;
                        if alv
                            <= xm * log(f1 / x1)
                                + ((n - m) as libc::c_double + 0.5f64) * log(z / w)
                                + (ix - m) as libc::c_double * log(w * p / (x1 * q))
                                + (13860.0f64
                                    - (462.0f64 - (132.0f64 - (99.0f64 - 140.0f64 / f2) / f2) / f2)
                                        / f2)
                                    / f1
                                    / 166320.0f64
                                + (13860.0f64
                                    - (462.0f64 - (132.0f64 - (99.0f64 - 140.0f64 / z2) / z2) / z2)
                                        / z2)
                                    / z
                                    / 166320.0f64
                                + (13860.0f64
                                    - (462.0f64 - (132.0f64 - (99.0f64 - 140.0f64 / x2) / x2) / x2)
                                        / x2)
                                    / x1
                                    / 166320.0f64
                                + (13860.0f64
                                    - (462.0f64 - (132.0f64 - (99.0f64 - 140.0f64 / w2) / w2) / w2)
                                        / w2)
                                    / w
                                    / 166320.0f64
                        {
                            break;
                        } else {
                            current_block = 3736434875406665187;
                        }
                    }
                }
            }
        }
    }
    if psave > 0.5f64 {
        ix = n - ix;
    }
    return ix as libc::c_double;
}
