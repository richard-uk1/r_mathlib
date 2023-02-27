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
    fn ldexp(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_gamma_cody(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn R_pow_di(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn sinpi(_: libc::c_double) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn bessel_k(_: libc::c_double, _: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn bessel_k_ex(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: *mut libc::c_double,
    ) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_i(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut expo: libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut ize: libc::c_int = 0;
    let mut na: libc::c_double = 0.;
    let mut bi: *mut libc::c_double = 0 as *mut libc::c_double;
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
            printf(msg, b"bessel_i\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    ize = expo as libc::c_int;
    na = floor(alpha);
    if alpha < 0 as libc::c_int as libc::c_double {
        return bessel_i(x, -alpha, expo)
            + (if alpha == na {
                0 as libc::c_int as libc::c_double
            } else {
                bessel_k(x, -alpha, expo)
                    * (if ize == 1 as libc::c_int {
                        2.0f64
                    } else {
                        2.0f64 * exp(-2.0f64 * x)
                    })
                    / 3.141592653589793238462643383280f64
                    * sinpi(-alpha)
            });
    }
    nb = 1 as libc::c_int + na as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    bi = calloc(
        nb as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if bi.is_null() {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            b"bessel_i allocation error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    I_bessel(&mut x, &mut alpha, &mut nb, &mut ize, bi, &mut ncalc);
    if ncalc != nb {
        if ncalc < 0 as libc::c_int {
            printf(
                b"bessel_i(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                    as *const u8 as *const libc::c_char,
                x,
                ncalc,
                nb,
                alpha,
            );
        } else {
            printf(
                b"bessel_i(%g,nu=%g): precision lost in result\n\0" as *const u8
                    as *const libc::c_char,
                x,
                alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
            );
        }
    }
    x = *bi.offset((nb - 1 as libc::c_int) as isize);
    free(bi as *mut libc::c_void);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_i_ex(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut expo: libc::c_double,
    mut bi: *mut libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut ize: libc::c_int = 0;
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
            printf(msg, b"bessel_i\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    ize = expo as libc::c_int;
    na = floor(alpha);
    if alpha < 0 as libc::c_int as libc::c_double {
        return bessel_i_ex(x, -alpha, expo, bi)
            + (if alpha == na {
                0 as libc::c_int as libc::c_double
            } else {
                bessel_k_ex(x, -alpha, expo, bi)
                    * (if ize == 1 as libc::c_int {
                        2.0f64
                    } else {
                        2.0f64 * exp(-2.0f64 * x)
                    })
                    / 3.141592653589793238462643383280f64
                    * sinpi(-alpha)
            });
    }
    nb = 1 as libc::c_int + na as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    I_bessel(&mut x, &mut alpha, &mut nb, &mut ize, bi, &mut ncalc);
    if ncalc != nb {
        if ncalc < 0 as libc::c_int {
            printf(
                b"bessel_i(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                    as *const u8 as *const libc::c_char,
                x,
                ncalc,
                nb,
                alpha,
            );
        } else {
            printf(
                b"bessel_i(%g,nu=%g): precision lost in result\n\0" as *const u8
                    as *const libc::c_char,
                x,
                alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
            );
        }
    }
    x = *bi.offset((nb - 1 as libc::c_int) as isize);
    return x;
}
unsafe extern "C" fn I_bessel(
    mut x: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
    mut nb: *mut libc::c_int,
    mut ize: *mut libc::c_int,
    mut bi: *mut libc::c_double,
    mut ncalc: *mut libc::c_int,
) {
    static mut const__: libc::c_double = 1.585f64;
    let mut nend: libc::c_int = 0;
    let mut intx: libc::c_int = 0;
    let mut nbmx: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nstart: libc::c_int = 0;
    let mut pold: libc::c_double = 0.;
    let mut test: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut em: libc::c_double = 0.;
    let mut en: libc::c_double = 0.;
    let mut empal: libc::c_double = 0.;
    let mut emp2al: libc::c_double = 0.;
    let mut halfx: libc::c_double = 0.;
    let mut aa: libc::c_double = 0.;
    let mut bb: libc::c_double = 0.;
    let mut cc: libc::c_double = 0.;
    let mut psave: libc::c_double = 0.;
    let mut plast: libc::c_double = 0.;
    let mut tover: libc::c_double = 0.;
    let mut psavel: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut nu: libc::c_double = 0.;
    let mut twonu: libc::c_double = 0.;
    bi = bi.offset(-1);
    nu = *alpha;
    twonu = nu + nu;
    if *nb > 0 as libc::c_int
        && *x >= 0.0f64
        && (0.0f64 <= nu && nu < 1.0f64)
        && (1 as libc::c_int <= *ize && *ize <= 2 as libc::c_int)
    {
        *ncalc = *nb;
        if *ize == 1 as libc::c_int && *x > 709.0f64 {
            k = 1 as libc::c_int;
            while k <= *nb {
                *bi.offset(k as isize) = 1.0f64 / 0.0f64;
                k += 1;
            }
            return;
        }
        if *ize == 2 as libc::c_int && *x > 1e5f64 {
            k = 1 as libc::c_int;
            while k <= *nb {
                *bi.offset(k as isize) = 0.0f64;
                k += 1;
            }
            return;
        }
        intx = *x as libc::c_int;
        if *x >= 1e-4f64 {
            let mut current_block_152: u64;
            nbmx = *nb - intx;
            n = intx + 1 as libc::c_int;
            en = (n + n) as libc::c_double + twonu;
            plast = 1.0f64;
            p = en / *x;
            test = 1e16f64 + 1e16f64;
            if intx << 1 as libc::c_int > 16 as libc::c_int * 5 as libc::c_int {
                test = sqrt(test * p);
            } else {
                test /= R_pow_di(const__, intx);
            }
            if nbmx >= 3 as libc::c_int {
                tover = 1e308f64 / 1e16f64;
                nstart = intx + 2 as libc::c_int;
                nend = *nb - 1 as libc::c_int;
                k = nstart;
                loop {
                    if !(k <= nend) {
                        current_block_152 = 13826291924415791078;
                        break;
                    }
                    n = k;
                    en += 2.0f64;
                    pold = plast;
                    plast = p;
                    p = en * plast / *x + pold;
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
                            p = en * plast / *x + pold;
                            if !(p <= 1.0f64) {
                                break;
                            }
                        }
                        bb = en / *x;
                        test = pold * plast / 1e16f64;
                        test *= 0.5f64 - 0.5f64 / (bb * bb);
                        p = plast * tover;
                        n -= 1;
                        en -= 2.0f64;
                        nend = if *nb <= n { *nb } else { n };
                        l = nstart;
                        loop {
                            if !(l <= nend) {
                                current_block_152 = 3689906465960840878;
                                break;
                            }
                            *ncalc = l;
                            pold = psavel;
                            psavel = psave;
                            psave = en * psavel / *x + pold;
                            if psave * psavel > test {
                                current_block_152 = 16555342469606722687;
                                break;
                            }
                            l += 1;
                        }
                        match current_block_152 {
                            3689906465960840878 => {
                                *ncalc = nend + 1 as libc::c_int;
                            }
                            _ => {}
                        }
                        *ncalc -= 1;
                        current_block_152 = 6584368754485046663;
                        break;
                    } else {
                        k += 1;
                    }
                }
                match current_block_152 {
                    6584368754485046663 => {}
                    _ => {
                        n = nend;
                        en = (n + n) as libc::c_double + twonu;
                        test = fmax2(test, sqrt(plast * 1e16f64) * sqrt(p + p));
                        current_block_152 = 12829669402821218572;
                    }
                }
            } else {
                current_block_152 = 12829669402821218572;
            }
            match current_block_152 {
                12829669402821218572 => loop {
                    n += 1;
                    en += 2.0f64;
                    pold = plast;
                    plast = p;
                    p = en * plast / *x + pold;
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
            em = n as libc::c_double - 1.0f64;
            empal = em + nu;
            emp2al = em - 1.0f64 + twonu;
            sum = aa * empal * emp2al / em;
            nend = n - *nb;
            if nend < 0 as libc::c_int {
                *bi.offset(n as isize) = aa;
                nend = -nend;
                l = 1 as libc::c_int;
                while l <= nend {
                    *bi.offset((n + l) as isize) = 0.0f64;
                    l += 1;
                }
                current_block_152 = 10301740260014665685;
            } else {
                if nend > 0 as libc::c_int {
                    l = 1 as libc::c_int;
                    while l <= nend {
                        n -= 1;
                        en -= 2.0f64;
                        cc = bb;
                        bb = aa;
                        if nend > 100 as libc::c_int && aa > 1e200f64 {
                            cc = ldexp(cc, -(900 as libc::c_int));
                            bb = ldexp(bb, -(900 as libc::c_int));
                            sum = ldexp(sum, -(900 as libc::c_int));
                        }
                        aa = en * bb / *x + cc;
                        em -= 1.0f64;
                        emp2al -= 1.0f64;
                        if n == 1 as libc::c_int {
                            break;
                        }
                        if n == 2 as libc::c_int {
                            emp2al = 1.0f64;
                        }
                        empal -= 1.0f64;
                        sum = (sum + aa * empal) * emp2al / em;
                        l += 1;
                    }
                }
                *bi.offset(n as isize) = aa;
                if *nb <= 1 as libc::c_int {
                    sum = sum + sum + aa;
                    current_block_152 = 9751606451941373277;
                } else {
                    n -= 1;
                    en -= 2.0f64;
                    *bi.offset(n as isize) = en * aa / *x + bb;
                    if n == 1 as libc::c_int {
                        current_block_152 = 15703518533755058203;
                    } else {
                        em -= 1.0f64;
                        if n == 2 as libc::c_int {
                            emp2al = 1.0f64;
                        } else {
                            emp2al -= 1.0f64;
                        }
                        empal -= 1.0f64;
                        sum = (sum + *bi.offset(n as isize) * empal) * emp2al / em;
                        current_block_152 = 10301740260014665685;
                    }
                }
            }
            match current_block_152 {
                10301740260014665685 => {
                    nend = n - 2 as libc::c_int;
                    if nend > 0 as libc::c_int {
                        l = 1 as libc::c_int;
                        while l <= nend {
                            n -= 1;
                            en -= 2.0f64;
                            *bi.offset(n as isize) =
                                en * *bi.offset((n + 1 as libc::c_int) as isize) / *x
                                    + *bi.offset((n + 2 as libc::c_int) as isize);
                            em -= 1.0f64;
                            if n == 2 as libc::c_int {
                                emp2al = 1.0f64;
                            } else {
                                emp2al -= 1.0f64;
                            }
                            empal -= 1.0f64;
                            sum = (sum + *bi.offset(n as isize) * empal) * emp2al / em;
                            l += 1;
                        }
                    }
                    *bi.offset(1 as libc::c_int as isize) =
                        2.0f64 * empal * *bi.offset(2 as libc::c_int as isize) / *x
                            + *bi.offset(3 as libc::c_int as isize);
                    current_block_152 = 15703518533755058203;
                }
                _ => {}
            }
            match current_block_152 {
                15703518533755058203 => {
                    sum = sum + sum + *bi.offset(1 as libc::c_int as isize);
                }
                _ => {}
            }
            if nu != 0.0f64 {
                sum *= Rf_gamma_cody(1.0f64 + nu) * pow(*x * 0.5f64, -nu);
            }
            if *ize == 1 as libc::c_int {
                sum *= exp(-*x);
            }
            aa = 8.9e-308f64;
            if sum > 1.0f64 {
                aa *= sum;
            }
            n = 1 as libc::c_int;
            while n <= *nb {
                if *bi.offset(n as isize) < aa {
                    *bi.offset(n as isize) = 0.0f64;
                } else {
                    *bi.offset(n as isize) /= sum;
                }
                n += 1;
            }
            return;
        } else {
            aa = 1.0f64;
            empal = 1.0f64 + nu;
            halfx = 0.5f64 * *x;
            if nu != 0.0f64 {
                aa = pow(halfx, nu) / Rf_gamma_cody(empal);
            }
            if *ize == 2 as libc::c_int {
                aa *= exp(-*x);
            }
            bb = halfx * halfx;
            *bi.offset(1 as libc::c_int as isize) = aa + aa * bb / empal;
            if *x != 0.0f64 && *bi.offset(1 as libc::c_int as isize) == 0.0f64 {
                *ncalc = 0 as libc::c_int;
            }
            if *nb > 1 as libc::c_int {
                if *x == 0.0f64 {
                    n = 2 as libc::c_int;
                    while n <= *nb {
                        *bi.offset(n as isize) = 0.0f64;
                        n += 1;
                    }
                } else {
                    cc = halfx;
                    tover = (8.9e-308f64 + 8.9e-308f64) / *x;
                    if bb != 0.0f64 {
                        tover = 8.9e-308f64 / bb;
                    }
                    n = 2 as libc::c_int;
                    while n <= *nb {
                        aa /= empal;
                        empal += 1.0f64;
                        aa *= cc;
                        if aa <= tover * empal {
                            aa = 0.0f64;
                            *bi.offset(n as isize) = aa;
                        } else {
                            *bi.offset(n as isize) = aa + aa * bb / empal;
                        }
                        if *bi.offset(n as isize) == 0.0f64 && *ncalc > n {
                            *ncalc = n - 1 as libc::c_int;
                        }
                        n += 1;
                    }
                }
            }
        }
    } else {
        *ncalc = (if *nb <= 0 as libc::c_int {
            *nb
        } else {
            0 as libc::c_int
        }) - 1 as libc::c_int;
    };
}
