#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::{d1mach::rf_d1mach, i1mach::rf_i1mach, pgamma::logspace_add};

pub fn rf_bratio(
    mut a: f64,
    mut b: f64,
    mut x: f64,
    mut y: f64,
    w: &mut f64,
    w1: &mut f64,
    ierr: &mut i32,
    mut log_p: bool,
) {
    let mut current_block: u64;
    let mut do_swap: bool = false;
    let mut n: libc::c_int = 0;
    let mut ierr1: libc::c_int = 0 as libc::c_int;
    let mut z: f64 = 0.;
    let mut a0: f64 = 0.;
    let mut b0: f64 = 0.;
    let mut x0: f64 = 0.;
    let mut y0: f64 = 0.;
    let mut lambda: f64 = 0.;
    let mut eps: f64 = 2.0 * rf_d1mach(3);

    *w = if log_p { -1.0 / 0.0 } else { 0.0 };
    *w1 = if log_p { -1.0 / 0.0 } else { 0.0 };
    if x.is_nan() || y.is_nan() || a.is_nan() || b.is_nan() {
        *ierr = 9;
        return;
    }
    if a < 0.0 || b < 0.0 {
        *ierr = 1;
        return;
    }
    if a == 0.0 && b == 0.0 {
        *ierr = 2;
        return;
    }
    if x < 0.0 || x > 1.0 {
        *ierr = 3;
        return;
    }
    if y < 0.0 || y > 1.0 {
        *ierr = 4;
        return;
    }
    z = x + y - 0.5 - 0.5;
    if z.abs() > eps * 3.0 {
        *ierr = 5;
        return;
    }
    *ierr = 0;
    if x == 0.0 {
        if a == 0.0 {
            *ierr = 6;
            return;
        }
    } else {
        if y == 0.0 {
            if b == 0.0 {
                *ierr = 7;
                return;
            }
            current_block = 17478089311782157434;
        } else if a == 0.0 {
            current_block = 17478089311782157434;
        } else if b == 0.0 {
            current_block = 12410001467641165671;
        } else {
            eps = eps.max(1e-15);
            if a.max(b) < eps * 0.001 {
                (*w, *w1) = if log_p {
                    if a < b {
                        ((-a / (a + b)).ln_1p(), (a / (a + b)).ln())
                    } else {
                        ((b / (a + b)).ln(), (-b / (a + b)).ln_1p())
                    }
                } else {
                    (b / (a + b), a / (a + b))
                };
                return;
            }
            if a.min(b) <= 1.0 {
                if x > 0.5 {
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
                    *w1 = if log_p {
                        if *w > -0.693147180559945309417232121458 {
                            (-(*w).exp_m1()).ln()
                        } else {
                            (-(*w).exp()).ln_1p()
                        }
                    } else {
                        0.5 - *w + 0.5
                    };
                    current_block = 7496337703584048604;
                } else {
                    if a0 < (if eps < eps * b0 { eps } else { eps * b0 }) && b0 * x0 <= 1.0 {
                        *w1 = apser(a0, b0, x0, eps);
                        current_block = 9167786483348357766;
                    } else {
                        let mut did_bup = false;
                        if (if a0 > b0 { a0 } else { b0 }) > 1.0 {
                            if b0 <= 1.0 {
                                current_block = 16942492502630583769;
                            } else if x0 >= 0.29 {
                                current_block = 2336920824898283577;
                            } else if x0 < 0.1 && (x0 * b0).powf(a0) <= 0.7 {
                                current_block = 16942492502630583769;
                            } else if b0 > 15.0 {
                                *w1 = 0.0;
                                current_block = 1868291631715963762;
                            } else {
                                current_block = 307447392441238883;
                            }
                        } else if a0 >= (if 0.2 < b0 { 0.2 } else { b0 }) {
                            current_block = 16942492502630583769;
                        } else if x0.powf(a0) <= 0.9 {
                            current_block = 16942492502630583769;
                        } else if x0 >= 0.3 {
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
                                        *w1 = bup(b0, a0, y0, x0, n, eps, false);
                                        did_bup = true;
                                        b0 += n as f64;
                                        current_block = 1868291631715963762;
                                    }
                                    2336920824898283577 => {
                                        *w1 = bpser(b0, a0, y0, eps, log_p);
                                        *w = if log_p {
                                            if *w1 > -0.693147180559945309417232121458 {
                                                (-(*w1).exp_m1()).ln()
                                            } else {
                                                (-(*w1).exp()).ln_1p()
                                            }
                                        } else {
                                            0.5 - *w1 + 0.5
                                        };
                                        current_block = 7496337703584048604;
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    7496337703584048604 => {}
                                    _ => {
                                        bgrat(b0, a0, y0, x0, w1, 15. * eps, &mut ierr1, false);
                                        if *w1 == 0 as libc::c_int as f64
                                            || (0 as libc::c_int as f64) < *w1
                                                && *w1 < 2.2250738585072014e-308
                                        {
                                            if did_bup {
                                                *w1 = bup(b0 - n as f64, a0, y0, x0, n, eps, true);
                                            } else {
                                                *w1 = -1.0 / 0.0;
                                            }
                                            bgrat(
                                                b0,
                                                a0,
                                                y0,
                                                x0,
                                                w1,
                                                15 as libc::c_int as f64 * eps,
                                                &mut ierr1,
                                                true,
                                            );
                                            if ierr1 != 0 {
                                                return;
                                            }
                                            if log_p {
                                                *w = if *w1 > -0.693147180559945309417232121458 {
                                                    (-(*w1).exp_m1()).ln()
                                                } else {
                                                    (-(*w1).exp()).ln_1p()
                                                };
                                            } else {
                                                *w = -(*w1).exp_m1();
                                                *w1 = (*w1).exp();
                                            }
                                            current_block = 7496337703584048604;
                                        } else {
                                            if ierr1 != 0 {
                                                *ierr = 10 + ierr1;
                                                return;
                                            }
                                            if *w1 < 0. {
                                                println!(
                                                    "bratio(a={}, b={}, x={}): bgrat() -> w1 = {}",
                                                    a, b, x, *w1,
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
                            if log_p {
                                *w = (-*w1).ln_1p();
                                *w1 = (*w1).ln();
                            } else {
                                *w = 0.5 - *w1 + 0.5;
                            }
                            current_block = 7496337703584048604;
                        }
                    }
                }
            } else {
                lambda = if (a + b).is_finite() {
                    if a > b {
                        (a + b) * y - b
                    } else {
                        a - (a + b) * x
                    }
                } else {
                    a * y - b * x
                };
                if lambda < 0. {
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
                if b0 < 40.0 {
                    if b0 * x0 <= 0.7 || log_p && lambda > 650.0 {
                        current_block = 16942492502630583769;
                    } else {
                        n = b0 as libc::c_int;
                        b0 -= n as f64;
                        if b0 == 0.0 {
                            n -= 1;
                            b0 = 1.0;
                        }
                        *w = bup(b0, a0, y0, x0, n, eps, false);
                        if *w < 2.2250738585072014e-308 && log_p {
                            b0 += n as f64;
                            current_block = 16942492502630583769;
                        } else {
                            if x0 <= 0.7 {
                                *w += bpser(a0, b0, x0, eps, false);
                            } else {
                                if a0 <= 15.0 {
                                    n = 20 as libc::c_int;
                                    *w += bup(a0, b0, x0, y0, n, eps, false);
                                    a0 += n as f64;
                                }
                                bgrat(a0, b0, x0, y0, w, 15. * eps, &mut ierr1, false);
                                if ierr1 != 0 {
                                    *ierr = 10 + ierr1;
                                    return;
                                }
                            }
                            if log_p {
                                *w1 = (-*w).ln_1p();
                                *w = (*w).ln();
                            } else {
                                *w1 = 0.5 - *w + 0.5;
                            }
                            current_block = 7496337703584048604;
                        }
                    }
                } else {
                    if a0 > b0 {
                        if b0 <= 100.0 || lambda > b0 * 0.03 {
                            current_block = 17597882877141686238;
                        } else {
                            current_block = 15447629348493591490;
                        }
                    } else if a0 <= 100.0 {
                        current_block = 17597882877141686238;
                    } else if lambda > a0 * 0.03 {
                        current_block = 17597882877141686238;
                    } else {
                        current_block = 15447629348493591490;
                    }
                    match current_block {
                        17597882877141686238 => {
                            *w = bfrac(a0, b0, x0, y0, lambda, eps * 15.0, log_p);
                            *w1 = if log_p {
                                if *w > -0.693147180559945309417232121458 {
                                    (-(*w).exp_m1()).ln()
                                } else {
                                    (-(*w).exp()).ln_1p()
                                }
                            } else {
                                0.5 - *w + 0.5
                            };
                        }
                        _ => {
                            *w = basym(a0, b0, lambda, eps * 100.0, log_p);
                            *w1 = if log_p {
                                if *w > -0.693147180559945309417232121458 {
                                    (-(*w).exp_m1()).ln()
                                } else {
                                    (-(*w).exp()).ln_1p()
                                }
                            } else {
                                0.5 - *w + 0.5
                            };
                        }
                    }
                    current_block = 7496337703584048604;
                }
            }
            match current_block {
                16942492502630583769 => {
                    *w = bpser(a0, b0, x0, eps, log_p);
                    *w1 = if log_p {
                        if *w > -0.693147180559945309417232121458 {
                            (-(*w).exp_m1()).ln()
                        } else {
                            (-(*w).exp()).ln_1p()
                        }
                    } else {
                        0.5 - *w + 0.5
                    };
                }
                _ => {}
            }
            if do_swap as u64 != 0 {
                let mut t: f64 = *w;
                *w = *w1;
                *w1 = t;
            }
            return;
        }
        match current_block {
            12410001467641165671 => {}
            _ => {
                *w = if log_p { 0.0 } else { 1.0 };
                *w1 = if log_p { f64::NEG_INFINITY } else { 0.0 };
                return;
            }
        }
    }
    *w = if log_p { f64::NEG_INFINITY } else { 0.0 };
    *w1 = if log_p { 0.0 } else { 1.0 };
    return;
}

fn fpser(mut a: f64, mut b: f64, mut x: f64, mut eps: f64, mut log_p: bool) -> f64 {
    let mut ans: f64 = 0.;
    let mut c: f64 = 0.;
    let mut s: f64 = 0.;
    let mut t: f64 = 0.;
    let mut an: f64 = 0.;
    let mut tol: f64 = 0.;
    if log_p {
        ans = a * x.ln();
    } else if a > eps * 0.001 {
        t = a * x.ln();
        if t < exparg(1 as libc::c_int) {
            return 0.0;
        }
        ans = t.exp();
    } else {
        ans = 1.0;
    }
    if log_p {
        ans += b.ln() - a.ln();
    } else {
        ans *= b / a;
    }
    tol = eps / a;
    an = a + 1.0;
    t = x;
    s = t / an;
    loop {
        an += 1.0;
        t = x * t;
        c = t / an;
        s += c;
        if !(c.abs() > tol) {
            break;
        }
    }
    if log_p {
        ans += (a * s).ln_1p();
    } else {
        ans *= a * s + 1.0;
    }
    return ans;
}
fn apser(mut a: f64, mut b: f64, mut x: f64, mut eps: f64) -> f64 {
    const g: f64 = 0.577215664901533;
    let mut tol: f64 = 0.;
    let mut c: f64 = 0.;
    let mut j: f64 = 0.;
    let mut s: f64 = 0.;
    let mut t: f64 = 0.;
    let mut aj: f64 = 0.;
    let mut bx: f64 = b * x;
    t = x - bx;
    if b * eps <= 0.02 {
        c = x.ln() + psi(b) + g + t;
    } else {
        c = bx.ln() + g + t;
    }
    tol = eps * 5.0 * c.abs();
    j = 1.0;
    s = 0.0;
    loop {
        j += 1.0;
        t *= x - bx / j;
        aj = t / j;
        s += aj;
        if !(aj.abs() > tol) {
            break;
        }
    }
    return -a * (c + s);
}

fn bpser(mut a: f64, mut b: f64, mut x: f64, mut eps: f64, mut log_p: bool) -> f64 {
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut ans: f64 = 0.;
    let mut c: f64 = 0.;
    let mut t: f64 = 0.;
    let mut u: f64 = 0.;
    let mut z: f64 = 0.;
    let mut a0: f64 = 0.;
    let mut b0: f64 = 0.;
    let mut apb: f64 = 0.;
    if x == 0.0 {
        return if log_p { f64::NEG_INFINITY } else { 0.0 };
    }
    a0 = if a < b { a } else { b };
    if a0 >= 1.0 {
        z = a * x.ln() - betaln(a, b);
        ans = if log_p { z - a.ln() } else { z.exp() / a };
    } else {
        b0 = a.max(b);
        if b0 < 8.0 {
            if b0 <= 1.0 {
                if log_p {
                    ans = a * x.ln();
                } else {
                    ans = x.powf(a);
                    if ans == 0.0 {
                        return ans;
                    }
                }
                apb = a + b;
                if apb > 1.0 {
                    u = a + b - 1.0;
                    z = (gam1(u) + 1.0) / apb;
                } else {
                    z = gam1(apb) + 1.0;
                }
                c = (gam1(a) + 1.0) * (gam1(b) + 1.0) / z;
                if log_p {
                    ans += (c * (b / apb)).ln();
                } else {
                    ans *= c * (b / apb);
                }
            } else {
                u = gamln1(a0);
                m = (b0 - 1.0) as libc::c_int;
                if m >= 1 as libc::c_int {
                    c = 1.0;
                    i = 1 as libc::c_int;
                    while i <= m {
                        b0 += -1.0;
                        c *= b0 / (a0 + b0);
                        i += 1;
                    }
                    u += c.ln();
                }
                z = a * x.ln() - u;
                b0 += -1.0;
                apb = a0 + b0;
                if apb > 1.0 {
                    u = a0 + b0 - 1.0;
                    t = (gam1(u) + 1.0) / apb;
                } else {
                    t = gam1(apb) + 1.0;
                }
                if log_p {
                    ans = z + (a0 / a).ln() + gam1(b0).ln_1p() - t.ln();
                } else {
                    ans = z.exp() * (a0 / a) * (gam1(b0) + 1.0) / t;
                }
            }
        } else {
            u = gamln1(a0) + algdiv(a0, b0);
            z = a * x.ln() - u;
            if log_p {
                ans = z + (a0 / a).ln();
            } else {
                ans = a0 / a * z.exp();
            }
        }
    }
    if ans == (if log_p { f64::NEG_INFINITY } else { 0.0 }) || !log_p && a <= eps * 0.1 {
        return ans;
    }
    let mut tol: f64 = eps / a;
    let mut n: f64 = 0.0;
    let mut sum: f64 = 0.0;
    let mut w: f64 = 0.;
    c = 1.0;
    loop {
        n += 1.0;
        c *= (0.5 - b / n + 0.5) * x;
        w = c / (a + n);
        sum += w;
        if !(n < 1e7 && w.abs() > tol) {
            break;
        }
    }
    if w.abs() > tol {
        if log_p && !(a * sum > -1.0 && (a * sum).ln_1p().abs() < eps * ans.abs())
            || !log_p && (a * sum + 1.0).abs() != 1.0
        {
            println!(
                " bpser(a={}, b={}, x={},...) did not converge (n=1e7, |w|/tol={} > 1; A={})",
                a,
                b,
                x,
                w.abs() / tol,
                ans,
            );
        }
    }
    if log_p {
        if a * sum > -1.0 {
            ans += (a * sum).ln_1p();
        } else {
            if ans > f64::NEG_INFINITY {
                println!(
                    "pbeta(*, log.p=TRUE) -> bpser(a={}, b={}, x={},...) underflow to -Inf",
                    a, b, x,
                );
            }
            ans = f64::NEG_INFINITY;
        }
    } else if a * sum > -1.0 {
        ans *= a * sum + 1.0;
    } else {
        ans = 0.0;
    }
    return ans;
}

fn bup(
    mut a: f64,
    mut b: f64,
    mut x: f64,
    mut y: f64,
    mut n: libc::c_int,
    mut eps: f64,
    mut log_p: bool,
) -> f64 {
    let mut ret_val: f64 = 0.;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mu: libc::c_int = 0;
    let mut d: f64 = 0.;
    let mut l: f64 = 0.;
    let mut apb: f64 = a + b;
    let mut ap1: f64 = a + 1.0;
    if n > 1 as libc::c_int && a >= 1.0 && apb >= ap1 * 1.1 {
        mu = (exparg(1)).abs() as i32;
        k = exparg(0) as i32;
        if mu > k {
            mu = k;
        }
        d = (-(mu as f64)).exp();
    } else {
        mu = 0 as libc::c_int;
        d = 1.0;
    }
    ret_val = if log_p {
        brcmp1(mu, a, b, x, y, true) - a.ln()
    } else {
        brcmp1(mu, a, b, x, y, false) / a
    };
    if n == 1 as libc::c_int || log_p && ret_val == -1.0 / 0.0 || !log_p && ret_val == 0.0 {
        return ret_val;
    }
    let mut nm1: libc::c_int = n - 1 as libc::c_int;
    let mut w: f64 = d;
    k = 0 as libc::c_int;
    if b > 1.0 {
        if y > 1e-4 {
            let mut r: f64 = (b - 1.0) * x / y - a;
            if r >= 1.0 {
                k = if r < nm1 as f64 {
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
            l = i as f64;
            d *= (apb + l) / (ap1 + l) * x;
            w += d;
            i += 1;
        }
    }
    i = k;
    while i < nm1 {
        l = i as f64;
        d *= (apb + l) / (ap1 + l) * x;
        w += d;
        if d <= eps * w {
            break;
        }
        i += 1;
    }
    if log_p {
        ret_val += w.ln();
    } else {
        ret_val *= w;
    }
    return ret_val;
}

fn bfrac(
    mut a: f64,
    mut b: f64,
    mut x: f64,
    mut y: f64,
    mut lambda: f64,
    mut eps: f64,
    mut log_p: bool,
) -> f64 {
    let mut c: f64 = 0.;
    let mut e: f64 = 0.;
    let mut n: f64 = 0.;
    let mut p: f64 = 0.;
    let mut r: f64 = 0.;
    let mut s: f64 = 0.;
    let mut t: f64 = 0.;
    let mut w: f64 = 0.;
    let mut c0: f64 = 0.;
    let mut c1: f64 = 0.;
    let mut r0: f64 = 0.;
    let mut an: f64 = 0.;
    let mut bn: f64 = 0.;
    let mut yp1: f64 = 0.;
    let mut anp1: f64 = 0.;
    let mut bnp1: f64 = 0.;
    let mut beta: f64 = 0.;
    let mut alpha: f64 = 0.;
    let mut brc: f64 = 0.;
    if !lambda.is_finite() {
        return f64::NAN;
    }
    brc = brcomp(a, b, x, y, log_p);
    if brc.is_nan() {
        return f64::NAN;
    }
    if !log_p && brc == 0.0 {
        return 0.0;
    }
    c = lambda + 1.0;
    c0 = b / a;
    c1 = 1.0 / a + 1.0;
    yp1 = y + 1.0;
    n = 0.0;
    p = 1.0;
    s = a + 1.0;
    an = 0.0;
    bn = 1.0;
    anp1 = 1.0;
    bnp1 = c / c1;
    r = c1 / c;
    loop {
        n += 1.0;
        t = n / a;
        w = n * (b - n) * x;
        e = a / s;
        alpha = p * (p + c0) * e * e * (w * x);
        e = (t + 1.0) / (c1 + t + t);
        beta = n + w / s + e * (c + n * yp1);
        p = t + 1.0;
        s += 2.0;
        t = alpha * an + beta * anp1;
        an = anp1;
        anp1 = t;
        t = alpha * bn + beta * bnp1;
        bn = bnp1;
        bnp1 = t;
        r0 = r;
        r = anp1 / bnp1;
        if (r - r0).abs() <= eps * r {
            break;
        }
        an /= bnp1;
        bn /= bnp1;
        anp1 = r;
        bnp1 = 1.0;
        if !(n < 10000 as libc::c_int as f64) {
            break;
        }
    }
    if n >= 10000 as libc::c_int as f64 && (r - r0).abs() > eps * r {
        println!(
            " bfrac(a={}, b={}, x={}, y={}, lambda={}) did *not* converge (in 10000 steps)",
            a, b, x, y, lambda,
        );
    }
    return if log_p { brc + r.ln() } else { brc * r };
}

fn brcomp(mut a: f64, mut b: f64, mut x: f64, mut y: f64, mut log_p: bool) -> f64 {
    const const__: f64 = 0.398942280401433;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: f64 = 0.;
    let mut e: f64 = 0.;
    let mut u: f64 = 0.;
    let mut v: f64 = 0.;
    let mut z: f64 = 0.;
    let mut a0: f64 = 0.;
    let mut b0: f64 = 0.;
    let mut apb: f64 = 0.;
    if x == 0.0 || y == 0.0 {
        return if log_p { -1.0 / 0.0 } else { 0.0 };
    }
    a0 = if a < b { a } else { b };
    if a0 < 8.0 {
        let mut lnx: f64 = 0.;
        let mut lny: f64 = 0.;
        if x <= 0.375 {
            lnx = x.ln();
            lny = alnrel(-x);
        } else if y > 0.375 {
            lnx = x.ln();
            lny = y.ln();
        } else {
            lnx = alnrel(-y);
            lny = y.ln();
        }
        z = a * lnx + b * lny;
        if a0 >= 1.0 {
            z -= betaln(a, b);
            return if log_p { z } else { z.exp() };
        }
        b0 = if a > b { a } else { b };
        if b0 >= 8.0 {
            u = gamln1(a0) + algdiv(a0, b0);
            return if log_p {
                a0.ln() + (z - u)
            } else {
                a0 * (z - u).exp()
            };
        }
        if b0 <= 1.0 {
            let mut e_z: f64 = if log_p { z } else { z.exp() };
            if !log_p && e_z == 0.0 {
                return 0.0;
            }
            apb = a + b;
            if apb > 1.0 {
                u = a + b - 1.0;
                z = (gam1(u) + 1.0) / apb;
            } else {
                z = gam1(apb) + 1.0;
            }
            c = (gam1(a) + 1.0) * (gam1(b) + 1.0) / z;
            return if log_p {
                e_z + (a0 * c).ln() - (a0 / b0).ln_1p()
            } else {
                e_z * (a0 * c) / (a0 / b0 + 1.0)
            };
        }
        u = gamln1(a0);
        n = (b0 - 1.0) as libc::c_int;
        if n >= 1 as libc::c_int {
            c = 1.0;
            i = 1 as libc::c_int;
            while i <= n {
                b0 += -1.0;
                c *= b0 / (a0 + b0);
                i += 1;
            }
            u = c.ln() + u;
        }
        z -= u;
        b0 += -1.0;
        apb = a0 + b0;
        let mut t: f64 = 0.;
        if apb > 1.0 {
            u = a0 + b0 - 1.0;
            t = (gam1(u) + 1.0) / apb;
        } else {
            t = gam1(apb) + 1.0;
        }
        return if log_p {
            a0.ln() + z + (gam1(b0)).ln_1p() - t.ln()
        } else {
            a0 * z.exp() * (gam1(b0) + 1.0) / t
        };
    } else {
        let mut h: f64 = 0.;
        let mut x0: f64 = 0.;
        let mut y0: f64 = 0.;
        let mut lambda: f64 = 0.;
        if a <= b {
            h = a / b;
            x0 = h / (h + 1.0);
            y0 = 1.0 / (h + 1.0);
            lambda = a - (a + b) * x;
        } else {
            h = b / a;
            x0 = 1.0 / (h + 1.0);
            y0 = h / (h + 1.0);
            lambda = (a + b) * y - b;
        }
        e = -lambda / a;
        if e.abs() > 0.6 {
            u = e - (x / x0).ln();
        } else {
            u = rlog1(e);
        }
        e = lambda / b;
        if e.abs() <= 0.6 {
            v = rlog1(e);
        } else {
            v = e - (y / y0).ln();
        }
        z = if log_p {
            -(a * u + b * v)
        } else {
            (-(a * u + b * v)).exp()
        };
        return if log_p {
            -0.918938533204672741780329736406 + 0.5 * (b * x0).ln() + z - bcorr(a, b)
        } else {
            const__ * (b * x0).sqrt() * z * (-bcorr(a, b)).exp()
        };
    };
}
fn brcmp1(
    mut mu: libc::c_int,
    mut a: f64,
    mut b: f64,
    mut x: f64,
    mut y: f64,
    mut log_p: bool,
) -> f64 {
    const const__: f64 = 0.398942280401433;
    let mut c: f64 = 0.;
    let mut t: f64 = 0.;
    let mut u: f64 = 0.;
    let mut v: f64 = 0.;
    let mut z: f64 = 0.;
    let mut a0: f64 = 0.;
    let mut b0: f64 = 0.;
    let mut apb: f64 = 0.;
    a0 = if a < b { a } else { b };
    if a0 < 8.0 {
        let mut lnx: f64 = 0.;
        let mut lny: f64 = 0.;
        if x <= 0.375 {
            lnx = x.ln();
            lny = alnrel(-x);
        } else if y > 0.375 {
            lnx = x.ln();
            lny = y.ln();
        } else {
            lnx = alnrel(-y);
            lny = y.ln();
        }
        z = a * lnx + b * lny;
        if a0 >= 1.0 {
            z -= betaln(a, b);
            return esum(mu, z, log_p);
        }
        b0 = if a > b { a } else { b };
        if b0 >= 8.0 {
            u = gamln1(a0) + algdiv(a0, b0);
            return if log_p {
                a0.ln() + esum(mu, z - u, true)
            } else {
                a0 * esum(mu, z - u, false)
            };
        } else {
            if b0 <= 1.0 {
                let mut ans: f64 = esum(mu, z, log_p);
                if ans == (if log_p { -1.0 / 0.0 } else { 0.0 }) {
                    return ans;
                }
                apb = a + b;
                if apb > 1.0 {
                    u = a + b - 1.0;
                    z = (gam1(u) + 1.0) / apb;
                } else {
                    z = gam1(apb) + 1.0;
                }
                c = if log_p {
                    gam1(a).ln_1p() + gam1(b).ln_1p() - z.ln()
                } else {
                    (gam1(a) + 1.0) * (gam1(b) + 1.0) / z
                };
                return if log_p {
                    ans + a0.ln() + c - (a0 / b0).ln_1p()
                } else {
                    ans * (a0 * c) / (a0 / b0 + 1.0)
                };
            }
        }
        u = gamln1(a0);
        let mut n: libc::c_int = (b0 - 1.0) as libc::c_int;
        if n >= 1 as libc::c_int {
            c = 1.0;
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= n {
                b0 += -1.0;
                c *= b0 / (a0 + b0);
                i += 1;
            }
            u += c.ln();
        }
        z -= u;
        b0 += -1.0;
        apb = a0 + b0;
        if apb > 1.0 {
            t = (gam1(apb - 1.0) + 1.0) / apb;
        } else {
            t = gam1(apb) + 1.0;
        }
        return if log_p {
            a0.ln() + esum(mu, z, true) + gam1(b0).ln_1p() - t.ln()
        } else {
            a0 * esum(mu, z, false) * (gam1(b0) + 1.0) / t
        };
    } else {
        let mut h: f64 = 0.;
        let mut x0: f64 = 0.;
        let mut y0: f64 = 0.;
        let mut lambda: f64 = 0.;
        if a > b {
            h = b / a;
            x0 = 1.0 / (h + 1.0);
            y0 = h / (h + 1.0);
            lambda = (a + b) * y - b;
        } else {
            h = a / b;
            x0 = h / (h + 1.0);
            y0 = 1.0 / (h + 1.0);
            lambda = a - (a + b) * x;
        }
        let mut lx0: f64 = -(b / a).ln_1p();
        let mut e: f64 = -lambda / a;
        if e.abs() > 0.6 {
            u = e - (x / x0).ln();
        } else {
            u = rlog1(e);
        }
        e = lambda / b;
        if e.abs() > 0.6 {
            v = e - (y / y0).ln();
        } else {
            v = rlog1(e);
        }
        z = esum(mu, -(a * u + b * v), log_p);
        return if log_p {
            (const__).ln() + (b.ln() + lx0) / 2.0 + z - bcorr(a, b)
        } else {
            const__ * (b * x0).sqrt() * z * (-bcorr(a, b)).exp()
        };
    };
}

fn bgrat(
    mut a: f64,
    mut b: f64,
    mut x: f64,
    mut y: f64,
    mut w: &mut f64,
    mut eps: f64,
    mut ierr: &mut i32,
    mut log_w: bool,
) {
    let mut c: [f64; 30] = [0.; 30];
    let mut d: [f64; 30] = [0.; 30];
    let mut bm1: f64 = b - 0.5 - 0.5;
    let mut nu: f64 = a + bm1 * 0.5;
    let mut lnx: f64 = if y > 0.375 { x.ln() } else { alnrel(-y) };
    let mut z: f64 = -nu * lnx;
    if b * z == 0.0 {
        println!(
            "bgrat(a={}, b={}, x={}, y={}): z={}, b*z == 0 underflow, hence inaccurate pbeta()",
            a, b, x, y, z,
        );
        *ierr = 1;
        return;
    }
    let mut log_r: f64 = b.ln() + (gam1(b)).ln_1p() + b * z.ln() + nu * lnx;
    let mut log_u: f64 = log_r - (algdiv(b, a) + b * nu.ln());
    let mut u: f64 = (log_u).exp();
    if log_u == f64::NEG_INFINITY {
        *ierr = 2;
        return;
    }
    let mut u_0 = u == 0.0;
    let mut l: f64 = if log_w {
        if *w == -1.0 / 0.0 {
            0.0
        } else {
            (*w - log_u).exp()
        }
    } else if *w == 0.0 {
        0.0
    } else {
        ((*w).ln() - log_u).exp()
    };
    let mut q_r: f64 = grat_r(b, z, log_r, eps);
    let mut v: f64 = 0.25 / (nu * nu);
    let mut t2: f64 = lnx * 0.25 * lnx;
    let mut j: f64 = q_r;
    let mut sum: f64 = j;
    let mut t: f64 = 1.0;
    let mut cn: f64 = 1.0;
    let mut n2: f64 = 0.0;
    let mut n: libc::c_int = 1 as libc::c_int;
    while n <= 30 as libc::c_int {
        let mut bp2n: f64 = b + n2;
        j = (bp2n * (bp2n + 1.0) * j + (z + bp2n + 1.0) * t) * v;
        n2 += 2.0;
        t *= t2;
        cn /= n2 * (n2 + 1.0);
        let mut nm1: libc::c_int = n - 1 as libc::c_int;
        c[nm1 as usize] = cn;
        let mut s: f64 = 0.0;
        if n > 1 as libc::c_int {
            let mut coef: f64 = b - n as f64;
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= nm1 {
                s += coef * c[(i - 1 as libc::c_int) as usize] * d[(nm1 - i) as usize];
                coef += b;
                i += 1;
            }
        }
        d[nm1 as usize] = bm1 * cn + s / n as f64;
        let mut dj: f64 = d[nm1 as usize] * j;
        sum += dj;
        if sum <= 0.0 {
            *ierr = 3 as libc::c_int;
            return;
        }
        if dj.abs() <= eps * (sum + l) {
            *ierr = 0 as libc::c_int;
            break;
        } else {
            if n == 30 as libc::c_int {
                *ierr = 4 as libc::c_int;
                println!(
                    "bgrat(a={}, b={}, x={}) *no* convergence: NOTIFY R-core!\n dj={}, rel.err={}",
                    a,
                    b,
                    x,
                    dj,
                    dj.abs() / (sum + l),
                );
            }
            n += 1;
        }
    }
    if log_w as u64 != 0 {
        *w = logspace_add(*w, log_u + sum.ln());
    } else {
        *w += if u_0 as libc::c_uint != 0 {
            (log_u + sum.ln()).exp()
        } else {
            u * sum
        };
    };
}

fn grat_r(mut a: f64, mut x: f64, mut log_r: f64, mut eps: f64) -> f64 {
    if a * x == 0.0 {
        if x <= a {
            return (-log_r).exp();
        } else {
            return 0.0;
        }
    } else if a == 0.5 {
        if x < 0.25 {
            let mut p: f64 = erf__(x.sqrt());
            return (0.5 - p + 0.5) * (-log_r).exp();
        } else {
            let mut sx: f64 = x.sqrt();
            let mut q_r: f64 = erfc1(1 as libc::c_int, sx) / sx * 1.772453850905516027298167483341;
            return q_r;
        }
    } else if x < 1.1 {
        let mut an: f64 = 3.0;
        let mut c: f64 = x;
        let mut sum: f64 = x / (a + 3.0);
        let mut tol: f64 = eps * 0.1 / (a + 1.0);
        let mut t: f64 = 0.;
        loop {
            an += 1.0;
            c *= -(x / an);
            t = c / (a + an);
            sum += t;
            if !(t.abs() > tol) {
                break;
            }
        }
        let mut j: f64 = a * x * ((sum / 6.0 - 0.5 / (a + 2.0)) * x + 1.0 / (a + 1.0));
        let mut z: f64 = a * x.ln();
        let mut h: f64 = gam1(a);
        let mut g: f64 = h + 1.0;
        if x >= 0.25 && a < x / 2.59 || z > -0.13394 {
            let mut l: f64 = z.exp_m1();
            let mut q: f64 = ((l + 0.5 + 0.5) * j - l) * g - h;
            if q <= 0.0 {
                return 0.0;
            } else {
                return q * (-log_r).exp();
            }
        } else {
            let mut p_0: f64 = z.exp() * g * (0.5 - j + 0.5);
            return (0.5 - p_0 + 0.5) * (-log_r).exp();
        }
    } else {
        let mut a2n_1: f64 = 1.0;
        let mut a2n: f64 = 1.0;
        let mut b2n_1: f64 = x;
        let mut b2n: f64 = x + (1.0 - a);
        let mut c_0: f64 = 1.0;
        let mut am0: f64 = 0.;
        let mut an0: f64 = 0.;
        loop {
            a2n_1 = x * a2n + c_0 * a2n_1;
            b2n_1 = x * b2n + c_0 * b2n_1;
            am0 = a2n_1 / b2n_1;
            c_0 += 1.0;
            let mut c_a: f64 = c_0 - a;
            a2n = a2n_1 + c_a * a2n;
            b2n = b2n_1 + c_a * b2n;
            an0 = a2n / b2n;
            if !((an0 - am0).abs() >= eps * an0) {
                break;
            }
        }
        return an0;
    };
}

fn basym(mut a: f64, mut b: f64, mut lambda: f64, mut eps: f64, mut log_p: bool) -> f64 {
    const e0: f64 = 1.12837916709551;
    const e1: f64 = 0.353553390593274;
    const ln_e0: f64 = 0.120782237635245;
    let mut a0: [f64; 21] = [0.; 21];
    let mut b0: [f64; 21] = [0.; 21];
    let mut c: [f64; 21] = [0.; 21];
    let mut d: [f64; 21] = [0.; 21];
    let mut f: f64 = a * rlog1(-lambda / a) + b * rlog1(lambda / b);
    let mut t: f64 = 0.;
    if log_p {
        t = -f;
    } else {
        t = (-f).exp();
        if t == 0.0 {
            return 0 as libc::c_int as f64;
        }
    }
    let mut z0: f64 = f.sqrt();
    let mut z: f64 = z0 / e1 * 0.5;
    let mut z2: f64 = f + f;
    let mut h: f64 = 0.;
    let mut r0: f64 = 0.;
    let mut r1: f64 = 0.;
    let mut w0: f64 = 0.;
    if a < b {
        h = a / b;
        r0 = 1.0 / (h + 1.0);
        r1 = (b - a) / b;
        w0 = 1.0 / (a * (h + 1.0)).sqrt();
    } else {
        h = b / a;
        r0 = 1.0 / (h + 1.0);
        r1 = (b - a) / a;
        w0 = 1.0 / (b * (h + 1.0)).sqrt();
    }
    a0[0 as libc::c_int as usize] = r1 * 0.66666666666666663;
    c[0 as libc::c_int as usize] = a0[0 as libc::c_int as usize] * -0.5;
    d[0 as libc::c_int as usize] = -c[0 as libc::c_int as usize];
    let mut j0: f64 = 0.5 / e0 * erfc1(1 as libc::c_int, z0);
    let mut j1: f64 = e1;
    let mut sum: f64 = j0 + d[0 as libc::c_int as usize] * w0 * j1;
    let mut s: f64 = 1.0;
    let mut h2: f64 = h * h;
    let mut hn: f64 = 1.0;
    let mut w: f64 = w0;
    let mut znm1: f64 = z;
    let mut zn: f64 = z2;
    let mut n: libc::c_int = 2 as libc::c_int;
    while n <= 20 as libc::c_int {
        hn *= h2;
        a0[(n - 1 as libc::c_int) as usize] = r0 * 2.0 * (h * hn + 1.0) / (n as f64 + 2.0);
        let mut np1: libc::c_int = n + 1 as libc::c_int;
        s += hn;
        a0[(np1 - 1 as libc::c_int) as usize] = r1 * 2.0 * s / (n as f64 + 3.0);
        let mut i: libc::c_int = n;
        while i <= np1 {
            let mut r: f64 = (i as f64 + 1.0) * -0.5;
            b0[0 as libc::c_int as usize] = r * a0[0 as libc::c_int as usize];
            let mut m: libc::c_int = 2 as libc::c_int;
            while m <= i {
                let mut bsum: f64 = 0.0;
                let mut j: libc::c_int = 1 as libc::c_int;
                while j <= m - 1 as libc::c_int {
                    let mut mmj: libc::c_int = m - j;
                    bsum += (j as f64 * r - mmj as f64)
                        * a0[(j - 1 as libc::c_int) as usize]
                        * b0[(mmj - 1 as libc::c_int) as usize];
                    j += 1;
                }
                b0[(m - 1 as libc::c_int) as usize] =
                    r * a0[(m - 1 as libc::c_int) as usize] + bsum / m as f64;
                m += 1;
            }
            c[(i - 1 as libc::c_int) as usize] =
                b0[(i - 1 as libc::c_int) as usize] / (i as f64 + 1.0);
            let mut dsum: f64 = 0.0;
            let mut j_0: libc::c_int = 1 as libc::c_int;
            while j_0 <= i - 1 as libc::c_int {
                dsum +=
                    d[(i - j_0 - 1 as libc::c_int) as usize] * c[(j_0 - 1 as libc::c_int) as usize];
                j_0 += 1;
            }
            d[(i - 1 as libc::c_int) as usize] = -(dsum + c[(i - 1 as libc::c_int) as usize]);
            i += 1;
        }
        j0 = e1 * znm1 + (n as f64 - 1.0) * j0;
        j1 = e1 * zn + n as f64 * j1;
        znm1 = z2 * znm1;
        zn = z2 * zn;
        w *= w0;
        let mut t0: f64 = d[(n - 1 as libc::c_int) as usize] * w * j0;
        w *= w0;
        let mut t1: f64 = d[(np1 - 1 as libc::c_int) as usize] * w * j1;
        sum += t0 + t1;
        if t0.abs() + t1.abs() <= eps * sum {
            break;
        }
        n += 2;
    }
    if log_p {
        return ln_e0 + t - bcorr(a, b) + sum.ln();
    } else {
        let mut u: f64 = (-bcorr(a, b)).exp();
        return e0 * t * u * sum;
    };
}

fn exparg(mut l: i32) -> f64 {
    const lnb: f64 = 0.69314718055995;
    let m = if l == 0 {
        rf_i1mach(16 as libc::c_int)
    } else {
        rf_i1mach(15 as libc::c_int) - 1 as libc::c_int
    };
    m as f64 * lnb * 0.99999
}

fn esum(mut mu: libc::c_int, mut x: f64, mut log_p: bool) -> f64 {
    if log_p {
        return x + mu as f64;
    }
    let mut w: f64 = 0.;
    if x > 0.0 {
        if mu > 0 {
            return (mu as f64).exp() * x.exp();
        }
        w = mu as f64 + x;
        if w < 0.0 {
            return (mu as f64).exp() * x.exp();
        }
    } else {
        if mu < 0 {
            return (mu as f64).exp() * x.exp();
        }
        w = mu as f64 + x;
        if w > 0.0 {
            return (mu as f64).exp() * x.exp();
        }
    }
    w.exp()
}

fn alnrel(mut a: f64) -> f64 {
    if a.abs() > 0.375 {
        return (1.0 + a).ln();
    }
    const p1: f64 = -1.29418923021993;
    const p2: f64 = 0.405303492862024;
    const p3: f64 = -0.0178874546012214;
    const q1: f64 = -1.62752256355323;
    const q2: f64 = 0.747811014037616;
    const q3: f64 = -0.0845104217945565;
    let mut t: f64 = a / (a + 2.0);
    let mut t2: f64 = t * t;
    let mut w: f64 =
        (((p3 * t2 + p2) * t2 + p1) * t2 + 1.0) / (((q3 * t2 + q2) * t2 + q1) * t2 + 1.0);
    return t * 2.0 * w;
}

fn rlog1(mut x: f64) -> f64 {
    const a: f64 = 0.0566749439387324;
    const b: f64 = 0.0456512608815524;
    const p0: f64 = 0.333333333333333;
    const p1: f64 = -0.224696413112536;
    const p2: f64 = 0.00620886815375787;
    const q1: f64 = -1.27408923933623;
    const q2: f64 = 0.354508718369557;
    let mut h: f64 = 0.;
    let mut r: f64 = 0.;
    let mut t: f64 = 0.;
    let mut w: f64 = 0.;
    let mut w1: f64 = 0.;
    if x < -0.39 || x > 0.57 {
        w = x + 0.5 + 0.5;
        return x - w.ln();
    }
    if x < -0.18 {
        h = x + 0.3;
        h /= 0.7;
        w1 = a - h * 0.3;
    } else if x > 0.18 {
        h = x * 0.75 - 0.25;
        w1 = b + h / 3.0;
    } else {
        h = x;
        w1 = 0.0;
    }
    r = h / (h + 2.0);
    t = r * r;
    w = ((p2 * t + p1) * t + p0) / ((q2 * t + q1) * t + 1.0);
    return t * 2.0 * (1.0 / (1.0 - r) - r * w) + w1;
}

fn erf__(mut x: f64) -> f64 {
    const c: f64 = 0.564189583547756;
    static a: [f64; 5] = [
        7.7105849500132e-5,
        -0.00133733772997339,
        0.0323076579225834,
        0.0479137145607681,
        0.128379167095513,
    ];
    static b: [f64; 3] = [0.00301048631703895, 0.0538971687740286, 0.375795757275549];
    static p: [f64; 8] = [
        -1.36864857382717e-7,
        0.564195517478974,
        7.21175825088309,
        43.1622272220567,
        152.98928504694,
        339.320816734344,
        451.918953711873,
        300.459261020162,
    ];
    static q: [f64; 8] = [
        1.0,
        12.7827273196294,
        77.0001529352295,
        277.585444743988,
        638.980264465631,
        931.35409485061,
        790.950925327898,
        300.459260956983,
    ];
    static r: [f64; 5] = [
        2.10144126479064,
        26.2370141675169,
        21.3688200555087,
        4.6580782871847,
        0.282094791773523,
    ];
    static s: [f64; 4] = [
        94.153775055546,
        187.11481179959,
        99.0191814623914,
        18.0124575948747,
    ];
    let mut t: f64 = 0.;
    let mut x2: f64 = 0.;
    let mut ax: f64 = 0.;
    let mut bot: f64 = 0.;
    let mut top: f64 = 0.;
    ax = x.abs();
    if ax <= 0.5 {
        t = x * x;
        top = (((a[0 as libc::c_int as usize] * t + a[1 as libc::c_int as usize]) * t
            + a[2 as libc::c_int as usize])
            * t
            + a[3 as libc::c_int as usize])
            * t
            + a[4 as libc::c_int as usize]
            + 1.0;
        bot = ((b[0 as libc::c_int as usize] * t + b[1 as libc::c_int as usize]) * t
            + b[2 as libc::c_int as usize])
            * t
            + 1.0;
        return x * (top / bot);
    }
    if ax <= 4.0 {
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
        let mut R: f64 = 0.5 - (-x * x).exp() * top / bot + 0.5;
        return if x < 0 as libc::c_int as f64 { -R } else { R };
    }
    if ax >= 5.8 {
        return (if x > 0 as libc::c_int as f64 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }) as f64;
    }
    x2 = x * x;
    t = 1.0 / x2;
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
        + 1.0;
    t = (c - top / (x2 * bot)) / ax;
    let mut R_0: f64 = 0.5 - (-x2).exp() * t + 0.5;
    return if x < 0 as libc::c_int as f64 {
        -R_0
    } else {
        R_0
    };
}
fn erfc1(mut ind: libc::c_int, mut x: f64) -> f64 {
    const c: f64 = 0.564189583547756;
    static a: [f64; 5] = [
        7.7105849500132e-5,
        -0.00133733772997339,
        0.0323076579225834,
        0.0479137145607681,
        0.128379167095513,
    ];
    static b: [f64; 3] = [0.00301048631703895, 0.0538971687740286, 0.375795757275549];
    static p: [f64; 8] = [
        -1.36864857382717e-7,
        0.564195517478974,
        7.21175825088309,
        43.1622272220567,
        152.98928504694,
        339.320816734344,
        451.918953711873,
        300.459261020162,
    ];
    static q: [f64; 8] = [
        1.0,
        12.7827273196294,
        77.0001529352295,
        277.585444743988,
        638.980264465631,
        931.35409485061,
        790.950925327898,
        300.459260956983,
    ];
    static r: [f64; 5] = [
        2.10144126479064,
        26.2370141675169,
        21.3688200555087,
        4.6580782871847,
        0.282094791773523,
    ];
    static s: [f64; 4] = [
        94.153775055546,
        187.11481179959,
        99.0191814623914,
        18.0124575948747,
    ];
    let mut ret_val: f64 = 0.;
    let mut e: f64 = 0.;
    let mut t: f64 = 0.;
    let mut w: f64 = 0.;
    let mut bot: f64 = 0.;
    let mut top: f64 = 0.;
    let mut ax: f64 = x.abs();
    if ax <= 0.5 {
        let mut t_0: f64 = x * x;
        let mut top_0: f64 =
            (((a[0 as libc::c_int as usize] * t_0 + a[1 as libc::c_int as usize]) * t_0
                + a[2 as libc::c_int as usize])
                * t_0
                + a[3 as libc::c_int as usize])
                * t_0
                + a[4 as libc::c_int as usize]
                + 1.0;
        let mut bot_0: f64 = ((b[0 as libc::c_int as usize] * t_0 + b[1 as libc::c_int as usize])
            * t_0
            + b[2 as libc::c_int as usize])
            * t_0
            + 1.0;
        ret_val = 0.5 - x * (top_0 / bot_0) + 0.5;
        if ind != 0 as libc::c_int {
            ret_val = (t_0).exp() * ret_val;
        }
        return ret_val;
    }
    if ax <= 4.0 {
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
        if x <= -5.6 {
            ret_val = 2.0;
            if ind != 0 as libc::c_int {
                ret_val = (x * x).exp() * 2.0;
            }
            return ret_val;
        }
        if ind == 0 as libc::c_int && (x > 100.0 || x * x > -exparg(1 as libc::c_int)) {
            return 0.0;
        }
        t = 1.0 / (x * x);
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
            + 1.0;
        ret_val = (c - t * top / bot) / ax;
    }
    if ind != 0 as libc::c_int {
        if x < 0.0 {
            ret_val = (x * x).exp() * 2.0 - ret_val;
        }
    } else {
        w = x * x;
        t = w;
        e = w - t;
        ret_val = (0.5 - e + 0.5) * (-t).exp() * ret_val;
        if x < 0.0 {
            ret_val = 2.0 - ret_val;
        }
    }
    return ret_val;
}

fn gam1(mut a: f64) -> f64 {
    let mut d: f64 = 0.;
    let mut t: f64 = 0.;
    let mut w: f64 = 0.;
    let mut bot: f64 = 0.;
    let mut top: f64 = 0.;
    t = a;
    d = a - 0.5;
    if d > 0.0 {
        t = d - 0.5;
    }
    if t < 0.0 {
        static r: [f64; 9] = [
            -0.422784335098468,
            -0.771330383816272,
            -0.244757765222226,
            0.118378989872749,
            9.30357293360349e-4,
            -0.0118290993445146,
            0.00223047661158249,
            2.66505979058923e-4,
            -1.32674909766242e-4,
        ];
        const s1: f64 = 0.273076135303957;
        const s2: f64 = 0.0559398236957378;
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
        bot = (s2 * t + s1) * t + 1.0;
        w = top / bot;
        if d > 0.0 {
            return t * w / a;
        } else {
            return a * (w + 0.5 + 0.5);
        }
    } else if t == 0 as libc::c_int as f64 {
        return 0.0;
    } else {
        static p: [f64; 7] = [
            0.577215664901533,
            -0.409078193005776,
            -0.230975380857675,
            0.0597275330452234,
            0.0076696818164949,
            -0.00514889771323592,
            5.89597428611429e-4,
        ];
        static q: [f64; 5] = [
            1.0,
            0.427569613095214,
            0.158451672430138,
            0.0261132021441447,
            0.00423244297896961,
        ];
        top = (((((p[6] * t + p[5]) * t + p[4]) * t + p[3]) * t + p[2]) * t + p[1]) * t + p[0];
        bot = (((q[4] * t + q[3]) * t + q[2]) * t + q[1]) * t + 1.0;
        w = top / bot;
        if d > 0.0 {
            return t / a * (w - 0.5 - 0.5);
        } else {
            return a * w;
        }
    };
}

fn gamln1(mut a: f64) -> f64 {
    let mut w: f64 = 0.;
    if a < 0.6 {
        const P0: f64 = 0.577215664901533;
        const P1: f64 = 0.844203922187225;
        const P2: f64 = -0.168860593646662;
        const P3: f64 = -0.780427615533591;
        const P4: f64 = -0.402055799310489;
        const P5: f64 = -0.0673562214325671;
        const P6: f64 = -0.00271935708322958;
        const Q1: f64 = 2.88743195473681;
        const Q2: f64 = 3.12755088914843;
        const Q3: f64 = 1.56875193295039;
        const Q4: f64 = 0.361951990101499;
        const Q5: f64 = 0.0325038868253937;
        const Q6: f64 = 6.67465618796164e-4;
        w = ((((((P6 * a + P5) * a + P4) * a + P3) * a + P2) * a + P1) * a + P0)
            / ((((((Q6 * a + Q5) * a + Q4) * a + Q3) * a + Q2) * a + Q1) * a + 1.0);
        return -a * w;
    } else {
        const R0: f64 = 0.422784335098467;
        const R1: f64 = 0.848044614534529;
        const R2: f64 = 0.565221050691933;
        const R3: f64 = 0.156513060486551;
        const R4: f64 = 0.017050248402265;
        const R5: f64 = 4.97958207639485e-4;
        const S1: f64 = 1.24313399877507;
        const S2: f64 = 0.548042109832463;
        const S3: f64 = 0.10155218743983;
        const S4: f64 = 0.00713309612391;
        const S5: f64 = 1.16165475989616e-4;
        let mut x: f64 = a - 0.5 - 0.5;
        w = (((((R5 * x + R4) * x + R3) * x + R2) * x + R1) * x + R0)
            / (((((S5 * x + S4) * x + S3) * x + S2) * x + S1) * x + 1.0);
        return x * w;
    };
}

fn psi(mut x: f64) -> f64 {
    let mut current_block: u64;
    static piov4: f64 = 0.785398163397448;
    static dx0: f64 = 1.461632144968362341262659542325721325;
    static p1: [f64; 7] = [
        0.0089538502298197,
        4.77762828042627,
        142.441585084029,
        1186.45200713425,
        3633.51846806499,
        4138.10161269013,
        1305.60269827897,
    ];
    static q1: [f64; 6] = [
        44.8452573429826,
        520.752771467162,
        2210.0079924783,
        3641.27349079381,
        1908.310765963,
        6.91091682714533e-6,
    ];
    static p2: [f64; 4] = [
        -2.12940445131011,
        -7.01677227766759,
        -4.48616543918019,
        -0.648157123766197,
    ];
    static q2: [f64; 4] = [
        32.2703493791143,
        89.2920700481861,
        54.6117738103215,
        7.77788548522962,
    ];
    let mut i: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut nq: libc::c_int = 0;
    let mut d2: f64 = 0.;
    let mut w: f64 = 0.;
    let mut z: f64 = 0.;
    let mut den: f64 = 0.;
    let mut aug: f64 = 0.;
    let mut sgn: f64 = 0.;
    let mut xmx0: f64 = 0.;
    let mut xmax1: f64 = 0.;
    let mut upper: f64 = 0.;
    let mut xsmall: f64 = 0.;
    xmax1 = 2147483647 as libc::c_int as f64;
    d2 = 0.5 / rf_d1mach(3);
    if xmax1 > d2 {
        xmax1 = d2;
    }
    xsmall = 1e-9;
    aug = 0.0;
    if x < 0.5 {
        if x.abs() <= xsmall {
            if x == 0.0 {
                current_block = 16813246331342052853;
            } else {
                aug = -1.0 / x;
                current_block = 14434620278749266018;
            }
        } else {
            w = -x;
            sgn = piov4;
            if w <= 0.0 {
                w = -w;
                sgn = -sgn;
            }
            if w >= xmax1 {
                current_block = 16813246331342052853;
            } else {
                nq = w as libc::c_int;
                w -= nq as f64;
                nq = (w * 4.0) as libc::c_int;
                w = (w - nq as f64 * 0.25) * 4.0;
                n = nq / 2 as libc::c_int;
                if n + n != nq {
                    w = 1.0 - w;
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
                    if z == 0.0 {
                        current_block = 16813246331342052853;
                    } else {
                        aug = sgn * (z.cos() / z.sin() * 4.0);
                        current_block = 14434620278749266018;
                    }
                } else {
                    aug = sgn * (z.sin() / z.cos() * 4.0);
                    current_block = 14434620278749266018;
                }
            }
        }
        match current_block {
            16813246331342052853 => return 0.0,
            _ => {
                x = 1.0 - x;
            }
        }
    }
    if x <= 3.0 {
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
        w = 1.0 / (x * x);
        den = w;
        upper = p2[0 as libc::c_int as usize] * w;
        i = 1 as libc::c_int;
        while i <= 3 as libc::c_int {
            den = (den + q2[(i - 1 as libc::c_int) as usize]) * w;
            upper = (upper + p2[i as usize]) * w;
            i += 1;
        }
        aug = upper / (den + q2[3 as libc::c_int as usize]) - 0.5 / x + aug;
    }
    return aug + x.ln();
}

fn betaln(mut a0: f64, mut b0: f64) -> f64 {
    const e: f64 = 0.918938533204673;
    let mut a: f64 = if a0 < b0 { a0 } else { b0 };
    let mut b: f64 = if a0 > b0 { a0 } else { b0 };
    if a < 8.0 {
        let mut n: libc::c_int = 0;
        if a < 1.0 {
            if b < 8.0 {
                return gamln(a) + (gamln(b) - gamln(a + b));
            } else {
                return gamln(a) + algdiv(a, b);
            }
        }
        let mut w: f64 = 0.;
        if a < 2.0 {
            if b <= 2.0 {
                return gamln(a) + gamln(b) - gsumln(a, b);
            }
            if b < 8.0 {
                w = 0.0;
            } else {
                return gamln(a) + algdiv(a, b);
            }
        } else if b <= 1e3 {
            n = (a - 1.0) as libc::c_int;
            w = 1.0;
            let mut i: libc::c_int = 1 as libc::c_int;
            while i <= n {
                a += -1.0;
                let mut h: f64 = a / b;
                w *= h / (h + 1.0);
                i += 1;
            }
            w = w.ln();
            if b >= 8.0 {
                return w + gamln(a) + algdiv(a, b);
            }
        } else {
            let mut n_0: libc::c_int = (a - 1.0) as libc::c_int;
            w = 1.0;
            let mut i_1: libc::c_int = 1 as libc::c_int;
            while i_1 <= n_0 {
                a += -1.0;
                w *= a / (a / b + 1.0);
                i_1 += 1;
            }
            return w.ln() - n_0 as f64 * b.ln() + (gamln(a) + algdiv(a, b));
        }
        n = (b - 1.0) as libc::c_int;
        let mut z: f64 = 1.0;
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 <= n {
            b += -1.0;
            z *= b / (a + b);
            i_0 += 1;
        }
        return w + z.ln() + (gamln(a) + (gamln(b) - gsumln(a, b)));
    } else {
        let mut w_0: f64 = bcorr(a, b);
        let mut h_0: f64 = a / b;
        let mut u: f64 = -(a - 0.5) * (h_0 / (h_0 + 1.0)).ln();
        let mut v: f64 = b * alnrel(h_0);
        if u > v {
            return b.ln() * -0.5 + e + w_0 - v - u;
        } else {
            return b.ln() * -0.5 + e + w_0 - u - v;
        }
    };
}

fn gsumln(a: f64, b: f64) -> f64 {
    let x = a + b - 2.0;
    if x <= 0.25 {
        return gamln1(x + 1.0);
    }
    if x <= 1.25 {
        return gamln1(x) + alnrel(x);
    }
    return gamln1(x - 1.0) + (x * (x + 1.0)).ln();
}

fn bcorr(mut a0: f64, mut b0: f64) -> f64 {
    const c0: f64 = 0.0833333333333333;
    const c1: f64 = -0.00277777777760991;
    const c2: f64 = 7.9365066682539e-4;
    const c3: f64 = -5.9520293135187e-4;
    const c4: f64 = 8.37308034031215e-4;
    const c5: f64 = -0.00165322962780713;
    let mut ret_val: f64 = 0.;
    let mut r1: f64 = 0.;
    let mut a: f64 = 0.;
    let mut b: f64 = 0.;
    let mut c: f64 = 0.;
    let mut h: f64 = 0.;
    let mut t: f64 = 0.;
    let mut w: f64 = 0.;
    let mut x: f64 = 0.;
    let mut s3: f64 = 0.;
    let mut s5: f64 = 0.;
    let mut x2: f64 = 0.;
    let mut s7: f64 = 0.;
    let mut s9: f64 = 0.;
    let mut s11: f64 = 0.;
    a = if a0 < b0 { a0 } else { b0 };
    b = if a0 > b0 { a0 } else { b0 };
    h = a / b;
    c = h / (h + 1.0);
    x = 1.0 / (h + 1.0);
    x2 = x * x;
    s3 = x + x2 + 1.0;
    s5 = x + x2 * s3 + 1.0;
    s7 = x + x2 * s5 + 1.0;
    s9 = x + x2 * s7 + 1.0;
    s11 = x + x2 * s9 + 1.0;
    r1 = 1.0 / b;
    t = r1 * r1;
    w = ((((c5 * s11 * t + c4 * s9) * t + c3 * s7) * t + c2 * s5) * t + c1 * s3) * t + c0;
    w *= c / b;
    r1 = 1.0 / a;
    t = r1 * r1;
    ret_val = (((((c5 * t + c4) * t + c3) * t + c2) * t + c1) * t + c0) / a + w;
    return ret_val;
}

fn algdiv(mut a: f64, mut b: f64) -> f64 {
    const c0: f64 = 0.0833333333333333;
    const c1: f64 = -0.00277777777760991;
    const c2: f64 = 7.9365066682539e-4;
    const c3: f64 = -5.9520293135187e-4;
    const c4: f64 = 8.37308034031215e-4;
    const c5: f64 = -0.00165322962780713;
    let mut c: f64 = 0.;
    let mut d: f64 = 0.;
    let mut h: f64 = 0.;
    let mut t: f64 = 0.;
    let mut u: f64 = 0.;
    let mut v: f64 = 0.;
    let mut w: f64 = 0.;
    let mut x: f64 = 0.;
    let mut s3: f64 = 0.;
    let mut s5: f64 = 0.;
    let mut x2: f64 = 0.;
    let mut s7: f64 = 0.;
    let mut s9: f64 = 0.;
    let mut s11: f64 = 0.;
    if a > b {
        h = b / a;
        c = 1.0 / (h + 1.0);
        x = h / (h + 1.0);
        d = a + (b - 0.5);
    } else {
        h = a / b;
        c = h / (h + 1.0);
        x = 1.0 / (h + 1.0);
        d = b + (a - 0.5);
    }
    x2 = x * x;
    s3 = x + x2 + 1.0;
    s5 = x + x2 * s3 + 1.0;
    s7 = x + x2 * s5 + 1.0;
    s9 = x + x2 * s7 + 1.0;
    s11 = x + x2 * s9 + 1.0;
    t = 1.0 / (b * b);
    w = ((((c5 * s11 * t + c4 * s9) * t + c3 * s7) * t + c2 * s5) * t + c1 * s3) * t + c0;
    w *= c / b;
    u = d * alnrel(a / b);
    v = a * (b.ln() - 1.0);
    if u > v {
        return w - v - u;
    } else {
        return w - u - v;
    };
}

fn gamln(mut a: f64) -> f64 {
    const d: f64 = 0.418938533204673;
    const c0: f64 = 0.0833333333333333;
    const c1: f64 = -0.00277777777760991;
    const c2: f64 = 7.9365066682539e-4;
    const c3: f64 = -5.9520293135187e-4;
    const c4: f64 = 8.37308034031215e-4;
    const c5: f64 = -0.00165322962780713;
    if a <= 0.8 {
        return gamln1(a) - a.ln();
    } else if a <= 2.25 {
        return gamln1(a - 0.5 - 0.5);
    } else if a < 10.0 {
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = (a - 1.25) as libc::c_int;
        let mut t: f64 = a;
        let mut w: f64 = 1.0;
        i = 1 as libc::c_int;
        while i <= n {
            t += -1.0;
            w *= t;
            i += 1;
        }
        return gamln1(t - 1.0) + w.ln();
    } else {
        let mut t_0: f64 = 1.0 / (a * a);
        let mut w_0: f64 = (((((c5 * t_0 + c4) * t_0 + c3) * t_0 + c2) * t_0 + c1) * t_0 + c0) / a;
        return d + w_0 + (a - 0.5) * (a.ln() - 1.0);
    };
}
