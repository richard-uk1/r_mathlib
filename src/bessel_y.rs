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
    fn log(_: libc::c_double) -> libc::c_double;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn bessel_j(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn bessel_j_ex(_: libc::c_double, _: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn cospi(_: libc::c_double) -> libc::c_double;
    fn sinpi(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_y(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut na: libc::c_double = 0.;
    let mut by: *mut libc::c_double = 0 as *mut libc::c_double;
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
            printf(msg, b"bessel_y\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    na = floor(alpha);
    if alpha < 0 as libc::c_int as libc::c_double {
        return (if alpha - na == 0.5f64 {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_y(x, -alpha) * cospi(alpha)
        }) - (if alpha == na {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_j(x, -alpha) * sinpi(alpha)
        });
    } else {
        if alpha > 1e7f64 {
            printf(
                b"besselY(x, nu): nu=%g too large for bessel_y() algorithm\0" as *const u8
                    as *const libc::c_char,
                alpha,
            );
            return 0.0f64 / 0.0f64;
        }
    }
    nb = 1 as libc::c_int + na as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    by = calloc(
        nb as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if by.is_null() {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            b"bessel_y allocation error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    Y_bessel(&mut x, &mut alpha, &mut nb, by, &mut ncalc);
    if ncalc != nb {
        if ncalc == -(1 as libc::c_int) {
            free(by as *mut libc::c_void);
            return 1.0f64 / 0.0f64;
        } else {
            if ncalc < -(1 as libc::c_int) {
                printf(
                    b"bessel_y(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                        as *const u8 as *const libc::c_char,
                    x,
                    ncalc,
                    nb,
                    alpha,
                );
            } else {
                printf(
                    b"bessel_y(%g,nu=%g): precision lost in result\n\0" as *const u8
                        as *const libc::c_char,
                    x,
                    alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
                );
            }
        }
    }
    x = *by.offset((nb - 1 as libc::c_int) as isize);
    free(by as *mut libc::c_void);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_y_ex(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut by: *mut libc::c_double,
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
            printf(msg, b"bessel_y\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    na = floor(alpha);
    if alpha < 0 as libc::c_int as libc::c_double {
        return (if alpha - na == 0.5f64 {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_y_ex(x, -alpha, by) * cospi(alpha)
        }) - (if alpha == na {
            0 as libc::c_int as libc::c_double
        } else {
            bessel_j_ex(x, -alpha, by) * sinpi(alpha)
        });
    } else {
        if alpha > 1e7f64 {
            printf(
                b"besselY(x, nu): nu=%g too large for bessel_y() algorithm\0" as *const u8
                    as *const libc::c_char,
                alpha,
            );
            return 0.0f64 / 0.0f64;
        }
    }
    nb = 1 as libc::c_int + na as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    Y_bessel(&mut x, &mut alpha, &mut nb, by, &mut ncalc);
    if ncalc != nb {
        if ncalc == -(1 as libc::c_int) {
            return 1.0f64 / 0.0f64;
        } else {
            if ncalc < -(1 as libc::c_int) {
                printf(
                    b"bessel_y(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                        as *const u8 as *const libc::c_char,
                    x,
                    ncalc,
                    nb,
                    alpha,
                );
            } else {
                printf(
                    b"bessel_y(%g,nu=%g): precision lost in result\n\0" as *const u8
                        as *const libc::c_char,
                    x,
                    alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
                );
            }
        }
    }
    x = *by.offset((nb - 1 as libc::c_int) as isize);
    return x;
}
unsafe extern "C" fn Y_bessel(
    mut x: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
    mut nb: *mut libc::c_int,
    mut by: *mut libc::c_double,
    mut ncalc: *mut libc::c_int,
) {
    static mut fivpi: libc::c_double = 15.707963267948966192f64;
    static mut pim5: libc::c_double = 0.70796326794896619231f64;
    static mut ch: [libc::c_double; 21] = [
        -6.7735241822398840964e-24f64,
        -6.1455180116049879894e-23f64,
        2.9017595056104745456e-21f64,
        1.3639417919073099464e-19f64,
        2.3826220476859635824e-18f64,
        -9.0642907957550702534e-18f64,
        -1.4943667065169001769e-15f64,
        -3.3919078305362211264e-14f64,
        -1.7023776642512729175e-13f64,
        9.1609750938768647911e-12f64,
        2.4230957900482704055e-10f64,
        1.7451364971382984243e-9f64,
        -3.3126119768180852711e-8f64,
        -8.6592079961391259661e-7f64,
        -4.9717367041957398581e-6f64,
        7.6309597585908126618e-5f64,
        0.0012719271366545622927f64,
        0.0017063050710955562222f64,
        -0.07685284084478667369f64,
        -0.28387654227602353814f64,
        0.92187029365045265648f64,
    ];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut na: libc::c_int = 0;
    let mut alfa: libc::c_double = 0.;
    let mut div: libc::c_double = 0.;
    let mut ddiv: libc::c_double = 0.;
    let mut even: libc::c_double = 0.;
    let mut gamma: libc::c_double = 0.;
    let mut term: libc::c_double = 0.;
    let mut cosmu: libc::c_double = 0.;
    let mut sinmu: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut f: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut q0: libc::c_double = 0.;
    let mut pa: libc::c_double = 0.;
    let mut pa1: libc::c_double = 0.;
    let mut qa: libc::c_double = 0.;
    let mut qa1: libc::c_double = 0.;
    let mut en: libc::c_double = 0.;
    let mut en1: libc::c_double = 0.;
    let mut nu: libc::c_double = 0.;
    let mut ex: libc::c_double = 0.;
    let mut ya: libc::c_double = 0.;
    let mut ya1: libc::c_double = 0.;
    let mut twobyx: libc::c_double = 0.;
    let mut den: libc::c_double = 0.;
    let mut odd: libc::c_double = 0.;
    let mut aye: libc::c_double = 0.;
    let mut dmu: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut xna: libc::c_double = 0.;
    ya1 = 0 as libc::c_int as libc::c_double;
    ya = ya1;
    en1 = ya;
    ex = *x;
    nu = *alpha;
    if *nb > 0 as libc::c_int && 0.0f64 <= nu && nu < 1.0f64 {
        if ex < 2.2250738585072014e-308f64 || ex > 1e8f64 {
            *ncalc = *nb;
            if ex > 1e8f64 {
                *by.offset(0 as libc::c_int as isize) = 0.0f64;
            } else if ex < 2.2250738585072014e-308f64 {
                *by.offset(0 as libc::c_int as isize) = -1.0f64 / 0.0f64;
            }
            i = 0 as libc::c_int;
            while i < *nb {
                *by.offset(i as isize) = *by.offset(0 as libc::c_int as isize);
                i += 1;
            }
            return;
        }
        xna = trunc(nu + 0.5f64);
        na = xna as libc::c_int;
        if na == 1 as libc::c_int {
            nu -= xna;
        }
        if nu == -0.5f64 {
            p = 0.797884560802865355879892119869f64 / sqrt(ex);
            ya = p * sin(ex);
            ya1 = -p * cos(ex);
        } else if ex < 3.0f64 {
            b = ex * 0.5f64;
            d = -log(b);
            f = nu * d;
            e = pow(b, -nu);
            if fabs(nu) < 2.149e-8f64 {
                c = 0.318309886183790671537767526745f64;
            } else {
                c = nu / sinpi(nu);
            }
            if fabs(f) < 1.0f64 {
                x2 = f * f;
                en = 19.0f64;
                s = 1.0f64;
                i = 1 as libc::c_int;
                while i <= 9 as libc::c_int {
                    s = s * x2 / en / (en - 1.0f64) + 1.0f64;
                    en -= 2.0f64;
                    i += 1;
                }
            } else {
                s = (e - 1.0f64 / e) * 0.5f64 / f;
            }
            x2 = nu * nu * 8.0f64;
            aye = ch[0 as libc::c_int as usize];
            even = 0.0f64;
            alfa = ch[1 as libc::c_int as usize];
            odd = 0.0f64;
            i = 3 as libc::c_int;
            while i <= 19 as libc::c_int {
                even = -(aye + aye + even);
                aye = -even * x2 - aye + ch[(i - 1 as libc::c_int) as usize];
                odd = -(alfa + alfa + odd);
                alfa = -odd * x2 - alfa + ch[i as usize];
                i += 2 as libc::c_int;
            }
            even = (even * 0.5f64 + aye) * x2 - aye + ch[20 as libc::c_int as usize];
            odd = (odd + alfa) * 2.0f64;
            gamma = odd * nu + even;
            g = e * gamma;
            e = (e + 1.0f64 / e) * 0.5f64;
            f = 2.0f64 * c * (odd * e + even * s * d);
            e = nu * nu;
            p = g * c;
            q = 0.318309886183790671537767526745f64 / g;
            c = nu * 1.570796326794896619231321691640f64;
            if fabs(c) < 2.149e-8f64 {
                r = 1.0f64;
            } else {
                r = sinpi(nu / 2 as libc::c_int as libc::c_double) / c;
            }
            r = 3.141592653589793238462643383280f64 * c * r * r;
            c = 1.0f64;
            d = -b * b;
            h = 0.0f64;
            ya = f + r * q;
            ya1 = p;
            en = 1.0f64;
            while fabs(g / (1.0f64 + fabs(ya))) + fabs(h / (1.0f64 + fabs(ya1)))
                > 2.2204460492503131e-16f64
            {
                f = (f * en + p + q) / (en * en - e);
                c *= d / en;
                p /= en - nu;
                q /= en + nu;
                g = c * (f + r * q);
                h = c * p - en * g;
                ya += g;
                ya1 += h;
                en += 1.0f64;
            }
            ya = -ya;
            ya1 = -ya1 / b;
        } else if ex < 16.0f64 {
            c = (0.5f64 - nu) * (0.5f64 + nu);
            b = ex + ex;
            e = ex * 0.318309886183790671537767526745f64 * cospi(nu) / 2.2204460492503131e-16f64;
            e *= e;
            p = 1.0f64;
            q = -ex;
            r = 1.0f64 + ex * ex;
            s = r;
            en = 2.0f64;
            while r * en * en < e {
                en1 = en + 1.0f64;
                d = (en - 1.0f64 + c / en) / s;
                p = (en + en - p * d) / en1;
                q = (-b + q * d) / en1;
                s = p * p + q * q;
                r *= s;
                en = en1;
            }
            f = p / s;
            p = f;
            g = -q / s;
            q = g;
            loop {
                en -= 1.0f64;
                if !(en > 0.0f64) {
                    break;
                }
                r = en1 * (2.0f64 - p) - 2.0f64;
                s = b + en1 * q;
                d = (en - 1.0f64 + c / en) / (r * r + s * s);
                p = d * r;
                q = d * s;
                e = f + 1.0f64;
                f = p * e - g * q;
                g = q * e + p * g;
                en1 = en;
            }
            f = 1.0f64 + f;
            d = f * f + g * g;
            pa = f / d;
            qa = -g / d;
            d = nu + 0.5f64 - p;
            q += ex;
            pa1 = (pa * q - qa * d) / ex;
            qa1 = (qa * q + pa * d) / ex;
            b = ex - 1.570796326794896619231321691640f64 * (nu + 0.5f64);
            c = cos(b);
            s = sin(b);
            d = 0.797884560802865355879892119869f64 / sqrt(ex);
            ya = d * (pa * s + qa * c);
            ya1 = d * (qa1 * s - pa1 * c);
        } else {
            na = 0 as libc::c_int;
            d1 = trunc(ex / fivpi);
            i = d1 as libc::c_int;
            dmu = ex
                - 15.0f64 * d1
                - d1 * pim5
                - (*alpha + 0.5f64) * 1.570796326794896619231321691640f64;
            if i - ((i / 2 as libc::c_int) << 1 as libc::c_int) == 0 as libc::c_int {
                cosmu = cos(dmu);
                sinmu = sin(dmu);
            } else {
                cosmu = -cos(dmu);
                sinmu = -sin(dmu);
            }
            ddiv = 8.0f64 * ex;
            dmu = *alpha;
            den = sqrt(ex);
            k = 1 as libc::c_int;
            while k <= 2 as libc::c_int {
                p = cosmu;
                cosmu = sinmu;
                sinmu = -p;
                d1 = (2.0f64 * dmu - 1.0f64) * (2.0f64 * dmu + 1.0f64);
                d2 = 0.0f64;
                div = ddiv;
                p = 0.0f64;
                q = 0.0f64;
                q0 = d1 / div;
                term = q0;
                i = 2 as libc::c_int;
                while i <= 20 as libc::c_int {
                    d2 += 8.0f64;
                    d1 -= d2;
                    div += ddiv;
                    term = -term * d1 / div;
                    p += term;
                    d2 += 8.0f64;
                    d1 -= d2;
                    div += ddiv;
                    term *= d1 / div;
                    q += term;
                    if fabs(term) <= 2.2204460492503131e-16f64 {
                        break;
                    }
                    i += 1;
                }
                p += 1.0f64;
                q += q0;
                if k == 1 as libc::c_int {
                    ya = 0.797884560802865355879892119869f64 * (p * cosmu - q * sinmu) / den;
                } else {
                    ya1 = 0.797884560802865355879892119869f64 * (p * cosmu - q * sinmu) / den;
                }
                dmu += 1.0f64;
                k += 1;
            }
        }
        if na == 1 as libc::c_int {
            h = 2.0f64 * (nu + 1.0f64) / ex;
            if h > 1.0f64 {
                if fabs(ya1) > 1.7976931348623157e+308f64 / h {
                    h = 0.0f64;
                    ya = 0.0f64;
                }
            }
            h = h * ya1 - ya;
            ya = ya1;
            ya1 = h;
        }
        *by.offset(0 as libc::c_int as isize) = ya;
        *ncalc = 1 as libc::c_int;
        if *nb > 1 as libc::c_int {
            *by.offset(1 as libc::c_int as isize) = ya1;
            if ya1 != 0.0f64 {
                aye = 1.0f64 + *alpha;
                twobyx = 2.0f64 / ex;
                *ncalc = 2 as libc::c_int;
                i = 2 as libc::c_int;
                while i < *nb {
                    if twobyx < 1.0f64 {
                        if fabs(*by.offset((i - 1 as libc::c_int) as isize)) * twobyx
                            >= 1.7976931348623157e+308f64 / aye
                        {
                            break;
                        }
                    } else if fabs(*by.offset((i - 1 as libc::c_int) as isize))
                        >= 1.7976931348623157e+308f64 / aye / twobyx
                    {
                        break;
                    }
                    *by.offset(i as isize) =
                        twobyx * aye * *by.offset((i - 1 as libc::c_int) as isize)
                            - *by.offset((i - 2 as libc::c_int) as isize);
                    aye += 1.0f64;
                    *ncalc += 1;
                    i += 1;
                }
            }
        }
        i = *ncalc;
        while i < *nb {
            *by.offset(i as isize) = -1.0f64 / 0.0f64;
            i += 1;
        }
    } else {
        *by.offset(0 as libc::c_int as isize) = 0.0f64;
        *ncalc = (if *nb <= 0 as libc::c_int {
            *nb
        } else {
            0 as libc::c_int
        }) - 1 as libc::c_int;
    };
}
