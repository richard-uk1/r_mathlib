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
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_k(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut expo: libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut ize: libc::c_int = 0;
    let mut bk: *mut libc::c_double = 0 as *mut libc::c_double;
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
            printf(msg, b"bessel_k\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    ize = expo as libc::c_int;
    if alpha < 0 as libc::c_int as libc::c_double {
        alpha = -alpha;
    }
    nb = 1 as libc::c_int + floor(alpha) as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    bk = calloc(
        nb as libc::c_ulong,
        ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as *mut libc::c_double;
    if bk.is_null() {
        printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            b"bessel_k allocation error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    K_bessel(&mut x, &mut alpha, &mut nb, &mut ize, bk, &mut ncalc);
    if ncalc != nb {
        if ncalc < 0 as libc::c_int {
            printf(
                b"bessel_k(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                    as *const u8 as *const libc::c_char,
                x,
                ncalc,
                nb,
                alpha,
            );
        } else {
            printf(
                b"bessel_k(%g,nu=%g): precision lost in result\n\0" as *const u8
                    as *const libc::c_char,
                x,
                alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
            );
        }
    }
    x = *bk.offset((nb - 1 as libc::c_int) as isize);
    free(bk as *mut libc::c_void);
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn bessel_k_ex(
    mut x: libc::c_double,
    mut alpha: libc::c_double,
    mut expo: libc::c_double,
    mut bk: *mut libc::c_double,
) -> libc::c_double {
    let mut nb: libc::c_int = 0;
    let mut ncalc: libc::c_int = 0;
    let mut ize: libc::c_int = 0;
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
            printf(msg, b"bessel_k\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    ize = expo as libc::c_int;
    if alpha < 0 as libc::c_int as libc::c_double {
        alpha = -alpha;
    }
    nb = 1 as libc::c_int + floor(alpha) as libc::c_int;
    alpha -= (nb - 1 as libc::c_int) as libc::c_double;
    K_bessel(&mut x, &mut alpha, &mut nb, &mut ize, bk, &mut ncalc);
    if ncalc != nb {
        if ncalc < 0 as libc::c_int {
            printf(
                b"bessel_k(%g): ncalc (=%d) != nb (=%d); alpha=%g. Arg. out of range?\n\0"
                    as *const u8 as *const libc::c_char,
                x,
                ncalc,
                nb,
                alpha,
            );
        } else {
            printf(
                b"bessel_k(%g,nu=%g): precision lost in result\n\0" as *const u8
                    as *const libc::c_char,
                x,
                alpha + nb as libc::c_double - 1 as libc::c_int as libc::c_double,
            );
        }
    }
    x = *bk.offset((nb - 1 as libc::c_int) as isize);
    return x;
}
unsafe extern "C" fn K_bessel(
    mut x: *mut libc::c_double,
    mut alpha: *mut libc::c_double,
    mut nb: *mut libc::c_int,
    mut ize: *mut libc::c_int,
    mut bk: *mut libc::c_double,
    mut ncalc: *mut libc::c_int,
) {
    static mut a: libc::c_double = 0.11593151565841244881f64;
    static mut p: [libc::c_double; 8] = [
        0.805629875690432845f64,
        20.4045500205365151f64,
        157.705605106676174f64,
        536.671116469207504f64,
        900.382759291288778f64,
        730.923886650660393f64,
        229.299301509425145f64,
        0.822467033424113231f64,
    ];
    static mut q: [libc::c_double; 7] = [
        29.4601986247850434f64,
        277.577868510221208f64,
        1206.70325591027438f64,
        2762.91444159791519f64,
        3443.74050506564618f64,
        2210.63190113378647f64,
        572.267338359892221f64,
    ];
    static mut r: [libc::c_double; 5] = [
        -0.48672575865218401848f64,
        13.079485869097804016f64,
        -101.96490580880537526f64,
        347.65409106507813131f64,
        3.495898124521934782e-4f64,
    ];
    static mut s: [libc::c_double; 4] = [
        -25.579105509976461286f64,
        212.57260432226544008f64,
        -610.69018684944109624f64,
        422.69668805777760407f64,
    ];
    static mut t: [libc::c_double; 6] = [
        1.6125990452916363814e-10f64,
        2.5051878502858255354e-8f64,
        2.7557319615147964774e-6f64,
        1.9841269840928373686e-4f64,
        0.0083333333333334751799f64,
        0.16666666666666666446f64,
    ];
    static mut estm: [libc::c_double; 6] = [
        52.0583f64,
        5.7607f64,
        2.7782f64,
        14.4303f64,
        185.3004f64,
        9.3715f64,
    ];
    static mut estf: [libc::c_double; 7] = [
        41.8341f64, 7.1075f64, 6.4306f64, 42.511f64, 1.35633f64, 84.5096f64, 20.0f64,
    ];
    let mut iend: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut mplus1: libc::c_int = 0;
    let mut x2by4: libc::c_double = 0.;
    let mut twox: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut blpha: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    let mut wminf: libc::c_double = 0.;
    let mut d1: libc::c_double = 0.;
    let mut d2: libc::c_double = 0.;
    let mut d3: libc::c_double = 0.;
    let mut f0: libc::c_double = 0.;
    let mut f1: libc::c_double = 0.;
    let mut f2: libc::c_double = 0.;
    let mut p0: libc::c_double = 0.;
    let mut q0: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut twonu: libc::c_double = 0.;
    let mut dm: libc::c_double = 0.;
    let mut ex: libc::c_double = 0.;
    let mut bk1: libc::c_double = 0.;
    let mut bk2: libc::c_double = 0.;
    let mut nu: libc::c_double = 0.;
    ii = 0 as libc::c_int;
    ex = *x;
    nu = *alpha;
    *ncalc = (if *nb <= 0 as libc::c_int {
        *nb
    } else {
        0 as libc::c_int
    }) - 2 as libc::c_int;
    if *nb > 0 as libc::c_int
        && (0.0f64 <= nu && nu < 1.0f64)
        && (1 as libc::c_int <= *ize && *ize <= 2 as libc::c_int)
    {
        let mut current_block_262: u64;
        if ex <= 0 as libc::c_int as libc::c_double || *ize == 1 as libc::c_int && ex > 705.342f64 {
            if ex <= 0 as libc::c_int as libc::c_double {
                if ex < 0 as libc::c_int as libc::c_double {
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
                                msg = b"value out of range in '%s'\n\0" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            4 => {
                                msg = b"convergence failed in '%s'\n\0" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            8 => {
                                msg = b"full precision may not have been achieved in '%s'\n\0"
                                    as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            16 => {
                                msg = b"underflow occurred in '%s'\n\0" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char;
                            }
                            _ => {}
                        }
                        printf(msg, b"K_bessel\0" as *const u8 as *const libc::c_char);
                    }
                }
                i = 0 as libc::c_int;
                while i < *nb {
                    *bk.offset(i as isize) = 1.0f64 / 0.0f64;
                    i += 1;
                }
            } else {
                i = 0 as libc::c_int;
                while i < *nb {
                    *bk.offset(i as isize) = 0.0f64;
                    i += 1;
                }
            }
            *ncalc = *nb;
            return;
        }
        k = 0 as libc::c_int;
        if nu < 1.49e-154f64 {
            nu = 0.0f64;
        } else if nu > 0.5f64 {
            k = 1 as libc::c_int;
            nu -= 1.0f64;
        }
        twonu = nu + nu;
        iend = *nb + k - 1 as libc::c_int;
        c = nu * nu;
        d3 = -c;
        if ex <= 1.0f64 {
            d1 = 0.0f64;
            d2 = p[0 as libc::c_int as usize];
            t1 = 1.0f64;
            t2 = q[0 as libc::c_int as usize];
            i = 2 as libc::c_int;
            while i <= 7 as libc::c_int {
                d1 = c * d1 + p[(i - 1 as libc::c_int) as usize];
                d2 = c * d2 + p[i as usize];
                t1 = c * t1 + q[(i - 1 as libc::c_int) as usize];
                t2 = c * t2 + q[i as usize];
                i += 2 as libc::c_int;
            }
            d1 = nu * d1;
            t1 = nu * t1;
            f1 = log(ex);
            f0 = a + nu * (p[7 as libc::c_int as usize] - nu * (d1 + d2) / (t1 + t2)) - f1;
            q0 =
                exp(-nu
                    * (a - nu * (p[7 as libc::c_int as usize] + nu * (d1 - d2) / (t1 - t2)) - f1));
            f1 = nu * f0;
            p0 = exp(f1);
            d1 = r[4 as libc::c_int as usize];
            t1 = 1.0f64;
            i = 0 as libc::c_int;
            while i < 4 as libc::c_int {
                d1 = c * d1 + r[i as usize];
                t1 = c * t1 + s[i as usize];
                i += 1;
            }
            if fabs(f1) <= 0.5f64 {
                f1 *= f1;
                d2 = 0.0f64;
                i = 0 as libc::c_int;
                while i < 6 as libc::c_int {
                    d2 = f1 * d2 + t[i as usize];
                    i += 1;
                }
                d2 = f0 + f0 * f1 * d2;
            } else {
                d2 = sinh(f1) / nu;
            }
            f0 = d2 - nu * d1 / (t1 * p0);
            if ex <= 1e-10f64 {
                *bk.offset(0 as libc::c_int as isize) = f0 + ex * f0;
                if *ize == 1 as libc::c_int {
                    *bk.offset(0 as libc::c_int as isize) -=
                        ex * *bk.offset(0 as libc::c_int as isize);
                }
                ratio = p0 / f0;
                c = ex * 1.7976931348623157e+308f64;
                if k != 0 as libc::c_int {
                    *ncalc = -(1 as libc::c_int);
                    if *bk.offset(0 as libc::c_int as isize) >= c / ratio {
                        return;
                    }
                    *bk.offset(0 as libc::c_int as isize) =
                        ratio * *bk.offset(0 as libc::c_int as isize) / ex;
                    twonu += 2.0f64;
                    ratio = twonu;
                }
                *ncalc = 1 as libc::c_int;
                if *nb == 1 as libc::c_int {
                    return;
                }
                *ncalc = -(1 as libc::c_int);
                i = 1 as libc::c_int;
                while i < *nb {
                    if ratio >= c {
                        return;
                    }
                    *bk.offset(i as isize) = ratio / ex;
                    twonu += 2.0f64;
                    ratio = twonu;
                    i += 1;
                }
                *ncalc = 1 as libc::c_int;
                current_block_262 = 4022300672336782409;
            } else {
                c = 1.0f64;
                x2by4 = ex * ex / 4.0f64;
                p0 = 0.5f64 * p0;
                q0 = 0.5f64 * q0;
                d1 = -1.0f64;
                d2 = 0.0f64;
                bk1 = 0.0f64;
                bk2 = 0.0f64;
                f1 = f0;
                f2 = p0;
                loop {
                    d1 += 2.0f64;
                    d2 += 1.0f64;
                    d3 = d1 + d3;
                    c = x2by4 * c / d2;
                    f0 = (d2 * f0 + p0 + q0) / d3;
                    p0 /= d2 - nu;
                    q0 /= d2 + nu;
                    t1 = c * f0;
                    t2 = c * (p0 - d2 * f0);
                    bk1 += t1;
                    bk2 += t2;
                    if !(fabs(t1 / (f1 + bk1)) > 2.2204460492503131e-16f64
                        || fabs(t2 / (f2 + bk2)) > 2.2204460492503131e-16f64)
                    {
                        break;
                    }
                }
                bk1 = f1 + bk1;
                bk2 = 2.0f64 * (f2 + bk2) / ex;
                if *ize == 2 as libc::c_int {
                    d1 = exp(ex);
                    bk1 *= d1;
                    bk2 *= d1;
                }
                wminf = estf[0 as libc::c_int as usize] * ex + estf[1 as libc::c_int as usize];
                current_block_262 = 17100064147490331435;
            }
        } else {
            if 2.2204460492503131e-16f64 * ex > 1.0f64 {
                *ncalc = *nb;
                bk1 = 1.0f64 / (0.797884560802865355879892119869f64 * sqrt(ex));
                i = 0 as libc::c_int;
                while i < *nb {
                    *bk.offset(i as isize) = bk1;
                    i += 1;
                }
                return;
            } else {
                twox = ex + ex;
                blpha = 0.0f64;
                ratio = 0.0f64;
                if ex <= 4.0f64 {
                    d2 = trunc(
                        estm[0 as libc::c_int as usize] / ex + estm[1 as libc::c_int as usize],
                    );
                    m = d2 as libc::c_int;
                    d1 = d2 + d2;
                    d2 -= 0.5f64;
                    d2 *= d2;
                    i = 2 as libc::c_int;
                    while i <= m {
                        d1 -= 2.0f64;
                        d2 -= d1;
                        ratio = (d3 + d2) / (twox + d1 - ratio);
                        i += 1;
                    }
                    d2 = trunc(
                        estm[2 as libc::c_int as usize] * ex + estm[3 as libc::c_int as usize],
                    );
                    m = d2 as libc::c_int;
                    c = fabs(nu);
                    d3 = c + c;
                    d1 = d3 - 1.0f64;
                    f1 = 2.2250738585072014e-308f64;
                    f0 = (2.0f64 * (c + d2) / ex + 0.5f64 * ex / (c + d2 + 1.0f64))
                        * 2.2250738585072014e-308f64;
                    i = 3 as libc::c_int;
                    while i <= m {
                        d2 -= 1.0f64;
                        f2 = (d3 + d2 + d2) * f0;
                        blpha = (1.0f64 + d1 / d2) * (f2 + blpha);
                        f2 = f2 / ex + f1;
                        f1 = f0;
                        f0 = f2;
                        i += 1;
                    }
                    f1 = (d3 + 2.0f64) * f0 / ex + f1;
                    d1 = 0.0f64;
                    t1 = 1.0f64;
                    i = 1 as libc::c_int;
                    while i <= 7 as libc::c_int {
                        d1 = c * d1 + p[(i - 1 as libc::c_int) as usize];
                        t1 = c * t1 + q[(i - 1 as libc::c_int) as usize];
                        i += 1;
                    }
                    p0 = exp(c * (a + c * (p[7 as libc::c_int as usize] - c * d1 / t1) - log(ex)))
                        / ex;
                    f2 = (c + 0.5f64 - ratio) * f1 / ex;
                    bk1 = p0 + (d3 * f0 - f2 + f0 + blpha) / (f2 + f1 + f0) * p0;
                    if *ize == 1 as libc::c_int {
                        bk1 *= exp(-ex);
                    }
                    wminf = estf[2 as libc::c_int as usize] * ex + estf[3 as libc::c_int as usize];
                } else {
                    dm = trunc(
                        estm[4 as libc::c_int as usize] / ex + estm[5 as libc::c_int as usize],
                    );
                    m = dm as libc::c_int;
                    d2 = dm - 0.5f64;
                    d2 *= d2;
                    d1 = dm + dm;
                    i = 2 as libc::c_int;
                    while i <= m {
                        dm -= 1.0f64;
                        d1 -= 2.0f64;
                        d2 -= d1;
                        ratio = (d3 + d2) / (twox + d1 - ratio);
                        blpha = (ratio + ratio * blpha) / dm;
                        i += 1;
                    }
                    bk1 = 1.0f64
                        / ((0.797884560802865355879892119869f64
                            + 0.797884560802865355879892119869f64 * blpha)
                            * sqrt(ex));
                    if *ize == 1 as libc::c_int {
                        bk1 *= exp(-ex);
                    }
                    wminf = estf[4 as libc::c_int as usize]
                        * (ex - fabs(ex - estf[6 as libc::c_int as usize]))
                        + estf[5 as libc::c_int as usize];
                }
                bk2 = bk1 + bk1 * (nu + 0.5f64 - ratio) / ex;
            }
            current_block_262 = 17100064147490331435;
        }
        match current_block_262 {
            17100064147490331435 => {
                *ncalc = *nb;
                *bk.offset(0 as libc::c_int as isize) = bk1;
                if iend == 0 as libc::c_int {
                    return;
                }
                j = 1 as libc::c_int - k;
                if j >= 0 as libc::c_int {
                    *bk.offset(j as isize) = bk2;
                }
                if iend == 1 as libc::c_int {
                    return;
                }
                m = if (wminf - nu) as libc::c_int <= iend {
                    (wminf - nu) as libc::c_int
                } else {
                    iend
                };
                i = 2 as libc::c_int;
                while i <= m {
                    t1 = bk1;
                    bk1 = bk2;
                    twonu += 2.0f64;
                    if ex < 1.0f64 {
                        if bk1 >= 1.7976931348623157e+308f64 / twonu * ex {
                            break;
                        }
                    } else if bk1 / ex >= 1.7976931348623157e+308f64 / twonu {
                        break;
                    }
                    bk2 = twonu / ex * bk1 + t1;
                    ii = i;
                    j += 1;
                    if j >= 0 as libc::c_int {
                        *bk.offset(j as isize) = bk2;
                    }
                    i += 1;
                }
                m = ii;
                if m == iend {
                    return;
                }
                ratio = bk2 / bk1;
                mplus1 = m + 1 as libc::c_int;
                *ncalc = -(1 as libc::c_int);
                i = mplus1;
                while i <= iend {
                    twonu += 2.0f64;
                    ratio = twonu / ex + 1.0f64 / ratio;
                    j += 1;
                    if j >= 1 as libc::c_int {
                        *bk.offset(j as isize) = ratio;
                    } else {
                        if bk2 >= 1.7976931348623157e+308f64 / ratio {
                            return;
                        }
                        bk2 *= ratio;
                    }
                    i += 1;
                }
                *ncalc = if 1 as libc::c_int <= mplus1 - k {
                    mplus1 - k
                } else {
                    1 as libc::c_int
                };
                if *ncalc == 1 as libc::c_int {
                    *bk.offset(0 as libc::c_int as isize) = bk2;
                }
                if *nb == 1 as libc::c_int {
                    return;
                }
            }
            _ => {}
        }
        i = *ncalc;
        while i < *nb {
            *bk.offset(i as isize) *= *bk.offset((i - 1 as libc::c_int) as isize);
            *ncalc += 1;
            i += 1;
        }
    }
}
