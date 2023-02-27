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
    fn unif_rand() -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rbeta(mut aa: libc::c_double, mut bb: libc::c_double) -> libc::c_double {
    if aa.is_nan() as i32 != 0 as libc::c_int
        || bb.is_nan() as i32 != 0 as libc::c_int
        || aa < 0.0f64
        || bb < 0.0f64
    {
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
    if R_finite(aa) == 0 && R_finite(bb) == 0 {
        return 0.5f64;
    }
    if aa == 0.0f64 && bb == 0.0f64 {
        return if unif_rand() < 0.5f64 { 0.0f64 } else { 1.0f64 };
    }
    if R_finite(aa) == 0 || bb == 0.0f64 {
        return 1.0f64;
    }
    if R_finite(bb) == 0 || aa == 0.0f64 {
        return 0.0f64;
    }
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut qsame: libc::c_int = 0;
    static mut beta: libc::c_double = 0.;
    static mut gamma: libc::c_double = 0.;
    static mut delta: libc::c_double = 0.;
    static mut k1: libc::c_double = 0.;
    static mut k2: libc::c_double = 0.;
    static mut olda: libc::c_double = -1.0f64;
    static mut oldb: libc::c_double = -1.0f64;
    qsame = (olda == aa && oldb == bb) as libc::c_int;
    if qsame == 0 {
        olda = aa;
        oldb = bb;
    }
    a = fmin2(aa, bb);
    b = fmax2(aa, bb);
    alpha = a + b;
    if a <= 1.0f64 {
        if qsame == 0 {
            beta = 1.0f64 / a;
            delta = 1.0f64 + b - a;
            k1 = delta * (0.0138889f64 + 0.0416667f64 * a) / (b * beta - 0.777778f64);
            k2 = 0.25f64 + (0.5f64 + 0.25f64 / delta) * a;
        }
        loop {
            u1 = unif_rand();
            u2 = unif_rand();
            if u1 < 0.5f64 {
                y = u1 * u2;
                z = u1 * y;
                if 0.25f64 * u2 + z - y >= k1 {
                    continue;
                }
            } else {
                z = u1 * u1 * u2;
                if z <= 0.25f64 {
                    v = beta * log(u1 / (1.0f64 - u1));
                    if v <= 1024 as libc::c_int as libc::c_double
                        * 0.693147180559945309417232121458f64
                    {
                        w = b * exp(v);
                        if R_finite(w) == 0 {
                            w = 1.7976931348623157e+308f64;
                        }
                    } else {
                        w = 1.7976931348623157e+308f64;
                    }
                    break;
                } else if z >= k2 {
                    continue;
                }
            }
            v = beta * log(u1 / (1.0f64 - u1));
            if v <= 1024 as libc::c_int as libc::c_double * 0.693147180559945309417232121458f64 {
                w = b * exp(v);
                if R_finite(w) == 0 {
                    w = 1.7976931348623157e+308f64;
                }
            } else {
                w = 1.7976931348623157e+308f64;
            }
            if alpha * (log(alpha / (a + w)) + v) - 1.3862944f64 >= log(z) {
                break;
            }
        }
        return if aa == a { a / (a + w) } else { w / (a + w) };
    } else {
        if qsame == 0 {
            beta = sqrt((alpha - 2.0f64) / (2.0f64 * a * b - alpha));
            gamma = a + 1.0f64 / beta;
        }
        loop {
            u1 = unif_rand();
            u2 = unif_rand();
            v = beta * log(u1 / (1.0f64 - u1));
            if v <= 1024 as libc::c_int as libc::c_double * 0.693147180559945309417232121458f64 {
                w = a * exp(v);
                if R_finite(w) == 0 {
                    w = 1.7976931348623157e+308f64;
                }
            } else {
                w = 1.7976931348623157e+308f64;
            }
            z = u1 * u1 * u2;
            r = gamma * v - 1.3862944f64;
            s = a + r - w;
            if s + 2.609438f64 >= 5.0f64 * z {
                break;
            }
            t = log(z);
            if s > t {
                break;
            }
            if !(r + alpha * log(alpha / (b + w)) < t) {
                break;
            }
        }
        return if aa != a { b / (b + w) } else { w / (b + w) };
    };
}
