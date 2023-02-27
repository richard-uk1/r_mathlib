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
    fn round(_: libc::c_double) -> libc::c_double;
    fn unif_rand() -> libc::c_double;
    fn rbinom(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn qhyper(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn REprintf(_: *const libc::c_char, _: ...);
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn imin2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn imax2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
unsafe extern "C" fn afc(mut i: libc::c_int) -> libc::c_double {
    static mut al: [libc::c_double; 8] = [
        0.0f64,
        0.0f64,
        0.69314718055994530941723212145817f64,
        1.79175946922805500081247735838070f64,
        3.17805383034794561964694160129705f64,
        4.78749174278204599424770093452324f64,
        6.57925121201010099506017829290394f64,
        8.52516136106541430016553103634712f64,
    ];
    if i < 0 as libc::c_int {
        printf(
            b"rhyper.c: afc(i), i=%d < 0 -- SHOULD NOT HAPPEN!\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
        return -(1 as libc::c_int) as libc::c_double;
    }
    if i <= 7 as libc::c_int {
        return al[i as usize];
    }
    let mut di: libc::c_double = i as libc::c_double;
    let mut i2: libc::c_double = di * di;
    return (di + 0.5f64) * log(di) - di
        + 0.918938533204672741780329736406f64
        + (0.0833333333333333f64 - 0.00277777777777778f64 / i2) / di;
}
#[no_mangle]
pub unsafe extern "C" fn rhyper(
    mut nn1in: libc::c_double,
    mut nn2in: libc::c_double,
    mut kkin: libc::c_double,
) -> libc::c_double {
    let mut nn1: libc::c_int = 0;
    let mut nn2: libc::c_int = 0;
    let mut kk: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    let mut setup1: Rboolean = FALSE;
    let mut setup2: Rboolean = FALSE;
    static mut ks: libc::c_int = -(1 as libc::c_int);
    static mut n1s: libc::c_int = -(1 as libc::c_int);
    static mut n2s: libc::c_int = -(1 as libc::c_int);
    static mut m: libc::c_int = 0;
    static mut minjx: libc::c_int = 0;
    static mut maxjx: libc::c_int = 0;
    static mut k: libc::c_int = 0;
    static mut n1: libc::c_int = 0;
    static mut n2: libc::c_int = 0;
    static mut N: libc::c_double = 0.;
    static mut w: libc::c_double = 0.;
    static mut a: libc::c_double = 0.;
    static mut d: libc::c_double = 0.;
    static mut s: libc::c_double = 0.;
    static mut xl: libc::c_double = 0.;
    static mut xr: libc::c_double = 0.;
    static mut kl: libc::c_double = 0.;
    static mut kr: libc::c_double = 0.;
    static mut lamdl: libc::c_double = 0.;
    static mut lamdr: libc::c_double = 0.;
    static mut p1: libc::c_double = 0.;
    static mut p2: libc::c_double = 0.;
    static mut p3: libc::c_double = 0.;
    if R_finite(nn1in) == 0 || R_finite(nn2in) == 0 || R_finite(kkin) == 0 {
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
    nn1in = round(nn1in);
    nn2in = round(nn2in);
    kkin = round(kkin);
    if nn1in < 0 as libc::c_int as libc::c_double
        || nn2in < 0 as libc::c_int as libc::c_double
        || kkin < 0 as libc::c_int as libc::c_double
        || kkin > nn1in + nn2in
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
    if nn1in >= 2147483647 as libc::c_int as libc::c_double
        || nn2in >= 2147483647 as libc::c_int as libc::c_double
        || kkin >= 2147483647 as libc::c_int as libc::c_double
    {
        if kkin == 1.0f64 {
            return rbinom(kkin, nn1in / (nn1in + nn2in));
        }
        return qhyper(
            unif_rand(),
            nn1in,
            nn2in,
            kkin,
            FALSE as libc::c_int,
            FALSE as libc::c_int,
        );
    }
    nn1 = nn1in as libc::c_int;
    nn2 = nn2in as libc::c_int;
    kk = kkin as libc::c_int;
    if nn1 != n1s || nn2 != n2s {
        setup1 = TRUE;
        setup2 = TRUE;
    } else if kk != ks {
        setup1 = FALSE;
        setup2 = TRUE;
    } else {
        setup1 = FALSE;
        setup2 = FALSE;
    }
    if setup1 as u64 != 0 {
        n1s = nn1;
        n2s = nn2;
        N = nn1 as libc::c_double + nn2 as libc::c_double;
        if nn1 <= nn2 {
            n1 = nn1;
            n2 = nn2;
        } else {
            n1 = nn2;
            n2 = nn1;
        }
    }
    if setup2 as u64 != 0 {
        ks = kk;
        if kk as libc::c_double + kk as libc::c_double >= N {
            k = (N - kk as libc::c_double) as libc::c_int;
        } else {
            k = kk;
        }
    }
    if setup1 as libc::c_uint != 0 || setup2 as libc::c_uint != 0 {
        m = ((k as libc::c_double + 1.0f64) * (n1 as libc::c_double + 1.0f64) / (N + 2.0f64))
            as libc::c_int;
        minjx = imax2(0 as libc::c_int, k - n2);
        maxjx = imin2(n1, k);
    }
    if minjx == maxjx {
        ix = maxjx;
    } else if m - minjx < 10 as libc::c_int {
        static mut scale: libc::c_double = 1e25f64;
        static mut con: libc::c_double = 57.5646273248511421f64;
        if setup1 as libc::c_uint != 0 || setup2 as libc::c_uint != 0 {
            let mut lw: libc::c_double = 0.;
            if k < n2 {
                lw = afc(n2) + afc(n1 + n2 - k) - afc(n2 - k) - afc(n1 + n2);
            } else {
                lw = afc(n1) + afc(k) - afc(k - n2) - afc(n1 + n2);
            }
            w = exp(lw + con);
        }
        let mut p: libc::c_double = 0.;
        let mut u: libc::c_double = 0.;
        '_L10: loop {
            p = w;
            ix = minjx;
            u = unif_rand() * scale;
            loop {
                if !(u > p) {
                    break '_L10;
                }
                u -= p;
                p *= (n1 as libc::c_double - ix as libc::c_double) * (k - ix) as libc::c_double;
                ix += 1;
                p = p / ix as libc::c_double / (n2 - k + ix) as libc::c_double;
                if ix > maxjx {
                    break;
                }
            }
        }
    } else {
        let mut u_0: libc::c_double = 0.;
        let mut v: libc::c_double = 0.;
        if setup1 as libc::c_uint != 0 || setup2 as libc::c_uint != 0 {
            s = sqrt(
                (N - k as libc::c_double)
                    * k as libc::c_double
                    * n1 as libc::c_double
                    * n2 as libc::c_double
                    / (N - 1 as libc::c_int as libc::c_double)
                    / N
                    / N,
            );
            d = (1.5f64 * s) as libc::c_int as libc::c_double + 0.5f64;
            xl = m as libc::c_double - d + 0.5f64;
            xr = m as libc::c_double + d + 0.5f64;
            a = afc(m) + afc(n1 - m) + afc(k - m) + afc(n2 - k + m);
            kl = exp(a
                - afc(xl as libc::c_int)
                - afc((n1 as libc::c_double - xl) as libc::c_int)
                - afc((k as libc::c_double - xl) as libc::c_int)
                - afc(((n2 - k) as libc::c_double + xl) as libc::c_int));
            kr = exp(a
                - afc((xr - 1 as libc::c_int as libc::c_double) as libc::c_int)
                - afc(
                    (n1 as libc::c_double - xr + 1 as libc::c_int as libc::c_double) as libc::c_int,
                )
                - afc(
                    (k as libc::c_double - xr + 1 as libc::c_int as libc::c_double) as libc::c_int,
                )
                - afc(
                    ((n2 - k) as libc::c_double + xr - 1 as libc::c_int as libc::c_double)
                        as libc::c_int,
                ));
            lamdl = -log(xl * ((n2 - k) as libc::c_double + xl)
                / (n1 as libc::c_double - xl + 1 as libc::c_int as libc::c_double)
                / (k as libc::c_double - xl + 1 as libc::c_int as libc::c_double));
            lamdr = -log(
                (n1 as libc::c_double - xr + 1 as libc::c_int as libc::c_double)
                    * (k as libc::c_double - xr + 1 as libc::c_int as libc::c_double)
                    / xr
                    / ((n2 - k) as libc::c_double + xr),
            );
            p1 = d + d;
            p2 = p1 + kl / lamdl;
            p3 = p2 + kr / lamdr;
        }
        let mut n_uv: libc::c_int = 0 as libc::c_int;
        loop {
            u_0 = unif_rand() * p3;
            v = unif_rand();
            n_uv += 1;
            if n_uv >= 10000 as libc::c_int {
                REprintf(
                    b"rhyper(*, n1=%d, n2=%d, k=%d): branch III: giving up after %d rejections\n\0"
                        as *const u8 as *const libc::c_char,
                    nn1,
                    nn2,
                    kk,
                    n_uv,
                );
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
                                as *const u8
                                as *const libc::c_char
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
            if u_0 < p1 {
                ix = (xl + u_0) as libc::c_int;
            } else if u_0 <= p2 {
                ix = (xl + log(v) / lamdl) as libc::c_int;
                if ix < minjx {
                    continue;
                }
                v = v * (u_0 - p1) * lamdl;
            } else {
                ix = (xr - log(v) / lamdr) as libc::c_int;
                if ix > maxjx {
                    continue;
                }
                v = v * (u_0 - p2) * lamdr;
            }
            let mut reject: Rboolean = TRUE;
            if m < 100 as libc::c_int || ix <= 50 as libc::c_int {
                let mut i: libc::c_int = 0;
                let mut f: libc::c_double = 1.0f64;
                if m < ix {
                    i = m + 1 as libc::c_int;
                    while i <= ix {
                        f = f
                            * (n1 - i + 1 as libc::c_int) as libc::c_double
                            * (k - i + 1 as libc::c_int) as libc::c_double
                            / (n2 - k + i) as libc::c_double
                            / i as libc::c_double;
                        i += 1;
                    }
                } else if m > ix {
                    i = ix + 1 as libc::c_int;
                    while i <= m {
                        f = f * i as libc::c_double * (n2 - k + i) as libc::c_double
                            / (n1 - i + 1 as libc::c_int) as libc::c_double
                            / (k - i + 1 as libc::c_int) as libc::c_double;
                        i += 1;
                    }
                }
                if v <= f {
                    reject = FALSE;
                }
            } else {
                static mut deltal: libc::c_double = 0.0078f64;
                static mut deltau: libc::c_double = 0.0034f64;
                let mut e: libc::c_double = 0.;
                let mut g: libc::c_double = 0.;
                let mut r: libc::c_double = 0.;
                let mut t: libc::c_double = 0.;
                let mut y: libc::c_double = 0.;
                let mut de: libc::c_double = 0.;
                let mut dg: libc::c_double = 0.;
                let mut dr: libc::c_double = 0.;
                let mut ds: libc::c_double = 0.;
                let mut dt: libc::c_double = 0.;
                let mut gl: libc::c_double = 0.;
                let mut gu: libc::c_double = 0.;
                let mut nk: libc::c_double = 0.;
                let mut nm: libc::c_double = 0.;
                let mut ub: libc::c_double = 0.;
                let mut xk: libc::c_double = 0.;
                let mut xm: libc::c_double = 0.;
                let mut xn: libc::c_double = 0.;
                let mut y1: libc::c_double = 0.;
                let mut ym: libc::c_double = 0.;
                let mut yn: libc::c_double = 0.;
                let mut yk: libc::c_double = 0.;
                let mut alv: libc::c_double = 0.;
                y = ix as libc::c_double;
                y1 = y + 1.0f64;
                ym = y - m as libc::c_double;
                yn = n1 as libc::c_double - y + 1.0f64;
                yk = k as libc::c_double - y + 1.0f64;
                nk = (n2 - k) as libc::c_double + y1;
                r = -ym / y1;
                s = ym / yn;
                t = ym / yk;
                e = -ym / nk;
                g = yn * yk / (y1 * nk) - 1.0f64;
                dg = 1.0f64;
                if g < 0.0f64 {
                    dg = 1.0f64 + g;
                }
                gu = g * (1.0f64 + g * (-0.5f64 + g / 3.0f64));
                gl = gu - 0.25f64 * (g * g * g * g) / dg;
                xm = m as libc::c_double + 0.5f64;
                xn = (n1 - m) as libc::c_double + 0.5f64;
                xk = (k - m) as libc::c_double + 0.5f64;
                nm = (n2 - k) as libc::c_double + xm;
                ub = y * gu - m as libc::c_double * gl
                    + deltau
                    + xm * r * (1.0f64 + r * (-0.5f64 + r / 3.0f64))
                    + xn * s * (1.0f64 + s * (-0.5f64 + s / 3.0f64))
                    + xk * t * (1.0f64 + t * (-0.5f64 + t / 3.0f64))
                    + nm * e * (1.0f64 + e * (-0.5f64 + e / 3.0f64));
                alv = log(v);
                if alv > ub {
                    reject = TRUE;
                } else {
                    dr = xm * (r * r * r * r);
                    if r < 0.0f64 {
                        dr /= 1.0f64 + r;
                    }
                    ds = xn * (s * s * s * s);
                    if s < 0.0f64 {
                        ds /= 1.0f64 + s;
                    }
                    dt = xk * (t * t * t * t);
                    if t < 0.0f64 {
                        dt /= 1.0f64 + t;
                    }
                    de = nm * (e * e * e * e);
                    if e < 0.0f64 {
                        de /= 1.0f64 + e;
                    }
                    if alv
                        < ub - 0.25f64 * (dr + ds + dt + de) + (y + m as libc::c_double) * (gl - gu)
                            - deltal
                    {
                        reject = FALSE;
                    } else if alv <= a - afc(ix) - afc(n1 - ix) - afc(k - ix) - afc(n2 - k + ix) {
                        reject = FALSE;
                    } else {
                        reject = TRUE;
                    }
                }
            }
            if !(reject as u64 != 0) {
                break;
            }
        }
    }
    if kk as libc::c_double + kk as libc::c_double >= N {
        if nn1 > nn2 {
            ix = kk - nn2 + ix;
        } else {
            ix = nn1 - ix;
        }
    } else if nn1 > nn2 {
        ix = kk - ix;
    }
    return ix as libc::c_double;
}
