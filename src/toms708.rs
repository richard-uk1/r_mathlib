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
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn logspace_add(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn Rf_i1mach(_: libc::c_int) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn Rf_d1mach(_: libc::c_int) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type Rboolean = libc::c_uint;
pub const TRUE: Rboolean = 1;
pub const FALSE: Rboolean = 0;
#[no_mangle]
pub unsafe extern "C" fn Rf_bratio(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut w: *mut libc::c_double,
    mut w1: *mut libc::c_double,
    mut ierr: *mut libc::c_int,
    mut log_p: libc::c_int,
) {
    let mut a_lt_b: Rboolean = FALSE;
    let mut current_block: u64;
    let mut do_swap: Rboolean = FALSE;
    let mut n: libc::c_int = 0;
    let mut ierr1: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    let mut b0: libc::c_double = 0.;
    let mut x0: libc::c_double = 0.;
    let mut y0: libc::c_double = 0.;
    let mut lambda: libc::c_double = 0.;
    let mut eps: libc::c_double = 2.0f64 * Rf_d1mach(3 as libc::c_int);
    *w = if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    *w1 = if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    if x.is_nan() as i32 != 0 as libc::c_int
        || y.is_nan() as i32 != 0 as libc::c_int
        || a.is_nan() as i32 != 0 as libc::c_int
        || b.is_nan() as i32 != 0 as libc::c_int
    {
        *ierr = 9 as libc::c_int;
        return;
    }
    if a < 0.0f64 || b < 0.0f64 {
        *ierr = 1 as libc::c_int;
        return;
    }
    if a == 0.0f64 && b == 0.0f64 {
        *ierr = 2 as libc::c_int;
        return;
    }
    if x < 0.0f64 || x > 1.0f64 {
        *ierr = 3 as libc::c_int;
        return;
    }
    if y < 0.0f64 || y > 1.0f64 {
        *ierr = 4 as libc::c_int;
        return;
    }
    z = x + y - 0.5f64 - 0.5f64;
    if fabs(z) > eps * 3.0f64 {
        *ierr = 5 as libc::c_int;
        return;
    }
    *ierr = 0 as libc::c_int;
    if x == 0.0f64 {
        if a == 0.0f64 {
            *ierr = 6 as libc::c_int;
            return;
        }
    } else {
        if y == 0.0f64 {
            if b == 0.0f64 {
                *ierr = 7 as libc::c_int;
                return;
            }
            current_block = 17478089311782157434;
        } else if a == 0.0f64 {
            current_block = 17478089311782157434;
        } else if b == 0.0f64 {
            current_block = 12410001467641165671;
        } else {
            eps = if eps > 1e-15f64 { eps } else { 1e-15f64 };
            a_lt_b = (a < b) as libc::c_int as Rboolean;
            if (if a_lt_b as libc::c_uint != 0 { b } else { a }) < eps * 0.001f64 {
                if log_p != 0 {
                    if a_lt_b as u64 != 0 {
                        *w = Rlog1p(-a / (a + b));
                        *w1 = log(a / (a + b));
                    } else {
                        *w = log(b / (a + b));
                        *w1 = Rlog1p(-b / (a + b));
                    }
                } else {
                    *w = b / (a + b);
                    *w1 = a / (a + b);
                }
                return;
            }
            if (if a < b { a } else { b }) <= 1.0f64 {
                do_swap = (x > 0.5f64) as libc::c_int as Rboolean;
                if do_swap as u64 != 0 {
                    a0 = b;
                    x0 = y;
                    b0 = a;
                    y0 = x;
                } else {
                    a0 = a;
                    x0 = x;
                    b0 = b;
                    y0 = y;
                }
                if b0 < (if eps < eps * a0 { eps } else { eps * a0 }) {
                    *w = fpser(a0, b0, x0, eps, log_p);
                    *w1 = if log_p != 0 {
                        if *w > -0.693147180559945309417232121458f64 {
                            log(-rexpm1(*w))
                        } else {
                            Rlog1p(-exp(*w))
                        }
                    } else {
                        0.5f64 - *w + 0.5f64
                    };
                    current_block = 7496337703584048604;
                } else {
                    if a0 < (if eps < eps * b0 { eps } else { eps * b0 }) && b0 * x0 <= 1.0f64 {
                        *w1 = apser(a0, b0, x0, eps);
                        current_block = 9167786483348357766;
                    } else {
                        let mut did_bup: Rboolean = FALSE;
                        if (if a0 > b0 { a0 } else { b0 }) > 1.0f64 {
                            if b0 <= 1.0f64 {
                                current_block = 16942492502630583769;
                            } else if x0 >= 0.29f64 {
                                current_block = 2336920824898283577;
                            } else if x0 < 0.1f64 && pow(x0 * b0, a0) <= 0.7f64 {
                                current_block = 16942492502630583769;
                            } else if b0 > 15.0f64 {
                                *w1 = 0.0f64;
                                current_block = 1868291631715963762;
                            } else {
                                current_block = 307447392441238883;
                            }
                        } else if a0 >= (if 0.2f64 < b0 { 0.2f64 } else { b0 }) {
                            current_block = 16942492502630583769;
                        } else if pow(x0, a0) <= 0.9f64 {
                            current_block = 16942492502630583769;
                        } else if x0 >= 0.3f64 {
                            current_block = 2336920824898283577;
                        } else {
                            current_block = 307447392441238883;
                        }
                        match current_block {
                            16942492502630583769 => {}
                            _ => {
                                match current_block {
                                    307447392441238883 => {
                                        n = 20 as libc::c_int;
                                        *w1 = bup(b0, a0, y0, x0, n, eps, FALSE as libc::c_int);
                                        did_bup = TRUE;
                                        b0 += n as libc::c_double;
                                        current_block = 1868291631715963762;
                                    }
                                    2336920824898283577 => {
                                        *w1 = bpser(b0, a0, y0, eps, log_p);
                                        *w = if log_p != 0 {
                                            if *w1 > -0.693147180559945309417232121458f64 {
                                                log(-rexpm1(*w1))
                                            } else {
                                                Rlog1p(-exp(*w1))
                                            }
                                        } else {
                                            0.5f64 - *w1 + 0.5f64
                                        };
                                        current_block = 7496337703584048604;
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    7496337703584048604 => {}
                                    _ => {
                                        bgrat(
                                            b0,
                                            a0,
                                            y0,
                                            x0,
                                            w1,
                                            15 as libc::c_int as libc::c_double * eps,
                                            &mut ierr1,
                                            FALSE,
                                        );
                                        if *w1 == 0 as libc::c_int as libc::c_double
                                            || (0 as libc::c_int as libc::c_double) < *w1
                                                && *w1 < 2.2250738585072014e-308f64
                                        {
                                            if did_bup as u64 != 0 {
                                                *w1 = bup(
                                                    b0 - n as libc::c_double,
                                                    a0,
                                                    y0,
                                                    x0,
                                                    n,
                                                    eps,
                                                    TRUE as libc::c_int,
                                                );
                                            } else {
                                                *w1 = -1.0f64 / 0.0f64;
                                            }
                                            bgrat(
                                                b0,
                                                a0,
                                                y0,
                                                x0,
                                                w1,
                                                15 as libc::c_int as libc::c_double * eps,
                                                &mut ierr1,
                                                TRUE,
                                            );
                                            if ierr1 != 0 {
                                                *ierr = 10 as libc::c_int + ierr1;
                                            }
                                            if log_p != 0 {
                                                *w = if *w1 > -0.693147180559945309417232121458f64 {
                                                    log(-rexpm1(*w1))
                                                } else {
                                                    Rlog1p(-exp(*w1))
                                                };
                                            } else {
                                                *w = -expm1(*w1);
                                                *w1 = exp(*w1);
                                            }
                                            current_block = 7496337703584048604;
                                        } else {
                                            if ierr1 != 0 {
                                                *ierr = 10 as libc::c_int + ierr1;
                                            }
                                            if *w1 < 0 as libc::c_int as libc::c_double {
                                                printf(
                                                    b"bratio(a=%g, b=%g, x=%g): bgrat() -> w1 = %g\0"
                                                        as *const u8 as *const libc::c_char,
                                                    a,
                                                    b,
                                                    x,
                                                    *w1,
                                                );
                                            }
                                            current_block = 9167786483348357766;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        7496337703584048604 => {}
                        16942492502630583769 => {}
                        _ => {
                            if log_p != 0 {
                                *w = Rlog1p(-*w1);
                                *w1 = log(*w1);
                            } else {
                                *w = 0.5f64 - *w1 + 0.5f64;
                            }
                            current_block = 7496337703584048604;
                        }
                    }
                }
            } else {
                lambda = if R_finite(a + b) != 0 {
                    if a > b {
                        (a + b) * y - b
                    } else {
                        a - (a + b) * x
                    }
                } else {
                    a * y - b * x
                };
                do_swap = (lambda < 0.0f64) as libc::c_int as Rboolean;
                if do_swap as u64 != 0 {
                    lambda = -lambda;
                    a0 = b;
                    x0 = y;
                    b0 = a;
                    y0 = x;
                } else {
                    a0 = a;
                    x0 = x;
                    b0 = b;
                    y0 = y;
                }
                if b0 < 40.0f64 {
                    if b0 * x0 <= 0.7f64 || log_p != 0 && lambda > 650.0f64 {
                        current_block = 16942492502630583769;
                    } else {
                        n = b0 as libc::c_int;
                        b0 -= n as libc::c_double;
                        if b0 == 0.0f64 {
                            n -= 1;
                            b0 = 1.0f64;
                        }
                        *w = bup(b0, a0, y0, x0, n, eps, FALSE as libc::c_int);
                        if *w < 2.2250738585072014e-308f64 && log_p != 0 {
                            b0 += n as libc::c_double;
                            current_block = 16942492502630583769;
                        } else {
                            if x0 <= 0.7f64 {
                                *w += bpser(a0, b0, x0, eps, FALSE as libc::c_int);
                            } else {
                                if a0 <= 15.0f64 {
                                    n = 20 as libc::c_int;
                                    *w += bup(a0, b0, x0, y0, n, eps, FALSE as libc::c_int);
                                    a0 += n as libc::c_double;
                                }
                                bgrat(
                                    a0,
                                    b0,
                                    x0,
                                    y0,
                                    w,
                                    15 as libc::c_int as libc::c_double * eps,
                                    &mut ierr1,
                                    FALSE,
                                );
                                if ierr1 != 0 {
                                    *ierr = 10 as libc::c_int + ierr1;
                                }
                            }
                            if log_p != 0 {
                                *w1 = Rlog1p(-*w);
                                *w = log(*w);
                            } else {
                                *w1 = 0.5f64 - *w + 0.5f64;
                            }
                            current_block = 7496337703584048604;
                        }
                    }
                } else {
                    if a0 > b0 {
                        if b0 <= 100.0f64 || lambda > b0 * 0.03f64 {
                            current_block = 17597882877141686238;
                        } else {
                            current_block = 15447629348493591490;
                        }
                    } else if a0 <= 100.0f64 {
                        current_block = 17597882877141686238;
                    } else if lambda > a0 * 0.03f64 {
                        current_block = 17597882877141686238;
                    } else {
                        current_block = 15447629348493591490;
                    }
                    match current_block {
                        17597882877141686238 => {
                            *w = bfrac(a0, b0, x0, y0, lambda, eps * 15.0f64, log_p);
                            *w1 = if log_p != 0 {
                                if *w > -0.693147180559945309417232121458f64 {
                                    log(-rexpm1(*w))
                                } else {
                                    Rlog1p(-exp(*w))
                                }
                            } else {
                                0.5f64 - *w + 0.5f64
                            };
                        }
                        _ => {
                            *w = basym(a0, b0, lambda, eps * 100.0f64, log_p);
                            *w1 = if log_p != 0 {
                                if *w > -0.693147180559945309417232121458f64 {
                                    log(-rexpm1(*w))
                                } else {
                                    Rlog1p(-exp(*w))
                                }
                            } else {
                                0.5f64 - *w + 0.5f64
                            };
                        }
                    }
                    current_block = 7496337703584048604;
                }
            }
            match current_block {
                16942492502630583769 => {
                    *w = bpser(a0, b0, x0, eps, log_p);
                    *w1 = if log_p != 0 {
                        if *w > -0.693147180559945309417232121458f64 {
                            log(-rexpm1(*w))
                        } else {
                            Rlog1p(-exp(*w))
                        }
                    } else {
                        0.5f64 - *w + 0.5f64
                    };
                }
                _ => {}
            }
            if do_swap as u64 != 0 {
                let mut t: libc::c_double = *w;
                *w = *w1;
                *w1 = t;
            }
            return;
        }
        match current_block {
            12410001467641165671 => {}
            _ => {
                *w = if log_p != 0 { 0.0f64 } else { 1.0f64 };
                *w1 = if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
                return;
            }
        }
    }
    *w = if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    *w1 = if log_p != 0 { 0.0f64 } else { 1.0f64 };
}
unsafe extern "C" fn fpser(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut eps: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut ans: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut an: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    if log_p != 0 {
        ans = a * log(x);
    } else if a > eps * 0.001f64 {
        t = a * log(x);
        if t < exparg(1 as libc::c_int) {
            return 0.0f64;
        }
        ans = exp(t);
    } else {
        ans = 1.0f64;
    }
    if log_p != 0 {
        ans += log(b) - log(a);
    } else {
        ans *= b / a;
    }
    tol = eps / a;
    an = a + 1.0f64;
    t = x;
    s = t / an;
    loop {
        an += 1.0f64;
        t = x * t;
        c = t / an;
        s += c;
        if !(fabs(c) > tol) {
            break;
        }
    }
    if log_p != 0 {
        ans += Rlog1p(a * s);
    } else {
        ans *= a * s + 1.0f64;
    }
    return ans;
}
unsafe extern "C" fn apser(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut eps: libc::c_double,
) -> libc::c_double {
    static mut g: libc::c_double = 0.577215664901533f64;
    let mut tol: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut j: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut aj: libc::c_double = 0.;
    let mut bx: libc::c_double = b * x;
    t = x - bx;
    if b * eps <= 0.02f64 {
        c = log(x) + psi(b) + g + t;
    } else {
        c = log(bx) + g + t;
    }
    tol = eps * 5.0f64 * fabs(c);
    j = 1.0f64;
    s = 0.0f64;
    loop {
        j += 1.0f64;
        t *= x - bx / j;
        aj = t / j;
        s += aj;
        if !(fabs(aj) > tol) {
            break;
        }
    }
    return -a * (c + s);
}
unsafe extern "C" fn bpser(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut eps: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut ans: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    let mut b0: libc::c_double = 0.;
    let mut apb: libc::c_double = 0.;
    if x == 0.0f64 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    a0 = if a < b { a } else { b };
    if a0 >= 1.0f64 {
        z = a * log(x) - betaln(a, b);
        ans = if log_p != 0 { z - log(a) } else { exp(z) / a };
    } else {
        b0 = if a > b { a } else { b };
        if b0 < 8.0f64 {
            if b0 <= 1.0f64 {
                if log_p != 0 {
                    ans = a * log(x);
                } else {
                    ans = pow(x, a);
                    if ans == 0.0f64 {
                        return ans;
                    }
                }
                apb = a + b;
                if apb > 1.0f64 {
                    u = a + b - 1.0f64;
                    z = (gam1(u) + 1.0f64) / apb;
                } else {
                    z = gam1(apb) + 1.0f64;
                }
                c = (gam1(a) + 1.0f64) * (gam1(b) + 1.0f64) / z;
                if log_p != 0 {
                    ans += log(c * (b / apb));
                } else {
                    ans *= c * (b / apb);
                }
            } else {
                u = gamln1(a0);
                m = (b0 - 1.0f64) as libc::c_int;
                if m >= 1 as libc::c_int {
                    c = 1.0f64;
                    i = 1 as libc::c_int;
                    while i <= m {
                        b0 += -1.0f64;
                        c *= b0 / (a0 + b0);
                        i += 1;
                    }
                    u += log(c);
                }
                z = a * log(x) - u;
                b0 += -1.0f64;
                apb = a0 + b0;
                if apb > 1.0f64 {
                    u = a0 + b0 - 1.0f64;
                    t = (gam1(u) + 1.0f64) / apb;
                } else {
                    t = gam1(apb) + 1.0f64;
                }
                if log_p != 0 {
                    ans = z + log(a0 / a) + Rlog1p(gam1(b0)) - log(t);
                } else {
                    ans = exp(z) * (a0 / a) * (gam1(b0) + 1.0f64) / t;
                }
            }
        } else {
            u = gamln1(a0) + algdiv(a0, b0);
            z = a * log(x) - u;
            if log_p != 0 {
                ans = z + log(a0 / a);
            } else {
                ans = a0 / a * exp(z);
            }
        }
    }
    if ans == (if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 })
        || log_p == 0 && a <= eps * 0.1f64
    {
        return ans;
    }
    let mut tol: libc::c_double = eps / a;
    let mut n: libc::c_double = 0.0f64;
    let mut sum: libc::c_double = 0.0f64;
    let mut w: libc::c_double = 0.;
    c = 1.0f64;
    loop {
        n += 1.0f64;
        c *= (0.5f64 - b / n + 0.5f64) * x;
        w = c / (a + n);
        sum += w;
        if !(n < 1e7f64 && fabs(w) > tol) {
            break;
        }
    }
    if fabs(w) > tol {
        if log_p != 0 && !(a * sum > -1.0f64 && fabs(Rlog1p(a * sum)) < eps * fabs(ans))
            || log_p == 0 && fabs(a * sum + 1.0f64) != 1.0f64
        {
            printf(
                b" bpser(a=%g, b=%g, x=%g,...) did not converge (n=1e7, |w|/tol=%g > 1; A=%g)\0"
                    as *const u8 as *const libc::c_char,
                a,
                b,
                x,
                fabs(w) / tol,
                ans,
            );
        }
    }
    if log_p != 0 {
        if a * sum > -1.0f64 {
            ans += Rlog1p(a * sum);
        } else {
            if ans > -1.0f64 / 0.0f64 {
                printf(
                    b"pbeta(*, log.p=TRUE) -> bpser(a=%g, b=%g, x=%g,...) underflow to -Inf\0"
                        as *const u8 as *const libc::c_char,
                    a,
                    b,
                    x,
                );
            }
            ans = -1.0f64 / 0.0f64;
        }
    } else if a * sum > -1.0f64 {
        ans *= a * sum + 1.0f64;
    } else {
        ans = 0.0f64;
    }
    return ans;
}
unsafe extern "C" fn bup(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut n: libc::c_int,
    mut eps: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut ret_val: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mu: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut l: libc::c_double = 0.;
    let mut apb: libc::c_double = a + b;
    let mut ap1: libc::c_double = a + 1.0f64;
    if n > 1 as libc::c_int && a >= 1.0f64 && apb >= ap1 * 1.1f64 {
        mu = fabs(exparg(1 as libc::c_int)) as libc::c_int;
        k = exparg(0 as libc::c_int) as libc::c_int;
        if mu > k {
            mu = k;
        }
        d = exp(-(mu as libc::c_double));
    } else {
        mu = 0 as libc::c_int;
        d = 1.0f64;
    }
    ret_val = if log_p != 0 {
        brcmp1(mu, a, b, x, y, TRUE as libc::c_int) - log(a)
    } else {
        brcmp1(mu, a, b, x, y, FALSE as libc::c_int) / a
    };
    if n == 1 as libc::c_int
        || log_p != 0 && ret_val == -1.0f64 / 0.0f64
        || log_p == 0 && ret_val == 0.0f64
    {
        return ret_val;
    }
    let mut nm1: libc::c_int = n - 1 as libc::c_int;
    let mut w: libc::c_double = d;
    k = 0 as libc::c_int;
    if b > 1.0f64 {
        if y > 1e-4f64 {
            let mut r: libc::c_double = (b - 1.0f64) * x / y - a;
            if r >= 1.0f64 {
                k = if r < nm1 as libc::c_double {
                    r as libc::c_int
                } else {
                    nm1
                };
            }
        } else {
            k = nm1;
        }
        i = 0 as libc::c_int;
        while i < k {
            l = i as libc::c_double;
            d *= (apb + l) / (ap1 + l) * x;
            w += d;
            i += 1;
        }
    }
    i = k;
    while i < nm1 {
        l = i as libc::c_double;
        d *= (apb + l) / (ap1 + l) * x;
        w += d;
        if d <= eps * w {
            break;
        }
        i += 1;
    }
    if log_p != 0 {
        ret_val += log(w);
    } else {
        ret_val *= w;
    }
    return ret_val;
}
unsafe extern "C" fn bfrac(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut lambda: libc::c_double,
    mut eps: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut c: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut n: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut c0: libc::c_double = 0.;
    let mut c1: libc::c_double = 0.;
    let mut r0: libc::c_double = 0.;
    let mut an: libc::c_double = 0.;
    let mut bn: libc::c_double = 0.;
    let mut yp1: libc::c_double = 0.;
    let mut anp1: libc::c_double = 0.;
    let mut bnp1: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut brc: libc::c_double = 0.;
    if R_finite(lambda) == 0 {
        return 0.0f64 / 0.0f64;
    }
    brc = brcomp(a, b, x, y, log_p);
    if brc.is_nan() as i32 != 0 as libc::c_int {
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
    if log_p == 0 && brc == 0.0f64 {
        return 0.0f64;
    }
    c = lambda + 1.0f64;
    c0 = b / a;
    c1 = 1.0f64 / a + 1.0f64;
    yp1 = y + 1.0f64;
    n = 0.0f64;
    p = 1.0f64;
    s = a + 1.0f64;
    an = 0.0f64;
    bn = 1.0f64;
    anp1 = 1.0f64;
    bnp1 = c / c1;
    r = c1 / c;
    loop {
        n += 1.0f64;
        t = n / a;
        w = n * (b - n) * x;
        e = a / s;
        alpha = p * (p + c0) * e * e * (w * x);
        e = (t + 1.0f64) / (c1 + t + t);
        beta = n + w / s + e * (c + n * yp1);
        p = t + 1.0f64;
        s += 2.0f64;
        t = alpha * an + beta * anp1;
        an = anp1;
        anp1 = t;
        t = alpha * bn + beta * bnp1;
        bn = bnp1;
        bnp1 = t;
        r0 = r;
        r = anp1 / bnp1;
        if fabs(r - r0) <= eps * r {
            break;
        }
        an /= bnp1;
        bn /= bnp1;
        anp1 = r;
        bnp1 = 1.0f64;
        if !(n < 10000 as libc::c_int as libc::c_double) {
            break;
        }
    }
    if n >= 10000 as libc::c_int as libc::c_double && fabs(r - r0) > eps * r {
        printf(
            b" bfrac(a=%g, b=%g, x=%g, y=%g, lambda=%g) did *not* converge (in 10000 steps)\n\0"
                as *const u8 as *const libc::c_char,
            a,
            b,
            x,
            y,
            lambda,
        );
    }
    return if log_p != 0 { brc + log(r) } else { brc * r };
}
unsafe extern "C" fn brcomp(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut const__: libc::c_double = 0.398942280401433f64;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    let mut b0: libc::c_double = 0.;
    let mut apb: libc::c_double = 0.;
    if x == 0.0f64 || y == 0.0f64 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    a0 = if a < b { a } else { b };
    if a0 < 8.0f64 {
        let mut lnx: libc::c_double = 0.;
        let mut lny: libc::c_double = 0.;
        if x <= 0.375f64 {
            lnx = log(x);
            lny = alnrel(-x);
        } else if y > 0.375f64 {
            lnx = log(x);
            lny = log(y);
        } else {
            lnx = alnrel(-y);
            lny = log(y);
        }
        z = a * lnx + b * lny;
        if a0 >= 1.0f64 {
            z -= betaln(a, b);
            return if log_p != 0 { z } else { exp(z) };
        }
        b0 = if a > b { a } else { b };
        if b0 >= 8.0f64 {
            u = gamln1(a0) + algdiv(a0, b0);
            return if log_p != 0 {
                log(a0) + (z - u)
            } else {
                a0 * exp(z - u)
            };
        }
        if b0 <= 1.0f64 {
            let mut e_z: libc::c_double = if log_p != 0 { z } else { exp(z) };
            if log_p == 0 && e_z == 0.0f64 {
                return 0.0f64;
            }
            apb = a + b;
            if apb > 1.0f64 {
                u = a + b - 1.0f64;
                z = (gam1(u) + 1.0f64) / apb;
            } else {
                z = gam1(apb) + 1.0f64;
            }
            c = (gam1(a) + 1.0f64) * (gam1(b) + 1.0f64) / z;
            return if log_p != 0 {
                e_z + log(a0 * c) - Rlog1p(a0 / b0)
            } else {
                e_z * (a0 * c) / (a0 / b0 + 1.0f64)
            };
        }
        u = gamln1(a0);
        n = (b0 - 1.0f64) as libc::c_int;
        if n >= 1 as libc::c_int {
            c = 1.0f64;
            i = 1 as libc::c_int;
            while i <= n {
                b0 += -1.0f64;
                c *= b0 / (a0 + b0);
                i += 1;
            }
            u = log(c) + u;
        }
        z -= u;
        b0 += -1.0f64;
        apb = a0 + b0;
        let mut t: libc::c_double = 0.;
        if apb > 1.0f64 {
            u = a0 + b0 - 1.0f64;
            t = (gam1(u) + 1.0f64) / apb;
        } else {
            t = gam1(apb) + 1.0f64;
        }
        return if log_p != 0 {
            log(a0) + z + Rlog1p(gam1(b0)) - log(t)
        } else {
            a0 * exp(z) * (gam1(b0) + 1.0f64) / t
        };
    } else {
        let mut h: libc::c_double = 0.;
        let mut x0: libc::c_double = 0.;
        let mut y0: libc::c_double = 0.;
        let mut lambda: libc::c_double = 0.;
        if a <= b {
            h = a / b;
            x0 = h / (h + 1.0f64);
            y0 = 1.0f64 / (h + 1.0f64);
            lambda = a - (a + b) * x;
        } else {
            h = b / a;
            x0 = 1.0f64 / (h + 1.0f64);
            y0 = h / (h + 1.0f64);
            lambda = (a + b) * y - b;
        }
        e = -lambda / a;
        if fabs(e) > 0.6f64 {
            u = e - log(x / x0);
        } else {
            u = rlog1(e);
        }
        e = lambda / b;
        if fabs(e) <= 0.6f64 {
            v = rlog1(e);
        } else {
            v = e - log(y / y0);
        }
        z = if log_p != 0 {
            -(a * u + b * v)
        } else {
            exp(-(a * u + b * v))
        };
        return if log_p != 0 {
            -0.918938533204672741780329736406f64 + 0.5f64 * log(b * x0) + z - bcorr(a, b)
        } else {
            const__ * sqrt(b * x0) * z * exp(-bcorr(a, b))
        };
    };
}
unsafe extern "C" fn brcmp1(
    mut mu: libc::c_int,
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut const__: libc::c_double = 0.398942280401433f64;
    let mut c: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut a0: libc::c_double = 0.;
    let mut b0: libc::c_double = 0.;
    let mut apb: libc::c_double = 0.;
    a0 = if a < b { a } else { b };
    if a0 < 8.0f64 {
        let mut lnx: libc::c_double = 0.;
        let mut lny: libc::c_double = 0.;
        if x <= 0.375f64 {
            lnx = log(x);
            lny = alnrel(-x);
        } else if y > 0.375f64 {
            lnx = log(x);
            lny = log(y);
        } else {
            lnx = alnrel(-y);
            lny = log(y);
        }
        z = a * lnx + b * lny;
        if a0 >= 1.0f64 {
            z -= betaln(a, b);
            return esum(mu, z, log_p);
        }
        b0 = if a > b { a } else { b };
        if b0 >= 8.0f64 {
            u = gamln1(a0) + algdiv(a0, b0);
            return if log_p != 0 {
                log(a0) + esum(mu, z - u, TRUE as libc::c_int)
            } else {
                a0 * esum(mu, z - u, FALSE as libc::c_int)
            };
        } else {
            if b0 <= 1.0f64 {
                let mut ans: libc::c_double = esum(mu, z, log_p);
                if ans == (if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 }) {
                    return ans;
                }
                apb = a + b;
                if apb > 1.0f64 {
                    u = a + b - 1.0f64;
                    z = (gam1(u) + 1.0f64) / apb;
                } else {
                    z = gam1(apb) + 1.0f64;
                }
                c = if log_p != 0 {
                    Rlog1p(gam1(a)) + Rlog1p(gam1(b)) - log(z)
                } else {
                    (gam1(a) + 1.0f64) * (gam1(b) + 1.0f64) / z
                };
                return if log_p != 0 {
                    ans + log(a0) + c - Rlog1p(a0 / b0)
                } else {
                    ans * (a0 * c) / (a0 / b0 + 1.0f64)
                };
            }
        }
        u = gamln1(a0);
        let mut n: libc::c_int = (b0 - 1.0f64) as libc::c_int;
        if n >= 1 as libc::c_int {
            c = 1.0f64;
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= n {
                b0 += -1.0f64;
                c *= b0 / (a0 + b0);
                i += 1;
            }
            u += log(c);
        }
        z -= u;
        b0 += -1.0f64;
        apb = a0 + b0;
        if apb > 1.0f64 {
            t = (gam1(apb - 1.0f64) + 1.0f64) / apb;
        } else {
            t = gam1(apb) + 1.0f64;
        }
        return if log_p != 0 {
            log(a0) + esum(mu, z, TRUE as libc::c_int) + Rlog1p(gam1(b0)) - log(t)
        } else {
            a0 * esum(mu, z, FALSE as libc::c_int) * (gam1(b0) + 1.0f64) / t
        };
    } else {
        let mut h: libc::c_double = 0.;
        let mut x0: libc::c_double = 0.;
        let mut y0: libc::c_double = 0.;
        let mut lambda: libc::c_double = 0.;
        if a > b {
            h = b / a;
            x0 = 1.0f64 / (h + 1.0f64);
            y0 = h / (h + 1.0f64);
            lambda = (a + b) * y - b;
        } else {
            h = a / b;
            x0 = h / (h + 1.0f64);
            y0 = 1.0f64 / (h + 1.0f64);
            lambda = a - (a + b) * x;
        }
        let mut lx0: libc::c_double = -Rlog1p(b / a);
        let mut e: libc::c_double = -lambda / a;
        if fabs(e) > 0.6f64 {
            u = e - log(x / x0);
        } else {
            u = rlog1(e);
        }
        e = lambda / b;
        if fabs(e) > 0.6f64 {
            v = e - log(y / y0);
        } else {
            v = rlog1(e);
        }
        z = esum(mu, -(a * u + b * v), log_p);
        return if log_p != 0 {
            log(const__) + (log(b) + lx0) / 2.0f64 + z - bcorr(a, b)
        } else {
            const__ * sqrt(b * x0) * z * exp(-bcorr(a, b))
        };
    };
}
unsafe extern "C" fn bgrat(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut x: libc::c_double,
    mut y: libc::c_double,
    mut w: *mut libc::c_double,
    mut eps: libc::c_double,
    mut ierr: *mut libc::c_int,
    mut log_w: Rboolean,
) {
    let mut c: [libc::c_double; 30] = [0.; 30];
    let mut d: [libc::c_double; 30] = [0.; 30];
    let mut bm1: libc::c_double = b - 0.5f64 - 0.5f64;
    let mut nu: libc::c_double = a + bm1 * 0.5f64;
    let mut lnx: libc::c_double = if y > 0.375f64 { log(x) } else { alnrel(-y) };
    let mut z: libc::c_double = -nu * lnx;
    if b * z == 0.0f64 {
        printf(
            b"bgrat(a=%g, b=%g, x=%g, y=%g): z=%g, b*z == 0 underflow, hence inaccurate pbeta()\0"
                as *const u8 as *const libc::c_char,
            a,
            b,
            x,
            y,
            z,
        );
        *ierr = 1 as libc::c_int;
        return;
    }
    let mut log_r: libc::c_double = log(b) + Rlog1p(gam1(b)) + b * log(z) + nu * lnx;
    let mut log_u: libc::c_double = log_r - (algdiv(b, a) + b * log(nu));
    let mut u: libc::c_double = exp(log_u);
    if log_u == -1.0f64 / 0.0f64 {
        *ierr = 2 as libc::c_int;
        return;
    }
    let mut u_0: Rboolean = (u == 0.0f64) as libc::c_int as Rboolean;
    let mut l: libc::c_double = if log_w as libc::c_uint != 0 {
        if *w == -1.0f64 / 0.0f64 {
            0.0f64
        } else {
            exp(*w - log_u)
        }
    } else if *w == 0.0f64 {
        0.0f64
    } else {
        exp(log(*w) - log_u)
    };
    let mut q_r: libc::c_double = grat_r(b, z, log_r, eps);
    let mut v: libc::c_double = 0.25f64 / (nu * nu);
    let mut t2: libc::c_double = lnx * 0.25f64 * lnx;
    let mut j: libc::c_double = q_r;
    let mut sum: libc::c_double = j;
    let mut t: libc::c_double = 1.0f64;
    let mut cn: libc::c_double = 1.0f64;
    let mut n2: libc::c_double = 0.0f64;
    let mut n: libc::c_int = 1 as libc::c_int;
    while n <= 30 as libc::c_int {
        let mut bp2n: libc::c_double = b + n2;
        j = (bp2n * (bp2n + 1.0f64) * j + (z + bp2n + 1.0f64) * t) * v;
        n2 += 2.0f64;
        t *= t2;
        cn /= n2 * (n2 + 1.0f64);
        let mut nm1: libc::c_int = n - 1 as libc::c_int;
        c[nm1 as usize] = cn;
        let mut s: libc::c_double = 0.0f64;
        if n > 1 as libc::c_int {
            let mut coef: libc::c_double = b - n as libc::c_double;
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= nm1 {
                s += coef * c[(i - 1 as libc::c_int) as usize] * d[(nm1 - i) as usize];
                coef += b;
                i += 1;
            }
        }
        d[nm1 as usize] = bm1 * cn + s / n as libc::c_double;
        let mut dj: libc::c_double = d[nm1 as usize] * j;
        sum += dj;
        if sum <= 0.0f64 {
            *ierr = 3 as libc::c_int;
            return;
        }
        if fabs(dj) <= eps * (sum + l) {
            *ierr = 0 as libc::c_int;
            break;
        } else {
            if n == 30 as libc::c_int {
                *ierr = 4 as libc::c_int;
                printf(
                    b"bgrat(a=%g, b=%g, x=%g) *no* convergence: NOTIFY R-core!\n dj=%g, rel.err=%g\n\0"
                        as *const u8 as *const libc::c_char,
                    a,
                    b,
                    x,
                    dj,
                    fabs(dj) / (sum + l),
                );
            }
            n += 1;
        }
    }
    if log_w as u64 != 0 {
        *w = logspace_add(*w, log_u + log(sum));
    } else {
        *w += if u_0 as libc::c_uint != 0 {
            exp(log_u + log(sum))
        } else {
            u * sum
        };
    };
}
unsafe extern "C" fn grat_r(
    mut a: libc::c_double,
    mut x: libc::c_double,
    mut log_r: libc::c_double,
    mut eps: libc::c_double,
) -> libc::c_double {
    if a * x == 0.0f64 {
        if x <= a {
            return exp(-log_r);
        } else {
            return 0.0f64;
        }
    } else if a == 0.5f64 {
        if x < 0.25f64 {
            let mut p: libc::c_double = erf__(sqrt(x));
            return (0.5f64 - p + 0.5f64) * exp(-log_r);
        } else {
            let mut sx: libc::c_double = sqrt(x);
            let mut q_r: libc::c_double =
                erfc1(1 as libc::c_int, sx) / sx * 1.772453850905516027298167483341f64;
            return q_r;
        }
    } else if x < 1.1f64 {
        let mut an: libc::c_double = 3.0f64;
        let mut c: libc::c_double = x;
        let mut sum: libc::c_double = x / (a + 3.0f64);
        let mut tol: libc::c_double = eps * 0.1f64 / (a + 1.0f64);
        let mut t: libc::c_double = 0.;
        loop {
            an += 1.0f64;
            c *= -(x / an);
            t = c / (a + an);
            sum += t;
            if !(fabs(t) > tol) {
                break;
            }
        }
        let mut j: libc::c_double =
            a * x * ((sum / 6.0f64 - 0.5f64 / (a + 2.0f64)) * x + 1.0f64 / (a + 1.0f64));
        let mut z: libc::c_double = a * log(x);
        let mut h: libc::c_double = gam1(a);
        let mut g: libc::c_double = h + 1.0f64;
        if x >= 0.25f64 && a < x / 2.59f64 || z > -0.13394f64 {
            let mut l: libc::c_double = rexpm1(z);
            let mut q: libc::c_double = ((l + 0.5f64 + 0.5f64) * j - l) * g - h;
            if q <= 0.0f64 {
                return 0.0f64;
            } else {
                return q * exp(-log_r);
            }
        } else {
            let mut p_0: libc::c_double = exp(z) * g * (0.5f64 - j + 0.5f64);
            return (0.5f64 - p_0 + 0.5f64) * exp(-log_r);
        }
    } else {
        let mut a2n_1: libc::c_double = 1.0f64;
        let mut a2n: libc::c_double = 1.0f64;
        let mut b2n_1: libc::c_double = x;
        let mut b2n: libc::c_double = x + (1.0f64 - a);
        let mut c_0: libc::c_double = 1.0f64;
        let mut am0: libc::c_double = 0.;
        let mut an0: libc::c_double = 0.;
        loop {
            a2n_1 = x * a2n + c_0 * a2n_1;
            b2n_1 = x * b2n + c_0 * b2n_1;
            am0 = a2n_1 / b2n_1;
            c_0 += 1.0f64;
            let mut c_a: libc::c_double = c_0 - a;
            a2n = a2n_1 + c_a * a2n;
            b2n = b2n_1 + c_a * b2n;
            an0 = a2n / b2n;
            if !(fabs(an0 - am0) >= eps * an0) {
                break;
            }
        }
        return an0;
    };
}
unsafe extern "C" fn basym(
    mut a: libc::c_double,
    mut b: libc::c_double,
    mut lambda: libc::c_double,
    mut eps: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut e0: libc::c_double = 1.12837916709551f64;
    static mut e1: libc::c_double = 0.353553390593274f64;
    static mut ln_e0: libc::c_double = 0.120782237635245f64;
    let mut a0: [libc::c_double; 21] = [0.; 21];
    let mut b0: [libc::c_double; 21] = [0.; 21];
    let mut c: [libc::c_double; 21] = [0.; 21];
    let mut d: [libc::c_double; 21] = [0.; 21];
    let mut f: libc::c_double = a * rlog1(-lambda / a) + b * rlog1(lambda / b);
    let mut t: libc::c_double = 0.;
    if log_p != 0 {
        t = -f;
    } else {
        t = exp(-f);
        if t == 0.0f64 {
            return 0 as libc::c_int as libc::c_double;
        }
    }
    let mut z0: libc::c_double = sqrt(f);
    let mut z: libc::c_double = z0 / e1 * 0.5f64;
    let mut z2: libc::c_double = f + f;
    let mut h: libc::c_double = 0.;
    let mut r0: libc::c_double = 0.;
    let mut r1: libc::c_double = 0.;
    let mut w0: libc::c_double = 0.;
    if a < b {
        h = a / b;
        r0 = 1.0f64 / (h + 1.0f64);
        r1 = (b - a) / b;
        w0 = 1.0f64 / sqrt(a * (h + 1.0f64));
    } else {
        h = b / a;
        r0 = 1.0f64 / (h + 1.0f64);
        r1 = (b - a) / a;
        w0 = 1.0f64 / sqrt(b * (h + 1.0f64));
    }
    a0[0 as libc::c_int as usize] = r1 * 0.66666666666666663f64;
    c[0 as libc::c_int as usize] = a0[0 as libc::c_int as usize] * -0.5f64;
    d[0 as libc::c_int as usize] = -c[0 as libc::c_int as usize];
    let mut j0: libc::c_double = 0.5f64 / e0 * erfc1(1 as libc::c_int, z0);
    let mut j1: libc::c_double = e1;
    let mut sum: libc::c_double = j0 + d[0 as libc::c_int as usize] * w0 * j1;
    let mut s: libc::c_double = 1.0f64;
    let mut h2: libc::c_double = h * h;
    let mut hn: libc::c_double = 1.0f64;
    let mut w: libc::c_double = w0;
    let mut znm1: libc::c_double = z;
    let mut zn: libc::c_double = z2;
    let mut n: libc::c_int = 2 as libc::c_int;
    while n <= 20 as libc::c_int {
        hn *= h2;
        a0[(n - 1 as libc::c_int) as usize] =
            r0 * 2.0f64 * (h * hn + 1.0f64) / (n as libc::c_double + 2.0f64);
        let mut np1: libc::c_int = n + 1 as libc::c_int;
        s += hn;
        a0[(np1 - 1 as libc::c_int) as usize] = r1 * 2.0f64 * s / (n as libc::c_double + 3.0f64);
        let mut i: libc::c_int = n;
        while i <= np1 {
            let mut r: libc::c_double = (i as libc::c_double + 1.0f64) * -0.5f64;
            b0[0 as libc::c_int as usize] = r * a0[0 as libc::c_int as usize];
            let mut m: libc::c_int = 2 as libc::c_int;
            while m <= i {
                let mut bsum: libc::c_double = 0.0f64;
                let mut j: libc::c_int = 1 as libc::c_int;
                while j <= m - 1 as libc::c_int {
                    let mut mmj: libc::c_int = m - j;
                    bsum += (j as libc::c_double * r - mmj as libc::c_double)
                        * a0[(j - 1 as libc::c_int) as usize]
                        * b0[(mmj - 1 as libc::c_int) as usize];
                    j += 1;
                }
                b0[(m - 1 as libc::c_int) as usize] =
                    r * a0[(m - 1 as libc::c_int) as usize] + bsum / m as libc::c_double;
                m += 1;
            }
            c[(i - 1 as libc::c_int) as usize] =
                b0[(i - 1 as libc::c_int) as usize] / (i as libc::c_double + 1.0f64);
            let mut dsum: libc::c_double = 0.0f64;
            let mut j_0: libc::c_int = 1 as libc::c_int;
            while j_0 <= i - 1 as libc::c_int {
                dsum +=
                    d[(i - j_0 - 1 as libc::c_int) as usize] * c[(j_0 - 1 as libc::c_int) as usize];
                j_0 += 1;
            }
            d[(i - 1 as libc::c_int) as usize] = -(dsum + c[(i - 1 as libc::c_int) as usize]);
            i += 1;
        }
        j0 = e1 * znm1 + (n as libc::c_double - 1.0f64) * j0;
        j1 = e1 * zn + n as libc::c_double * j1;
        znm1 = z2 * znm1;
        zn = z2 * zn;
        w *= w0;
        let mut t0: libc::c_double = d[(n - 1 as libc::c_int) as usize] * w * j0;
        w *= w0;
        let mut t1: libc::c_double = d[(np1 - 1 as libc::c_int) as usize] * w * j1;
        sum += t0 + t1;
        if fabs(t0) + fabs(t1) <= eps * sum {
            break;
        }
        n += 2 as libc::c_int;
    }
    if log_p != 0 {
        return ln_e0 + t - bcorr(a, b) + log(sum);
    } else {
        let mut u: libc::c_double = exp(-bcorr(a, b));
        return e0 * t * u * sum;
    };
}
unsafe extern "C" fn exparg(mut l: libc::c_int) -> libc::c_double {
    static mut lnb: libc::c_double = 0.69314718055995f64;
    let mut m: libc::c_int = if l == 0 as libc::c_int {
        Rf_i1mach(16 as libc::c_int)
    } else {
        Rf_i1mach(15 as libc::c_int) - 1 as libc::c_int
    };
    return m as libc::c_double * lnb * 0.99999f64;
}
unsafe extern "C" fn esum(
    mut mu: libc::c_int,
    mut x: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    if log_p != 0 {
        return x + mu as libc::c_double;
    }
    let mut w: libc::c_double = 0.;
    if x > 0.0f64 {
        if mu > 0 as libc::c_int {
            return exp(mu as libc::c_double) * exp(x);
        }
        w = mu as libc::c_double + x;
        if w < 0.0f64 {
            return exp(mu as libc::c_double) * exp(x);
        }
    } else {
        if mu < 0 as libc::c_int {
            return exp(mu as libc::c_double) * exp(x);
        }
        w = mu as libc::c_double + x;
        if w > 0.0f64 {
            return exp(mu as libc::c_double) * exp(x);
        }
    }
    return exp(w);
}
unsafe extern "C" fn rexpm1(mut x: libc::c_double) -> libc::c_double {
    static mut p1: libc::c_double = 9.14041914819518e-10f64;
    static mut p2: libc::c_double = 0.0238082361044469f64;
    static mut q1: libc::c_double = -0.499999999085958f64;
    static mut q2: libc::c_double = 0.107141568980644f64;
    static mut q3: libc::c_double = -0.0119041179760821f64;
    static mut q4: libc::c_double = 5.95130811860248e-4f64;
    if fabs(x) <= 0.15f64 {
        return x
            * (((p2 * x + p1) * x + 1.0f64) / ((((q4 * x + q3) * x + q2) * x + q1) * x + 1.0f64));
    } else {
        let mut w: libc::c_double = exp(x);
        if x > 0.0f64 {
            return w * (0.5f64 - 1.0f64 / w + 0.5f64);
        } else {
            return w - 0.5f64 - 0.5f64;
        }
    };
}
unsafe extern "C" fn alnrel(mut a: libc::c_double) -> libc::c_double {
    if fabs(a) > 0.375f64 {
        return log(1.0f64 + a);
    }
    static mut p1: libc::c_double = -1.29418923021993f64;
    static mut p2: libc::c_double = 0.405303492862024f64;
    static mut p3: libc::c_double = -0.0178874546012214f64;
    static mut q1: libc::c_double = -1.62752256355323f64;
    static mut q2: libc::c_double = 0.747811014037616f64;
    static mut q3: libc::c_double = -0.0845104217945565f64;
    let mut t: libc::c_double = a / (a + 2.0f64);
    let mut t2: libc::c_double = t * t;
    let mut w: libc::c_double =
        (((p3 * t2 + p2) * t2 + p1) * t2 + 1.0f64) / (((q3 * t2 + q2) * t2 + q1) * t2 + 1.0f64);
    return t * 2.0f64 * w;
}
unsafe extern "C" fn rlog1(mut x: libc::c_double) -> libc::c_double {
    static mut a: libc::c_double = 0.0566749439387324f64;
    static mut b: libc::c_double = 0.0456512608815524f64;
    static mut p0: libc::c_double = 0.333333333333333f64;
    static mut p1: libc::c_double = -0.224696413112536f64;
    static mut p2: libc::c_double = 0.00620886815375787f64;
    static mut q1: libc::c_double = -1.27408923933623f64;
    static mut q2: libc::c_double = 0.354508718369557f64;
    let mut h: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut w1: libc::c_double = 0.;
    if x < -0.39f64 || x > 0.57f64 {
        w = x + 0.5f64 + 0.5f64;
        return x - log(w);
    }
    if x < -0.18f64 {
        h = x + 0.3f64;
        h /= 0.7f64;
        w1 = a - h * 0.3f64;
    } else if x > 0.18f64 {
        h = x * 0.75f64 - 0.25f64;
        w1 = b + h / 3.0f64;
    } else {
        h = x;
        w1 = 0.0f64;
    }
    r = h / (h + 2.0f64);
    t = r * r;
    w = ((p2 * t + p1) * t + p0) / ((q2 * t + q1) * t + 1.0f64);
    return t * 2.0f64 * (1.0f64 / (1.0f64 - r) - r * w) + w1;
}
unsafe extern "C" fn erf__(mut x: libc::c_double) -> libc::c_double {
    static mut c: libc::c_double = 0.564189583547756f64;
    static mut a: [libc::c_double; 5] = [
        7.7105849500132e-5f64,
        -0.00133733772997339f64,
        0.0323076579225834f64,
        0.0479137145607681f64,
        0.128379167095513f64,
    ];
    static mut b: [libc::c_double; 3] = [
        0.00301048631703895f64,
        0.0538971687740286f64,
        0.375795757275549f64,
    ];
    static mut p: [libc::c_double; 8] = [
        -1.36864857382717e-7f64,
        0.564195517478974f64,
        7.21175825088309f64,
        43.1622272220567f64,
        152.98928504694f64,
        339.320816734344f64,
        451.918953711873f64,
        300.459261020162f64,
    ];
    static mut q: [libc::c_double; 8] = [
        1.0f64,
        12.7827273196294f64,
        77.0001529352295f64,
        277.585444743988f64,
        638.980264465631f64,
        931.35409485061f64,
        790.950925327898f64,
        300.459260956983f64,
    ];
    static mut r: [libc::c_double; 5] = [
        2.10144126479064f64,
        26.2370141675169f64,
        21.3688200555087f64,
        4.6580782871847f64,
        0.282094791773523f64,
    ];
    static mut s: [libc::c_double; 4] = [
        94.153775055546f64,
        187.11481179959f64,
        99.0191814623914f64,
        18.0124575948747f64,
    ];
    let mut t: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut ax: libc::c_double = 0.;
    let mut bot: libc::c_double = 0.;
    let mut top: libc::c_double = 0.;
    ax = fabs(x);
    if ax <= 0.5f64 {
        t = x * x;
        top = (((a[0 as libc::c_int as usize] * t + a[1 as libc::c_int as usize]) * t
            + a[2 as libc::c_int as usize])
            * t
            + a[3 as libc::c_int as usize])
            * t
            + a[4 as libc::c_int as usize]
            + 1.0f64;
        bot = ((b[0 as libc::c_int as usize] * t + b[1 as libc::c_int as usize]) * t
            + b[2 as libc::c_int as usize])
            * t
            + 1.0f64;
        return x * (top / bot);
    }
    if ax <= 4.0f64 {
        top = ((((((p[0 as libc::c_int as usize] * ax + p[1 as libc::c_int as usize]) * ax
            + p[2 as libc::c_int as usize])
            * ax
            + p[3 as libc::c_int as usize])
            * ax
            + p[4 as libc::c_int as usize])
            * ax
            + p[5 as libc::c_int as usize])
            * ax
            + p[6 as libc::c_int as usize])
            * ax
            + p[7 as libc::c_int as usize];
        bot = ((((((q[0 as libc::c_int as usize] * ax + q[1 as libc::c_int as usize]) * ax
            + q[2 as libc::c_int as usize])
            * ax
            + q[3 as libc::c_int as usize])
            * ax
            + q[4 as libc::c_int as usize])
            * ax
            + q[5 as libc::c_int as usize])
            * ax
            + q[6 as libc::c_int as usize])
            * ax
            + q[7 as libc::c_int as usize];
        let mut R: libc::c_double = 0.5f64 - exp(-x * x) * top / bot + 0.5f64;
        return if x < 0 as libc::c_int as libc::c_double {
            -R
        } else {
            R
        };
    }
    if ax >= 5.8f64 {
        return (if x > 0 as libc::c_int as libc::c_double {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as libc::c_double;
    }
    x2 = x * x;
    t = 1.0f64 / x2;
    top = (((r[0 as libc::c_int as usize] * t + r[1 as libc::c_int as usize]) * t
        + r[2 as libc::c_int as usize])
        * t
        + r[3 as libc::c_int as usize])
        * t
        + r[4 as libc::c_int as usize];
    bot = (((s[0 as libc::c_int as usize] * t + s[1 as libc::c_int as usize]) * t
        + s[2 as libc::c_int as usize])
        * t
        + s[3 as libc::c_int as usize])
        * t
        + 1.0f64;
    t = (c - top / (x2 * bot)) / ax;
    let mut R_0: libc::c_double = 0.5f64 - exp(-x2) * t + 0.5f64;
    return if x < 0 as libc::c_int as libc::c_double {
        -R_0
    } else {
        R_0
    };
}
unsafe extern "C" fn erfc1(mut ind: libc::c_int, mut x: libc::c_double) -> libc::c_double {
    static mut c: libc::c_double = 0.564189583547756f64;
    static mut a: [libc::c_double; 5] = [
        7.7105849500132e-5f64,
        -0.00133733772997339f64,
        0.0323076579225834f64,
        0.0479137145607681f64,
        0.128379167095513f64,
    ];
    static mut b: [libc::c_double; 3] = [
        0.00301048631703895f64,
        0.0538971687740286f64,
        0.375795757275549f64,
    ];
    static mut p: [libc::c_double; 8] = [
        -1.36864857382717e-7f64,
        0.564195517478974f64,
        7.21175825088309f64,
        43.1622272220567f64,
        152.98928504694f64,
        339.320816734344f64,
        451.918953711873f64,
        300.459261020162f64,
    ];
    static mut q: [libc::c_double; 8] = [
        1.0f64,
        12.7827273196294f64,
        77.0001529352295f64,
        277.585444743988f64,
        638.980264465631f64,
        931.35409485061f64,
        790.950925327898f64,
        300.459260956983f64,
    ];
    static mut r: [libc::c_double; 5] = [
        2.10144126479064f64,
        26.2370141675169f64,
        21.3688200555087f64,
        4.6580782871847f64,
        0.282094791773523f64,
    ];
    static mut s: [libc::c_double; 4] = [
        94.153775055546f64,
        187.11481179959f64,
        99.0191814623914f64,
        18.0124575948747f64,
    ];
    let mut ret_val: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut bot: libc::c_double = 0.;
    let mut top: libc::c_double = 0.;
    let mut ax: libc::c_double = fabs(x);
    if ax <= 0.5f64 {
        let mut t_0: libc::c_double = x * x;
        let mut top_0: libc::c_double =
            (((a[0 as libc::c_int as usize] * t_0 + a[1 as libc::c_int as usize]) * t_0
                + a[2 as libc::c_int as usize])
                * t_0
                + a[3 as libc::c_int as usize])
                * t_0
                + a[4 as libc::c_int as usize]
                + 1.0f64;
        let mut bot_0: libc::c_double =
            ((b[0 as libc::c_int as usize] * t_0 + b[1 as libc::c_int as usize]) * t_0
                + b[2 as libc::c_int as usize])
                * t_0
                + 1.0f64;
        ret_val = 0.5f64 - x * (top_0 / bot_0) + 0.5f64;
        if ind != 0 as libc::c_int {
            ret_val = exp(t_0) * ret_val;
        }
        return ret_val;
    }
    if ax <= 4.0f64 {
        top = ((((((p[0 as libc::c_int as usize] * ax + p[1 as libc::c_int as usize]) * ax
            + p[2 as libc::c_int as usize])
            * ax
            + p[3 as libc::c_int as usize])
            * ax
            + p[4 as libc::c_int as usize])
            * ax
            + p[5 as libc::c_int as usize])
            * ax
            + p[6 as libc::c_int as usize])
            * ax
            + p[7 as libc::c_int as usize];
        bot = ((((((q[0 as libc::c_int as usize] * ax + q[1 as libc::c_int as usize]) * ax
            + q[2 as libc::c_int as usize])
            * ax
            + q[3 as libc::c_int as usize])
            * ax
            + q[4 as libc::c_int as usize])
            * ax
            + q[5 as libc::c_int as usize])
            * ax
            + q[6 as libc::c_int as usize])
            * ax
            + q[7 as libc::c_int as usize];
        ret_val = top / bot;
    } else {
        if x <= -5.6f64 {
            ret_val = 2.0f64;
            if ind != 0 as libc::c_int {
                ret_val = exp(x * x) * 2.0f64;
            }
            return ret_val;
        }
        if ind == 0 as libc::c_int && (x > 100.0f64 || x * x > -exparg(1 as libc::c_int)) {
            return 0.0f64;
        }
        t = 1.0f64 / (x * x);
        top = (((r[0 as libc::c_int as usize] * t + r[1 as libc::c_int as usize]) * t
            + r[2 as libc::c_int as usize])
            * t
            + r[3 as libc::c_int as usize])
            * t
            + r[4 as libc::c_int as usize];
        bot = (((s[0 as libc::c_int as usize] * t + s[1 as libc::c_int as usize]) * t
            + s[2 as libc::c_int as usize])
            * t
            + s[3 as libc::c_int as usize])
            * t
            + 1.0f64;
        ret_val = (c - t * top / bot) / ax;
    }
    if ind != 0 as libc::c_int {
        if x < 0.0f64 {
            ret_val = exp(x * x) * 2.0f64 - ret_val;
        }
    } else {
        w = x * x;
        t = w;
        e = w - t;
        ret_val = (0.5f64 - e + 0.5f64) * exp(-t) * ret_val;
        if x < 0.0f64 {
            ret_val = 2.0f64 - ret_val;
        }
    }
    return ret_val;
}
unsafe extern "C" fn gam1(mut a: libc::c_double) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut bot: libc::c_double = 0.;
    let mut top: libc::c_double = 0.;
    t = a;
    d = a - 0.5f64;
    if d > 0.0f64 {
        t = d - 0.5f64;
    }
    if t < 0.0f64 {
        static mut r: [libc::c_double; 9] = [
            -0.422784335098468f64,
            -0.771330383816272f64,
            -0.244757765222226f64,
            0.118378989872749f64,
            9.30357293360349e-4f64,
            -0.0118290993445146f64,
            0.00223047661158249f64,
            2.66505979058923e-4f64,
            -1.32674909766242e-4f64,
        ];
        static mut s1: libc::c_double = 0.273076135303957f64;
        static mut s2: libc::c_double = 0.0559398236957378f64;
        top = (((((((r[8 as libc::c_int as usize] * t + r[7 as libc::c_int as usize]) * t
            + r[6 as libc::c_int as usize])
            * t
            + r[5 as libc::c_int as usize])
            * t
            + r[4 as libc::c_int as usize])
            * t
            + r[3 as libc::c_int as usize])
            * t
            + r[2 as libc::c_int as usize])
            * t
            + r[1 as libc::c_int as usize])
            * t
            + r[0 as libc::c_int as usize];
        bot = (s2 * t + s1) * t + 1.0f64;
        w = top / bot;
        if d > 0.0f64 {
            return t * w / a;
        } else {
            return a * (w + 0.5f64 + 0.5f64);
        }
    } else if t == 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    } else {
        static mut p: [libc::c_double; 7] = [
            0.577215664901533f64,
            -0.409078193005776f64,
            -0.230975380857675f64,
            0.0597275330452234f64,
            0.0076696818164949f64,
            -0.00514889771323592f64,
            5.89597428611429e-4f64,
        ];
        static mut q: [libc::c_double; 5] = [
            1.0f64,
            0.427569613095214f64,
            0.158451672430138f64,
            0.0261132021441447f64,
            0.00423244297896961f64,
        ];
        top = (((((p[6 as libc::c_int as usize] * t + p[5 as libc::c_int as usize]) * t
            + p[4 as libc::c_int as usize])
            * t
            + p[3 as libc::c_int as usize])
            * t
            + p[2 as libc::c_int as usize])
            * t
            + p[1 as libc::c_int as usize])
            * t
            + p[0 as libc::c_int as usize];
        bot = (((q[4 as libc::c_int as usize] * t + q[3 as libc::c_int as usize]) * t
            + q[2 as libc::c_int as usize])
            * t
            + q[1 as libc::c_int as usize])
            * t
            + 1.0f64;
        w = top / bot;
        if d > 0.0f64 {
            return t / a * (w - 0.5f64 - 0.5f64);
        } else {
            return a * w;
        }
    };
}
unsafe extern "C" fn gamln1(mut a: libc::c_double) -> libc::c_double {
    let mut w: libc::c_double = 0.;
    if a < 0.6f64 {
        static mut p0: libc::c_double = 0.577215664901533f64;
        static mut p1: libc::c_double = 0.844203922187225f64;
        static mut p2: libc::c_double = -0.168860593646662f64;
        static mut p3: libc::c_double = -0.780427615533591f64;
        static mut p4: libc::c_double = -0.402055799310489f64;
        static mut p5: libc::c_double = -0.0673562214325671f64;
        static mut p6: libc::c_double = -0.00271935708322958f64;
        static mut q1: libc::c_double = 2.88743195473681f64;
        static mut q2: libc::c_double = 3.12755088914843f64;
        static mut q3: libc::c_double = 1.56875193295039f64;
        static mut q4: libc::c_double = 0.361951990101499f64;
        static mut q5: libc::c_double = 0.0325038868253937f64;
        static mut q6: libc::c_double = 6.67465618796164e-4f64;
        w = ((((((p6 * a + p5) * a + p4) * a + p3) * a + p2) * a + p1) * a + p0)
            / ((((((q6 * a + q5) * a + q4) * a + q3) * a + q2) * a + q1) * a + 1.0f64);
        return -a * w;
    } else {
        static mut r0: libc::c_double = 0.422784335098467f64;
        static mut r1: libc::c_double = 0.848044614534529f64;
        static mut r2: libc::c_double = 0.565221050691933f64;
        static mut r3: libc::c_double = 0.156513060486551f64;
        static mut r4: libc::c_double = 0.017050248402265f64;
        static mut r5: libc::c_double = 4.97958207639485e-4f64;
        static mut s1: libc::c_double = 1.24313399877507f64;
        static mut s2: libc::c_double = 0.548042109832463f64;
        static mut s3: libc::c_double = 0.10155218743983f64;
        static mut s4: libc::c_double = 0.00713309612391f64;
        static mut s5: libc::c_double = 1.16165475989616e-4f64;
        let mut x: libc::c_double = a - 0.5f64 - 0.5f64;
        w = (((((r5 * x + r4) * x + r3) * x + r2) * x + r1) * x + r0)
            / (((((s5 * x + s4) * x + s3) * x + s2) * x + s1) * x + 1.0f64);
        return x * w;
    };
}
unsafe extern "C" fn psi(mut x: libc::c_double) -> libc::c_double {
    let mut current_block: u64;
    static mut piov4: libc::c_double = 0.785398163397448f64;
    static mut dx0: libc::c_double = 1.461632144968362341262659542325721325f64;
    static mut p1: [libc::c_double; 7] = [
        0.0089538502298197f64,
        4.77762828042627f64,
        142.441585084029f64,
        1186.45200713425f64,
        3633.51846806499f64,
        4138.10161269013f64,
        1305.60269827897f64,
    ];
    static mut q1: [libc::c_double; 6] = [
        44.8452573429826f64,
        520.752771467162f64,
        2210.0079924783f64,
        3641.27349079381f64,
        1908.310765963f64,
        6.91091682714533e-6f64,
    ];
    static mut p2: [libc::c_double; 4] = [
        -2.12940445131011f64,
        -7.01677227766759f64,
        -4.48616543918019f64,
        -0.648157123766197f64,
    ];
    static mut q2: [libc::c_double; 4] = [
        32.2703493791143f64,
        89.2920700481861f64,
        54.6117738103215f64,
        7.77788548522962f64,
    ];
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nq: libc::c_int = 0;
    let mut d2: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut den: libc::c_double = 0.;
    let mut aug: libc::c_double = 0.;
    let mut sgn: libc::c_double = 0.;
    let mut xmx0: libc::c_double = 0.;
    let mut xmax1: libc::c_double = 0.;
    let mut upper: libc::c_double = 0.;
    let mut xsmall: libc::c_double = 0.;
    xmax1 = 2147483647 as libc::c_int as libc::c_double;
    d2 = 0.5f64 / Rf_d1mach(3 as libc::c_int);
    if xmax1 > d2 {
        xmax1 = d2;
    }
    xsmall = 1e-9f64;
    aug = 0.0f64;
    if x < 0.5f64 {
        if fabs(x) <= xsmall {
            if x == 0.0f64 {
                current_block = 16813246331342052853;
            } else {
                aug = -1.0f64 / x;
                current_block = 14434620278749266018;
            }
        } else {
            w = -x;
            sgn = piov4;
            if w <= 0.0f64 {
                w = -w;
                sgn = -sgn;
            }
            if w >= xmax1 {
                current_block = 16813246331342052853;
            } else {
                nq = w as libc::c_int;
                w -= nq as libc::c_double;
                nq = (w * 4.0f64) as libc::c_int;
                w = (w - nq as libc::c_double * 0.25f64) * 4.0f64;
                n = nq / 2 as libc::c_int;
                if n + n != nq {
                    w = 1.0f64 - w;
                }
                z = piov4 * w;
                m = n / 2 as libc::c_int;
                if m + m != n {
                    sgn = -sgn;
                }
                n = (nq + 1 as libc::c_int) / 2 as libc::c_int;
                m = n / 2 as libc::c_int;
                m += m;
                if m == n {
                    if z == 0.0f64 {
                        current_block = 16813246331342052853;
                    } else {
                        aug = sgn * (cos(z) / sin(z) * 4.0f64);
                        current_block = 14434620278749266018;
                    }
                } else {
                    aug = sgn * (sin(z) / cos(z) * 4.0f64);
                    current_block = 14434620278749266018;
                }
            }
        }
        match current_block {
            16813246331342052853 => return 0.0f64,
            _ => {
                x = 1.0f64 - x;
            }
        }
    }
    if x <= 3.0f64 {
        den = x;
        upper = p1[0 as libc::c_int as usize] * x;
        i = 1 as libc::c_int;
        while i <= 5 as libc::c_int {
            den = (den + q1[(i - 1 as libc::c_int) as usize]) * x;
            upper = (upper + p1[i as usize]) * x;
            i += 1;
        }
        den = (upper + p1[6 as libc::c_int as usize]) / (den + q1[5 as libc::c_int as usize]);
        xmx0 = x - dx0;
        return den * xmx0 + aug;
    }
    if x < xmax1 {
        w = 1.0f64 / (x * x);
        den = w;
        upper = p2[0 as libc::c_int as usize] * w;
        i = 1 as libc::c_int;
        while i <= 3 as libc::c_int {
            den = (den + q2[(i - 1 as libc::c_int) as usize]) * w;
            upper = (upper + p2[i as usize]) * w;
            i += 1;
        }
        aug = upper / (den + q2[3 as libc::c_int as usize]) - 0.5f64 / x + aug;
    }
    return aug + log(x);
}
unsafe extern "C" fn betaln(mut a0: libc::c_double, mut b0: libc::c_double) -> libc::c_double {
    static mut e: libc::c_double = 0.918938533204673f64;
    let mut a: libc::c_double = if a0 < b0 { a0 } else { b0 };
    let mut b: libc::c_double = if a0 > b0 { a0 } else { b0 };
    if a < 8.0f64 {
        let mut n: libc::c_int = 0;
        if a < 1.0f64 {
            if b < 8.0f64 {
                return gamln(a) + (gamln(b) - gamln(a + b));
            } else {
                return gamln(a) + algdiv(a, b);
            }
        }
        let mut w: libc::c_double = 0.;
        if a < 2.0f64 {
            if b <= 2.0f64 {
                return gamln(a) + gamln(b) - gsumln(a, b);
            }
            if b < 8.0f64 {
                w = 0.0f64;
            } else {
                return gamln(a) + algdiv(a, b);
            }
        } else if b <= 1e3f64 {
            n = (a - 1.0f64) as libc::c_int;
            w = 1.0f64;
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= n {
                a += -1.0f64;
                let mut h: libc::c_double = a / b;
                w *= h / (h + 1.0f64);
                i += 1;
            }
            w = log(w);
            if b >= 8.0f64 {
                return w + gamln(a) + algdiv(a, b);
            }
        } else {
            let mut n_0: libc::c_int = (a - 1.0f64) as libc::c_int;
            w = 1.0f64;
            let mut i_1: libc::c_int = 1 as libc::c_int;
            while i_1 <= n_0 {
                a += -1.0f64;
                w *= a / (a / b + 1.0f64);
                i_1 += 1;
            }
            return log(w) - n_0 as libc::c_double * log(b) + (gamln(a) + algdiv(a, b));
        }
        n = (b - 1.0f64) as libc::c_int;
        let mut z: libc::c_double = 1.0f64;
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 <= n {
            b += -1.0f64;
            z *= b / (a + b);
            i_0 += 1;
        }
        return w + log(z) + (gamln(a) + (gamln(b) - gsumln(a, b)));
    } else {
        let mut w_0: libc::c_double = bcorr(a, b);
        let mut h_0: libc::c_double = a / b;
        let mut u: libc::c_double = -(a - 0.5f64) * log(h_0 / (h_0 + 1.0f64));
        let mut v: libc::c_double = b * alnrel(h_0);
        if u > v {
            return log(b) * -0.5f64 + e + w_0 - v - u;
        } else {
            return log(b) * -0.5f64 + e + w_0 - u - v;
        }
    };
}
unsafe extern "C" fn gsumln(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    let mut x: libc::c_double = a + b - 2.0f64;
    if x <= 0.25f64 {
        return gamln1(x + 1.0f64);
    }
    if x <= 1.25f64 {
        return gamln1(x) + alnrel(x);
    }
    return gamln1(x - 1.0f64) + log(x * (x + 1.0f64));
}
unsafe extern "C" fn bcorr(mut a0: libc::c_double, mut b0: libc::c_double) -> libc::c_double {
    static mut c0: libc::c_double = 0.0833333333333333f64;
    static mut c1: libc::c_double = -0.00277777777760991f64;
    static mut c2: libc::c_double = 7.9365066682539e-4f64;
    static mut c3: libc::c_double = -5.9520293135187e-4f64;
    static mut c4: libc::c_double = 8.37308034031215e-4f64;
    static mut c5: libc::c_double = -0.00165322962780713f64;
    let mut ret_val: libc::c_double = 0.;
    let mut r1: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut s3: libc::c_double = 0.;
    let mut s5: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut s7: libc::c_double = 0.;
    let mut s9: libc::c_double = 0.;
    let mut s11: libc::c_double = 0.;
    a = if a0 < b0 { a0 } else { b0 };
    b = if a0 > b0 { a0 } else { b0 };
    h = a / b;
    c = h / (h + 1.0f64);
    x = 1.0f64 / (h + 1.0f64);
    x2 = x * x;
    s3 = x + x2 + 1.0f64;
    s5 = x + x2 * s3 + 1.0f64;
    s7 = x + x2 * s5 + 1.0f64;
    s9 = x + x2 * s7 + 1.0f64;
    s11 = x + x2 * s9 + 1.0f64;
    r1 = 1.0f64 / b;
    t = r1 * r1;
    w = ((((c5 * s11 * t + c4 * s9) * t + c3 * s7) * t + c2 * s5) * t + c1 * s3) * t + c0;
    w *= c / b;
    r1 = 1.0f64 / a;
    t = r1 * r1;
    ret_val = (((((c5 * t + c4) * t + c3) * t + c2) * t + c1) * t + c0) / a + w;
    return ret_val;
}
unsafe extern "C" fn algdiv(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    static mut c0: libc::c_double = 0.0833333333333333f64;
    static mut c1: libc::c_double = -0.00277777777760991f64;
    static mut c2: libc::c_double = 7.9365066682539e-4f64;
    static mut c3: libc::c_double = -5.9520293135187e-4f64;
    static mut c4: libc::c_double = 8.37308034031215e-4f64;
    static mut c5: libc::c_double = -0.00165322962780713f64;
    let mut c: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut s3: libc::c_double = 0.;
    let mut s5: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut s7: libc::c_double = 0.;
    let mut s9: libc::c_double = 0.;
    let mut s11: libc::c_double = 0.;
    if a > b {
        h = b / a;
        c = 1.0f64 / (h + 1.0f64);
        x = h / (h + 1.0f64);
        d = a + (b - 0.5f64);
    } else {
        h = a / b;
        c = h / (h + 1.0f64);
        x = 1.0f64 / (h + 1.0f64);
        d = b + (a - 0.5f64);
    }
    x2 = x * x;
    s3 = x + x2 + 1.0f64;
    s5 = x + x2 * s3 + 1.0f64;
    s7 = x + x2 * s5 + 1.0f64;
    s9 = x + x2 * s7 + 1.0f64;
    s11 = x + x2 * s9 + 1.0f64;
    t = 1.0f64 / (b * b);
    w = ((((c5 * s11 * t + c4 * s9) * t + c3 * s7) * t + c2 * s5) * t + c1 * s3) * t + c0;
    w *= c / b;
    u = d * alnrel(a / b);
    v = a * (log(b) - 1.0f64);
    if u > v {
        return w - v - u;
    } else {
        return w - u - v;
    };
}
unsafe extern "C" fn gamln(mut a: libc::c_double) -> libc::c_double {
    static mut d: libc::c_double = 0.418938533204673f64;
    static mut c0: libc::c_double = 0.0833333333333333f64;
    static mut c1: libc::c_double = -0.00277777777760991f64;
    static mut c2: libc::c_double = 7.9365066682539e-4f64;
    static mut c3: libc::c_double = -5.9520293135187e-4f64;
    static mut c4: libc::c_double = 8.37308034031215e-4f64;
    static mut c5: libc::c_double = -0.00165322962780713f64;
    if a <= 0.8f64 {
        return gamln1(a) - log(a);
    } else if a <= 2.25f64 {
        return gamln1(a - 0.5f64 - 0.5f64);
    } else if a < 10.0f64 {
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = (a - 1.25f64) as libc::c_int;
        let mut t: libc::c_double = a;
        let mut w: libc::c_double = 1.0f64;
        i = 1 as libc::c_int;
        while i <= n {
            t += -1.0f64;
            w *= t;
            i += 1;
        }
        return gamln1(t - 1.0f64) + log(w);
    } else {
        let mut t_0: libc::c_double = 1.0f64 / (a * a);
        let mut w_0: libc::c_double =
            (((((c5 * t_0 + c4) * t_0 + c3) * t_0 + c2) * t_0 + c1) * t_0 + c0) / a;
        return d + w_0 + (a - 0.5f64) * (log(a) - 1.0f64);
    };
}
