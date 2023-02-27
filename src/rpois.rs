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
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn norm_rand() -> libc::c_double;
    fn unif_rand() -> libc::c_double;
    fn exp_rand() -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn imin2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn imax2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn fsign(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type C2RustUnnamed = libc::c_uint;
pub const TRUE: C2RustUnnamed = 1;
pub const FALSE: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn rpois(mut mu: libc::c_double) -> libc::c_double {
    let mut current_block: u64;
    static mut fact: [libc::c_double; 10] = [
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
    ];
    static mut l: libc::c_int = 0;
    static mut m: libc::c_int = 0;
    static mut b1: libc::c_double = 0.;
    static mut b2: libc::c_double = 0.;
    static mut c: libc::c_double = 0.;
    static mut c0: libc::c_double = 0.;
    static mut c1: libc::c_double = 0.;
    static mut c2: libc::c_double = 0.;
    static mut c3: libc::c_double = 0.;
    static mut pp: [libc::c_double; 36] = [0.; 36];
    static mut p0: libc::c_double = 0.;
    static mut p: libc::c_double = 0.;
    static mut q: libc::c_double = 0.;
    static mut s: libc::c_double = 0.;
    static mut d: libc::c_double = 0.;
    static mut omega: libc::c_double = 0.;
    static mut big_l: libc::c_double = 0.;
    static mut muprev: libc::c_double = 0.0f64;
    static mut muprev2: libc::c_double = 0.0f64;
    let mut del: libc::c_double = 0.;
    let mut difmuk: libc::c_double = 0.0f64;
    let mut E: libc::c_double = 0.0f64;
    let mut fk: libc::c_double = 0.0f64;
    let mut fx: libc::c_double = 0.;
    let mut fy: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut px: libc::c_double = 0.;
    let mut py: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.0f64;
    let mut v: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut pois: libc::c_double = -1.0f64;
    let mut k: libc::c_int = 0;
    let mut kflag: libc::c_int = 0;
    let mut big_mu: libc::c_int = 0;
    let mut new_big_mu: libc::c_int = FALSE as libc::c_int;
    if R_finite(mu) == 0 || mu < 0 as libc::c_int as libc::c_double {
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
    if mu <= 0.0f64 {
        return 0.0f64;
    }
    big_mu = (mu >= 10.0f64) as libc::c_int;
    if big_mu != 0 {
        new_big_mu = FALSE as libc::c_int;
    }
    if !(big_mu != 0 && mu == muprev) {
        if big_mu != 0 {
            new_big_mu = TRUE as libc::c_int;
            muprev = mu;
            s = sqrt(mu);
            d = 6.0f64 * mu * mu;
            big_l = floor(mu - 1.1484f64);
        } else {
            if mu != muprev {
                muprev = mu;
                m = imax2(1 as libc::c_int, mu as libc::c_int);
                l = 0 as libc::c_int;
                p = exp(-mu);
                p0 = p;
                q = p0;
            }
            loop {
                u = unif_rand();
                if u <= p0 {
                    return 0.0f64;
                }
                if l != 0 as libc::c_int {
                    k = if u <= 0.458f64 {
                        1 as libc::c_int
                    } else {
                        imin2(l, m)
                    };
                    while k <= l {
                        if u <= pp[k as usize] {
                            return k as libc::c_double;
                        }
                        k += 1;
                    }
                    if l == 35 as libc::c_int {
                        continue;
                    }
                }
                l += 1;
                k = l;
                while k <= 35 as libc::c_int {
                    p *= mu / k as libc::c_double;
                    q += p;
                    pp[k as usize] = q;
                    if u <= q {
                        l = k;
                        return k as libc::c_double;
                    }
                    k += 1;
                }
                l = 35 as libc::c_int;
            }
        }
    }
    g = mu + s * norm_rand();
    if g >= 0.0f64 {
        pois = floor(g);
        if pois >= big_l {
            return pois;
        }
        fk = pois;
        difmuk = mu - fk;
        u = unif_rand();
        if d * u >= difmuk * difmuk * difmuk {
            return pois;
        }
    }
    if new_big_mu != 0 || mu != muprev2 {
        muprev2 = mu;
        omega = 0.398942280401432677939946059934f64 / s;
        b1 = 0.0416666666666666667f64 / mu;
        b2 = 0.3f64 * b1 * b1;
        c3 = 0.1428571428571428571f64 * b1 * b2;
        c2 = b2 - 15.0f64 * c3;
        c1 = b1 - 6.0f64 * b2 + 45.0f64 * c3;
        c0 = 1.0f64 - b1 + 3.0f64 * b2 - 15.0f64 * c3;
        c = 0.1069f64 / mu;
    }
    if g >= 0.0f64 {
        kflag = 0 as libc::c_int;
        current_block = 16413728243562773128;
    } else {
        current_block = 7330218953828964527;
    }
    loop {
        match current_block {
            16413728243562773128 => {
                if pois < 10 as libc::c_int as libc::c_double {
                    px = -mu;
                    py = pow(mu, pois) / fact[pois as libc::c_int as usize];
                } else {
                    del = 0.0833333333333333333f64 / fk;
                    del = del * (1.0f64 - 4.8f64 * del * del);
                    v = difmuk / fk;
                    if fabs(v) <= 0.25f64 {
                        px = fk
                            * v
                            * v
                            * (((((((0.1250060f64 * v + -0.1384794f64) * v + 0.1421878f64) * v
                                + -0.1661269f64)
                                * v
                                + 0.2000118f64)
                                * v
                                + -0.2500068f64)
                                * v
                                + 0.3333333f64)
                                * v
                                + -0.5f64)
                            - del;
                    } else {
                        px = fk * log(1.0f64 + v) - difmuk - del;
                    }
                    py = 0.398942280401432677939946059934f64 / sqrt(fk);
                }
                x = (0.5f64 - difmuk) / s;
                x *= x;
                fx = -0.5f64 * x;
                fy = omega * (((c3 * x + c2) * x + c1) * x + c0);
                if kflag > 0 as libc::c_int {
                    if c * fabs(u) <= py * exp(px + E) - fy * exp(fx + E) {
                        break;
                    } else {
                        current_block = 7330218953828964527;
                    }
                } else if fy - u * fy <= py * exp(px - fx) {
                    break;
                } else {
                    current_block = 7330218953828964527;
                }
            }
            _ => {
                E = exp_rand();
                u = 2 as libc::c_int as libc::c_double * unif_rand() - 1.0f64;
                t = 1.8f64 + fsign(E, u);
                if !(t > -0.6744f64) {
                    current_block = 7330218953828964527;
                    continue;
                }
                pois = floor(mu + s * t);
                fk = pois;
                difmuk = mu - fk;
                kflag = 1 as libc::c_int;
                current_block = 16413728243562773128;
            }
        }
    }
    return pois;
}
