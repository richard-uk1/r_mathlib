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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_gamma_cody(_: libc::c_double) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn sinpi(_: libc::c_double) -> libc::c_double;
    fn bessel_y(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cospi(_: libc::c_double) -> libc::c_double;
    fn bessel_y_ex(_: libc::c_double, _: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_j(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut na: libc::c_double = 0.;
    let mut bj: *mut libc::c_double = 0 as *mut libc::c_double;
    if x.is_nan() as i32 != 0 as libc::c_int || alpha.is_nan() as i32 != 0 as libc::c_int {
        return x + alpha;
    }
    if x < 0 as libc::c_int as libc::c_double {
        if 2 as libc::c_int > 1 as libc::c_int {
            let mut msg: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 2 as libc::c_int {
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
            printf(msg, b"bessel_j\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    na = floor(alpha);
    if alpha < 0 as libc::c_int as libc::c_double {
        return (if alpha - na == 0.5f64 {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_j(x, -alpha) * cospi(alpha)
        }) + (if alpha == na {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_y(x, -alpha) * sinpi(alpha)
        });
    } else {
        if alpha > 1e7f64 {
            printf(
                b"besselJ(x, nu): nu=%g too large for bessel_j() algorithm\0" as *const u8
                    as *const libc::c_char,
                alpha,
            );
            return 0.0f64 / 0.0f64;
        }
    }
    nb = 1 as libc::c_int + na as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    bj = calloc(
        nb as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if bj.is_null() {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            b"bessel_j allocation error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    J_bessel(&mut x, &mut alpha, &mut nb, bj, &mut ncalc);
    if ncalc != nb {
        if ncalc < 0 as libc::c_int {
            printf(
                b"bessel_j(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                    as *const u8 as *const libc::c_char,
                x,
                ncalc,
                nb,
                alpha,
            );
        } else {
            printf(
                b"bessel_j(%g,nu=%g): precision lost in result\n\0" as *const u8
                    as *const libc::c_char,
                x,
                alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
            );
        }
    }
    x = *bj.offset((nb - 1 as libc::c_int) as isize);
    free(bj as *mut libc::c_void);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_j_ex(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut bj: *mut libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut na: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int || alpha.is_nan() as i32 != 0 as libc::c_int {
        return x + alpha;
    }
    if x < 0 as libc::c_int as libc::c_double {
        if 2 as libc::c_int > 1 as libc::c_int {
            let mut msg: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 2 as libc::c_int {
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
            printf(msg, b"bessel_j\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    na = floor(alpha);
    if alpha < 0 as libc::c_int as libc::c_double {
        return (if alpha - na == 0.5f64 {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_j_ex(x, -alpha, bj) * cospi(alpha)
        }) + (if alpha == na {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_y_ex(x, -alpha, bj) * sinpi(alpha)
        });
    } else {
        if alpha > 1e7f64 {
            printf(
                b"besselJ(x, nu): nu=%g too large for bessel_j() algorithm\0" as *const u8
                    as *const libc::c_char,
                alpha,
            );
            return 0.0f64 / 0.0f64;
        }
    }
    nb = 1 as libc::c_int + na as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    J_bessel(&mut x, &mut alpha, &mut nb, bj, &mut ncalc);
    if ncalc != nb {
        if ncalc < 0 as libc::c_int {
            printf(
                b"bessel_j(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                    as *const u8 as *const libc::c_char,
                x,
                ncalc,
                nb,
                alpha,
            );
        } else {
            printf(
                b"bessel_j(%g,nu=%g): precision lost in result\n\0" as *const u8
                    as *const libc::c_char,
                x,
                alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
            );
        }
    }
    x = *bj.offset((nb - 1 as libc::c_int) as isize);
    return x;
}
unsafe extern "C" fn J_bessel(
    mut x: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
    mut nb: *mut libc::c_int,
    mut b: *mut libc::c_double,
    mut ncalc: *mut libc::c_int,
) {
    let mut current_block: u64;
    static mut pi2: libc::c_double = 0.636619772367581343075535f64;
    static mut twopi1: libc::c_double = 6.28125f64;
    static mut twopi2: libc::c_double = 0.001935307179586476925286767f64;
    static mut fact: [libc::c_double; 25] = [
        1.0f64,
        1.0f64,
        2.0f64,
        6.0f64,
        24.0f64,
        120.0f64,
        720.0f64,
        5040.0f64,
        40320.0f64,
        362880.0f64,
        3628800.0f64,
        39916800.0f64,
        479001600.0f64,
        6227020800.0f64,
        87178291200.0f64,
        1.307674368e12f64,
        2.0922789888e13f64,
        3.55687428096e14f64,
        6.402373705728e15f64,
        1.21645100408832e17f64,
        2.43290200817664e18f64,
        5.109094217170944e19f64,
        1.12400072777760768e21f64,
        2.585201673888497664e22f64,
        6.2044840173323943936e23f64,
    ];
    let mut nend: libc::c_int = 0;
    let mut intx: libc::c_int = 0;
    let mut nbmx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nstart: libc::c_int = 0;
    let mut nu: libc::c_double = 0.;
    let mut twonu: libc::c_double = 0.;
    let mut capp: libc::c_double = 0.;
    let mut capq: libc::c_double = 0.;
    let mut pold: libc::c_double = 0.;
    let mut vcos: libc::c_double = 0.;
    let mut test: libc::c_double = 0.;
    let mut vsin: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut alpem: libc::c_double = 0.;
    let mut halfx: libc::c_double = 0.;
    let mut aa: libc::c_double = 0.;
    let mut bb: libc::c_double = 0.;
    let mut cc: libc::c_double = 0.;
    let mut psave: libc::c_double = 0.;
    let mut plast: libc::c_double = 0.;
    let mut tover: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut alp2em: libc::c_double = 0.;
    let mut em: libc::c_double = 0.;
    let mut en: libc::c_double = 0.;
    let mut xc: libc::c_double = 0.;
    let mut xk: libc::c_double = 0.;
    let mut xm: libc::c_double = 0.;
    let mut psavel: libc::c_double = 0.;
    let mut gnu: libc::c_double = 0.;
    let mut xin: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    b = b.offset(-1);
    nu = *alpha;
    twonu = nu + nu;
    if *nb > 0 as libc::c_int && *x >= 0.0f64 && 0.0f64 <= nu && nu < 1.0f64 {
        *ncalc = *nb;
        if *x > 1e5f64 {
            if 2 as libc::c_int > 1 as libc::c_int {
                let mut msg: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 2 as libc::c_int {
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
                printf(msg, b"J_bessel\0" as *const u8 as *const libc::c_char);
            }
            i = 1 as libc::c_int;
            while i <= *nb {
                *b.offset(i as isize) = 0.0f64;
                i += 1;
            }
            return;
        }
        intx = *x as libc::c_int;
        i = 1 as libc::c_int;
        while i <= *nb {
            *b.offset(i as isize) = 0.0f64;
            i += 1;
        }
        if *x < 1e-4f64 {
            alpem = 1.0f64 + nu;
            halfx = if *x > 8.9e-308f64 {
                0.5f64 * *x
            } else {
                0.0f64
            };
            aa = if nu != 0.0f64 {
                pow(halfx, nu) / (nu * Rf_gamma_cody(nu))
            } else {
                1.0f64
            };
            bb = if *x + 1.0f64 > 1.0f64 {
                -halfx * halfx
            } else {
                0.0f64
            };
            *b.offset(1 as libc::c_int as isize) = aa + aa * bb / alpem;
            if *x != 0.0f64 && *b.offset(1 as libc::c_int as isize) == 0.0f64 {
                *ncalc = 0 as libc::c_int;
            }
            if *nb != 1 as libc::c_int {
                if *x <= 0.0f64 {
                    n = 2 as libc::c_int;
                    while n <= *nb {
                        *b.offset(n as isize) = 0.0f64;
                        n += 1;
                    }
                } else {
                    if bb == 0.0f64 {
                        tover = (8.9e-308f64 + 8.9e-308f64) / *x;
                    } else {
                        tover = 8.9e-308f64 / bb;
                    }
                    cc = halfx;
                    n = 2 as libc::c_int;
                    while n <= *nb {
                        aa /= alpem;
                        alpem += 1.0f64;
                        aa *= cc;
                        if aa <= tover * alpem {
                            aa = 0.0f64;
                        }
                        *b.offset(n as isize) = aa + aa * bb / alpem;
                        if *b.offset(n as isize) == 0.0f64 && *ncalc > n {
                            *ncalc = n - 1 as libc::c_int;
                        }
                        n += 1;
                    }
                }
            }
        } else if *x > 25.0f64 && *nb <= intx + 1 as libc::c_int {
            xc = sqrt(pi2 / *x);
            xin = 1 as libc::c_int as libc::c_double
                / (64 as libc::c_int as libc::c_double * *x * *x);
            if *x >= 130.0f64 {
                m = 4 as libc::c_int;
            } else if *x >= 35.0f64 {
                m = 8 as libc::c_int;
            } else {
                m = 11 as libc::c_int;
            }
            xm = 4.0f64 * m as libc::c_double;
            t = trunc(*x / (twopi1 + twopi2) + 0.5f64);
            z = *x - t * twopi1 - t * twopi2 - (nu + 0.5f64) / pi2;
            vsin = sin(z);
            vcos = cos(z);
            gnu = twonu;
            i = 1 as libc::c_int;
            while i <= 2 as libc::c_int {
                s = (xm - 1.0f64 - gnu) * (xm - 1.0f64 + gnu) * xin * 0.5f64;
                t = (gnu - (xm - 3.0f64)) * (gnu + (xm - 3.0f64));
                t1 = (gnu - (xm + 1.0f64)) * (gnu + (xm + 1.0f64));
                k = m + m;
                capp = s * t / fact[k as usize];
                capq = s * t1 / fact[(k + 1 as libc::c_int) as usize];
                xk = xm;
                while k >= 4 as libc::c_int {
                    xk -= 4.0f64;
                    s = (xk - 1.0f64 - gnu) * (xk - 1.0f64 + gnu);
                    t1 = t;
                    t = (gnu - (xk - 3.0f64)) * (gnu + (xk - 3.0f64));
                    capp = (capp + 1.0f64 / fact[(k - 2 as libc::c_int) as usize]) * s * t * xin;
                    capq = (capq + 1.0f64 / fact[(k - 1 as libc::c_int) as usize]) * s * t1 * xin;
                    k -= 2 as libc::c_int;
                }
                capp += 1.0f64;
                capq = (capq + 1.0f64) * (gnu * gnu - 1.0f64) * (0.125f64 / *x);
                *b.offset(i as isize) = xc * (capp * vcos - capq * vsin);
                if *nb == 1 as libc::c_int {
                    return;
                }
                t = vsin;
                vsin = -vcos;
                vcos = t;
                gnu += 2.0f64;
                i += 1;
            }
            if *nb > 2 as libc::c_int {
                gnu = twonu + 2.0f64;
                j = 3 as libc::c_int;
                while j <= *nb {
                    *b.offset(j as isize) = gnu * *b.offset((j - 1 as libc::c_int) as isize) / *x
                        - *b.offset((j - 2 as libc::c_int) as isize);
                    j += 1;
                    gnu += 2.0f64;
                }
            }
        } else {
            nbmx = *nb - intx;
            n = intx + 1 as libc::c_int;
            en = (n + n) as libc::c_double + twonu;
            plast = 1.0f64;
            p = en / *x;
            test = 1e16f64 + 1e16f64;
            if nbmx >= 3 as libc::c_int {
                tover = 1e308f64 / 1e16f64;
                nstart = intx + 2 as libc::c_int;
                nend = *nb - 1 as libc::c_int;
                en = (nstart + nstart) as libc::c_double - 2.0f64 + twonu;
                k = nstart;
                's_461: loop {
                    if !(k <= nend) {
                        current_block = 14027225908442187354;
                        break;
                    }
                    n = k;
                    en += 2.0f64;
                    pold = plast;
                    plast = p;
                    p = en * plast / *x - pold;
                    if p > tover {
                        tover = 1e308f64;
                        p /= tover;
                        plast /= tover;
                        psave = p;
                        psavel = plast;
                        nstart = n + 1 as libc::c_int;
                        loop {
                            n += 1;
                            en += 2.0f64;
                            pold = plast;
                            plast = p;
                            p = en * plast / *x - pold;
                            if !(p <= 1.0f64) {
                                break;
                            }
                        }
                        bb = en / *x;
                        test = pold * plast * (0.5f64 - 0.5f64 / (bb * bb));
                        test /= 1e16f64;
                        p = plast * tover;
                        n -= 1;
                        en -= 2.0f64;
                        nend = if *nb <= n { *nb } else { n };
                        l = nstart;
                        while l <= nend {
                            pold = psavel;
                            psavel = psave;
                            psave = en * psavel / *x - pold;
                            if psave * psavel > test {
                                *ncalc = l - 1 as libc::c_int;
                                current_block = 15078410231313032900;
                                break 's_461;
                            } else {
                                l += 1;
                            }
                        }
                        *ncalc = nend;
                        current_block = 15078410231313032900;
                        break;
                    } else {
                        k += 1;
                    }
                }
                match current_block {
                    15078410231313032900 => {}
                    _ => {
                        n = nend;
                        en = (n + n) as libc::c_double + twonu;
                        test = fmax2(test, sqrt(plast * 1e16f64) * sqrt(p + p));
                        current_block = 13014351284863956202;
                    }
                }
            } else {
                current_block = 13014351284863956202;
            }
            match current_block {
                13014351284863956202 => loop {
                    n += 1;
                    en += 2.0f64;
                    pold = plast;
                    plast = p;
                    p = en * plast / *x - pold;
                    if !(p < test) {
                        break;
                    }
                },
                _ => {}
            }
            n += 1;
            en += 2.0f64;
            bb = 0.0f64;
            aa = 1.0f64 / p;
            m = n / 2 as libc::c_int;
            em = m as libc::c_double;
            m = (n << 1 as libc::c_int) - (m << 2 as libc::c_int);
            if m == 0 as libc::c_int {
                sum = 0.0f64;
            } else {
                alpem = em - 1.0f64 + nu;
                alp2em = em + em + nu;
                sum = aa * alpem * alp2em / em;
            }
            nend = n - *nb;
            l = 1 as libc::c_int;
            while l <= nend {
                n -= 1;
                en -= 2.0f64;
                cc = bb;
                bb = aa;
                aa = en * bb / *x - cc;
                m = if m != 0 {
                    0 as libc::c_int
                } else {
                    2 as libc::c_int
                };
                if m != 0 as libc::c_int {
                    em -= 1.0f64;
                    alp2em = em + em + nu;
                    if n == 1 as libc::c_int {
                        break;
                    }
                    alpem = em - 1.0f64 + nu;
                    if alpem == 0.0f64 {
                        alpem = 1.0f64;
                    }
                    sum = (sum + aa * alp2em) * alpem / em;
                }
                l += 1;
            }
            *b.offset(n as isize) = aa;
            if nend >= 0 as libc::c_int {
                if *nb <= 1 as libc::c_int {
                    if nu + 1.0f64 == 1.0f64 {
                        alp2em = 1.0f64;
                    } else {
                        alp2em = nu;
                    }
                    sum += *b.offset(1 as libc::c_int as isize) * alp2em;
                    current_block = 7803489152275420601;
                } else {
                    n -= 1;
                    en -= 2.0f64;
                    *b.offset(n as isize) = en * aa / *x - bb;
                    if n == 1 as libc::c_int {
                        current_block = 4148857094682238542;
                    } else {
                        m = if m != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if m != 0 as libc::c_int {
                            em -= 1.0f64;
                            alp2em = em + em + nu;
                            alpem = em - 1.0f64 + nu;
                            if alpem == 0.0f64 {
                                alpem = 1.0f64;
                            }
                            sum = (sum + *b.offset(n as isize) * alp2em) * alpem / em;
                        }
                        current_block = 7371321987304394147;
                    }
                }
            } else {
                current_block = 7371321987304394147;
            }
            match current_block {
                7371321987304394147 => {
                    n = n - 1 as libc::c_int;
                    while n >= 2 as libc::c_int {
                        en -= 2.0f64;
                        *b.offset(n as isize) = en * *b.offset((n + 1 as libc::c_int) as isize)
                            / *x
                            - *b.offset((n + 2 as libc::c_int) as isize);
                        m = if m != 0 {
                            0 as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if m != 0 as libc::c_int {
                            em -= 1.0f64;
                            alp2em = em + em + nu;
                            alpem = em - 1.0f64 + nu;
                            if alpem == 0.0f64 {
                                alpem = 1.0f64;
                            }
                            sum = (sum + *b.offset(n as isize) * alp2em) * alpem / em;
                        }
                        n -= 1;
                    }
                    *b.offset(1 as libc::c_int as isize) =
                        2.0f64 * (nu + 1.0f64) * *b.offset(2 as libc::c_int as isize) / *x
                            - *b.offset(3 as libc::c_int as isize);
                    current_block = 4148857094682238542;
                }
                _ => {}
            }
            match current_block {
                4148857094682238542 => {
                    em -= 1.0f64;
                    alp2em = em + em + nu;
                    if alp2em == 0.0f64 {
                        alp2em = 1.0f64;
                    }
                    sum += *b.offset(1 as libc::c_int as isize) * alp2em;
                }
                _ => {}
            }
            if fabs(nu) > 1e-15f64 {
                sum *= Rf_gamma_cody(nu) * pow(0.5f64 * *x, -nu);
            }
            aa = 8.9e-308f64;
            if sum > 1.0f64 {
                aa *= sum;
            }
            n = 1 as libc::c_int;
            while n <= *nb {
                if fabs(*b.offset(n as isize)) < aa {
                    *b.offset(n as isize) = 0.0f64;
                } else {
                    *b.offset(n as isize) /= sum;
                }
                n += 1;
            }
        }
    } else {
        *b.offset(1 as libc::c_int as isize) = 0.0f64;
        *ncalc = (if *nb <= 0 as libc::c_int {
            *nb
        } else {
            0 as libc::c_int
        }) - 1 as libc::c_int;
    };
}
