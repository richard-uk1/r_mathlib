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
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn R_pow_di(_: libc::c_double, _: libc::c_int) -> libc::c_double;
    fn Rf_i1mach(_: libc::c_int) -> libc::c_int;
    fn Rf_d1mach(_: libc::c_int) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn imin2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dpsifn(
    mut x: libc::c_double,
    mut n: libc::c_int,
    mut kode: libc::c_int,
    mut m: libc::c_int,
    mut ans: *mut libc::c_double,
    mut nz: *mut libc::c_int,
    mut ierr: *mut libc::c_int,
) {
    let mut current_block: u64;
    static mut bvalues: [libc::c_double; 22] = [
        1.00000000000000000e+00f64,
        -5.00000000000000000e-01f64,
        1.66666666666666667e-01f64,
        -3.33333333333333333e-02f64,
        2.38095238095238095e-02f64,
        -3.33333333333333333e-02f64,
        7.57575757575757576e-02f64,
        -2.53113553113553114e-01f64,
        1.16666666666666667e+00f64,
        -7.09215686274509804e+00f64,
        5.49711779448621554e+01f64,
        -5.29124242424242424e+02f64,
        6.19212318840579710e+03f64,
        -8.65802531135531136e+04f64,
        1.42551716666666667e+06f64,
        -2.72982310678160920e+07f64,
        6.01580873900642368e+08f64,
        -1.51163157670921569e+10f64,
        4.29614643061166667e+11f64,
        -1.37116552050883328e+13f64,
        4.88332318973593167e+14f64,
        -1.92965793419400681e+16f64,
    ];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mm: libc::c_int = 0;
    let mut mx: libc::c_int = 0;
    let mut nn: libc::c_int = 0;
    let mut np: libc::c_int = 0;
    let mut nx: libc::c_int = 0;
    let mut fn_0: libc::c_int = 0;
    let mut arg: libc::c_double = 0.;
    let mut den: libc::c_double = 0.;
    let mut elim: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    let mut fln: libc::c_double = 0.;
    let mut fx: libc::c_double = 0.;
    let mut rln: libc::c_double = 0.;
    let mut rxsq: libc::c_double = 0.;
    let mut r1m4: libc::c_double = 0.;
    let mut r1m5: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut slope: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut ta: libc::c_double = 0.;
    let mut tk: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    let mut tols: libc::c_double = 0.;
    let mut tss: libc::c_double = 0.;
    let mut tst: libc::c_double = 0.;
    let mut tt: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut t2: libc::c_double = 0.;
    let mut wdtol: libc::c_double = 0.;
    let mut xdmln: libc::c_double = 0.;
    let mut xdmy: libc::c_double = 0.;
    let mut xinc: libc::c_double = 0.;
    let mut xln: libc::c_double = 0.0f64;
    let mut xm: libc::c_double = 0.;
    let mut xmin: libc::c_double = 0.;
    let mut xq: libc::c_double = 0.;
    let mut yint: libc::c_double = 0.;
    let mut trm: [libc::c_double; 23] = [0.; 23];
    let mut trmr: [libc::c_double; 101] = [0.; 101];
    *ierr = 0 as libc::c_int;
    if n < 0 as libc::c_int
        || kode < 1 as libc::c_int
        || kode > 2 as libc::c_int
        || m < 1 as libc::c_int
    {
        *ierr = 1 as libc::c_int;
        return;
    }
    if x <= 0.0f64 {
        if x == round(x) {
            j = 0 as libc::c_int;
            while j < m {
                *ans.offset(j as isize) = if (j + n) % 2 as libc::c_int != 0 {
                    1.0f64 / 0.0f64
                } else {
                    0.0f64 / 0.0f64
                };
                j += 1;
            }
            return;
        }
        dpsifn(1.0f64 - x, n, 1 as libc::c_int, m, ans, nz, ierr);
        if m > 1 as libc::c_int || n > 3 as libc::c_int {
            *ierr = 4 as libc::c_int;
            return;
        }
        x *= 3.141592653589793238462643383280f64;
        if n == 0 as libc::c_int {
            tt = cos(x) / sin(x);
        } else if n == 1 as libc::c_int {
            tt = -(1 as libc::c_int) as libc::c_double / R_pow_di(sin(x), 2 as libc::c_int);
        } else if n == 2 as libc::c_int {
            tt = 2 as libc::c_int as libc::c_double * cos(x) / R_pow_di(sin(x), 3 as libc::c_int);
        } else if n == 3 as libc::c_int {
            tt = -(2 as libc::c_int) as libc::c_double
                * (2 as libc::c_int as libc::c_double * R_pow_di(cos(x), 2 as libc::c_int)
                    + 1.0f64)
                / R_pow_di(sin(x), 4 as libc::c_int);
        } else {
            tt = 0.0f64 / 0.0f64;
        }
        s = if n % 2 as libc::c_int != 0 {
            -1.0f64
        } else {
            1.0f64
        };
        s = 1.0f64;
        t2 = s;
        t1 = t2;
        k = 0 as libc::c_int;
        j = k - n;
        while j < m {
            t1 *= 3.141592653589793238462643383280f64;
            if k >= 2 as libc::c_int {
                t2 *= k as libc::c_double;
            }
            if j >= 0 as libc::c_int {
                *ans.offset(j as isize) = s * (*ans.offset(j as isize) + t1 / t2 * tt);
            }
            k += 1;
            j += 1;
            s = -s;
        }
        if n == 0 as libc::c_int && kode == 2 as libc::c_int {
            *ans.offset(0 as libc::c_int as isize) += xln;
        }
        return;
    }
    *nz = 0 as libc::c_int;
    xln = log(x);
    if kode == 1 as libc::c_int && m == 1 as libc::c_int {
        let mut lrg: libc::c_double =
            1 as libc::c_int as libc::c_double / (2.0f64 * 2.2204460492503131e-16f64);
        if n == 0 as libc::c_int && x * xln > lrg {
            *ans.offset(0 as libc::c_int as isize) = -xln;
            return;
        } else {
            if n >= 1 as libc::c_int && x > n as libc::c_double * lrg {
                *ans.offset(0 as libc::c_int as isize) =
                    exp(-n as libc::c_double * xln) / n as libc::c_double;
                return;
            }
        }
    }
    mm = m;
    nx = imin2(-Rf_i1mach(15 as libc::c_int), Rf_i1mach(16 as libc::c_int));
    r1m5 = Rf_d1mach(5 as libc::c_int);
    r1m4 = Rf_d1mach(4 as libc::c_int) * 0.5f64;
    wdtol = fmax2(r1m4, 0.5e-18f64);
    elim = 2.302f64 * (nx as libc::c_double * r1m5 - 3.0f64);
    loop {
        nn = n + mm - 1 as libc::c_int;
        fn_0 = nn;
        t = (fn_0 + 1 as libc::c_int) as libc::c_double * xln;
        if fabs(t) > elim {
            if t <= 0.0f64 {
                *nz = 0 as libc::c_int;
                *ierr = 2 as libc::c_int;
                return;
            }
        } else {
            if x < wdtol {
                *ans.offset(0 as libc::c_int as isize) = R_pow_di(x, -n - 1 as libc::c_int);
                if mm != 1 as libc::c_int {
                    k = 1 as libc::c_int;
                    while k < mm {
                        *ans.offset(k as isize) = *ans.offset((k - 1 as libc::c_int) as isize) / x;
                        k += 1;
                    }
                }
                if n == 0 as libc::c_int && kode == 2 as libc::c_int {
                    *ans.offset(0 as libc::c_int as isize) += xln;
                }
                return;
            }
            rln = r1m5 * Rf_i1mach(14 as libc::c_int) as libc::c_double;
            rln = fmin2(rln, 18.06f64);
            fln = fmax2(rln, 3.0f64) - 3.0f64;
            yint = 3.50f64 + 0.40f64 * fln;
            slope = 0.21f64 + fln * (0.0006038f64 * fln + 0.008677f64);
            xm = yint + slope * fn_0 as libc::c_double;
            mx = xm as libc::c_int + 1 as libc::c_int;
            xmin = mx as libc::c_double;
            if n != 0 as libc::c_int {
                xm = -2.302f64 * rln - fmin2(0.0f64, xln);
                arg = xm / n as libc::c_double;
                arg = fmin2(0.0f64, arg);
                eps = exp(arg);
                xm = 1.0f64 - eps;
                if fabs(arg) < 1.0e-3f64 {
                    xm = -arg;
                }
                fln = x * xm / eps;
                xm = xmin - x;
                if xm > 7.0f64 && fln < 15.0f64 {
                    current_block = 1852451392920375136;
                    break;
                }
            }
            xdmy = x;
            xdmln = xln;
            xinc = 0.0f64;
            if x < xmin {
                nx = x as libc::c_int;
                xinc = xmin - nx as libc::c_double;
                xdmy = x + xinc;
                xdmln = log(xdmy);
            }
            t = fn_0 as libc::c_double * xdmln;
            t1 = xdmln + xdmln;
            t2 = t + xdmln;
            tk = fmax2(fabs(t), fmax2(fabs(t1), fabs(t2)));
            if tk <= elim {
                current_block = 1125060206617174496;
                break;
            }
        }
        nz = nz.offset(1);
        mm -= 1;
        *ans.offset(mm as isize) = 0.0f64;
        if mm == 0 as libc::c_int {
            return;
        }
    }
    match current_block {
        1852451392920375136 => {
            nn = fln as libc::c_int + 1 as libc::c_int;
            np = n + 1 as libc::c_int;
            t1 = (n + 1 as libc::c_int) as libc::c_double * xln;
            t = exp(-t1);
            s = t;
            den = x;
            i = 1 as libc::c_int;
            while i <= nn {
                den += 1.0f64;
                trm[i as usize] = pow(den, -np as libc::c_double);
                s += trm[i as usize];
                i += 1;
            }
            *ans.offset(0 as libc::c_int as isize) = s;
            if n == 0 as libc::c_int && kode == 2 as libc::c_int {
                *ans.offset(0 as libc::c_int as isize) = s + xln;
            }
            if mm != 1 as libc::c_int {
                tol = wdtol / 5.0f64;
                j = 1 as libc::c_int;
                while j < mm {
                    t /= x;
                    s = t;
                    tols = t * tol;
                    den = x;
                    i = 1 as libc::c_int;
                    while i <= nn {
                        den += 1.0f64;
                        trm[i as usize] /= den;
                        s += trm[i as usize];
                        if trm[i as usize] < tols {
                            break;
                        }
                        i += 1;
                    }
                    *ans.offset(j as isize) = s;
                    j += 1;
                }
            }
            return;
        }
        _ => {
            tss = exp(-t);
            tt = 0.5f64 / xdmy;
            t1 = tt;
            tst = wdtol * tt;
            if nn != 0 as libc::c_int {
                t1 = tt + 1.0f64 / fn_0 as libc::c_double;
            }
            rxsq = 1.0f64 / (xdmy * xdmy);
            ta = 0.5f64 * rxsq;
            t = (fn_0 + 1 as libc::c_int) as libc::c_double * ta;
            s = t * bvalues[2 as libc::c_int as usize];
            if fabs(s) >= tst {
                tk = 2.0f64;
                k = 4 as libc::c_int;
                while k <= 22 as libc::c_int {
                    t = t
                        * ((tk + fn_0 as libc::c_double + 1 as libc::c_int as libc::c_double)
                            / (tk + 1.0f64))
                        * ((tk + fn_0 as libc::c_double) / (tk + 2.0f64))
                        * rxsq;
                    trm[k as usize] = t * bvalues[(k - 1 as libc::c_int) as usize];
                    if fabs(trm[k as usize]) < tst {
                        break;
                    }
                    s += trm[k as usize];
                    tk += 2.0f64;
                    k += 1;
                }
            }
            s = (s + t1) * tss;
            if xinc != 0.0f64 {
                nx = xinc as libc::c_int;
                np = nn + 1 as libc::c_int;
                if nx > 100 as libc::c_int {
                    *nz = 0 as libc::c_int;
                    *ierr = 3 as libc::c_int;
                    return;
                } else if nn == 0 as libc::c_int {
                    current_block = 4894668407003779454;
                } else {
                    xm = xinc - 1.0f64;
                    fx = x + xm;
                    i = 1 as libc::c_int;
                    while i <= nx {
                        trmr[i as usize] = pow(fx, -np as libc::c_double);
                        s += trmr[i as usize];
                        xm -= 1.0f64;
                        fx = x + xm;
                        i += 1;
                    }
                    current_block = 1915186496383530739;
                }
            } else {
                current_block = 1915186496383530739;
            }
            match current_block {
                1915186496383530739 => {
                    *ans.offset((mm - 1 as libc::c_int) as isize) = s;
                    if fn_0 == 0 as libc::c_int {
                        current_block = 612290612568952223;
                    } else {
                        j = 2 as libc::c_int;
                        loop {
                            if !(j <= mm) {
                                current_block = 9702083122263515018;
                                break;
                            }
                            fn_0 -= 1;
                            tss *= xdmy;
                            t1 = tt;
                            if fn_0 != 0 as libc::c_int {
                                t1 = tt + 1.0f64 / fn_0 as libc::c_double;
                            }
                            t = (fn_0 + 1 as libc::c_int) as libc::c_double * ta;
                            s = t * bvalues[2 as libc::c_int as usize];
                            if fabs(s) >= tst {
                                tk = (4 as libc::c_int + fn_0) as libc::c_double;
                                k = 4 as libc::c_int;
                                while k <= 22 as libc::c_int {
                                    trm[k as usize] = trm[k as usize]
                                        * (fn_0 + 1 as libc::c_int) as libc::c_double
                                        / tk;
                                    if fabs(trm[k as usize]) < tst {
                                        break;
                                    }
                                    s += trm[k as usize];
                                    tk += 2.0f64;
                                    k += 1;
                                }
                            }
                            s = (s + t1) * tss;
                            if xinc != 0.0f64 {
                                if fn_0 == 0 as libc::c_int {
                                    current_block = 4894668407003779454;
                                    break;
                                }
                                xm = xinc - 1.0f64;
                                fx = x + xm;
                                i = 1 as libc::c_int;
                                while i <= nx {
                                    trmr[i as usize] = trmr[i as usize] * fx;
                                    s += trmr[i as usize];
                                    xm -= 1.0f64;
                                    fx = x + xm;
                                    i += 1;
                                }
                            }
                            *ans.offset((mm - j) as isize) = s;
                            if fn_0 == 0 as libc::c_int {
                                current_block = 612290612568952223;
                                break;
                            }
                            j += 1;
                        }
                        match current_block {
                            612290612568952223 => {}
                            4894668407003779454 => {}
                            _ => return,
                        }
                    }
                }
                _ => {}
            }
            match current_block {
                4894668407003779454 => {
                    i = 1 as libc::c_int;
                    while i <= nx {
                        s += 1.0f64 / (x + (nx - i) as libc::c_double);
                        i += 1;
                    }
                }
                _ => {}
            }
            if kode != 2 as libc::c_int {
                *ans.offset(0 as libc::c_int as isize) = s - xdmln;
            } else if xdmy != x {
                xq = xdmy / x;
                *ans.offset(0 as libc::c_int as isize) = s - log(xq);
            }
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn psigamma(
    mut x: libc::c_double,
    mut deriv: libc::c_double,
) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut nz: libc::c_int = 0;
    let mut ierr: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    deriv = round(deriv);
    n = deriv as libc::c_int;
    if n > 100 as libc::c_int {
        printf(
            b"deriv = %d > %d (= n_max)\n\0" as *const u8 as *const libc::c_char,
            n,
            100 as libc::c_int,
        );
        return 0.0f64 / 0.0f64;
    }
    dpsifn(
        x,
        n,
        1 as libc::c_int,
        1 as libc::c_int,
        &mut ans,
        &mut nz,
        &mut ierr,
    );
    if ierr != 0 as libc::c_int {
        *__errno_location() = 33 as libc::c_int;
        return 0.0f64 / 0.0f64;
    }
    ans = -ans;
    k = 1 as libc::c_int;
    while k <= n {
        ans *= -k as libc::c_double;
        k += 1;
    }
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn digamma(mut x: libc::c_double) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut nz: libc::c_int = 0;
    let mut ierr: libc::c_int = 0;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    dpsifn(
        x,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        &mut ans,
        &mut nz,
        &mut ierr,
    );
    if ierr != 0 as libc::c_int {
        *__errno_location() = 33 as libc::c_int;
        return 0.0f64 / 0.0f64;
    }
    return -ans;
}
#[no_mangle]
pub unsafe extern "C" fn trigamma(mut x: libc::c_double) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut nz: libc::c_int = 0;
    let mut ierr: libc::c_int = 0;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    dpsifn(
        x,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        &mut ans,
        &mut nz,
        &mut ierr,
    );
    if ierr != 0 as libc::c_int {
        *__errno_location() = 33 as libc::c_int;
        return 0.0f64 / 0.0f64;
    }
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn tetragamma(mut x: libc::c_double) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut nz: libc::c_int = 0;
    let mut ierr: libc::c_int = 0;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    dpsifn(
        x,
        2 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        &mut ans,
        &mut nz,
        &mut ierr,
    );
    if ierr != 0 as libc::c_int {
        *__errno_location() = 33 as libc::c_int;
        return 0.0f64 / 0.0f64;
    }
    return -2.0f64 * ans;
}
#[no_mangle]
pub unsafe extern "C" fn pentagamma(mut x: libc::c_double) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut nz: libc::c_int = 0;
    let mut ierr: libc::c_int = 0;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    dpsifn(
        x,
        3 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        &mut ans,
        &mut nz,
        &mut ierr,
    );
    if ierr != 0 as libc::c_int {
        *__errno_location() = 33 as libc::c_int;
        return 0.0f64 / 0.0f64;
    }
    return 6.0f64 * ans;
}
