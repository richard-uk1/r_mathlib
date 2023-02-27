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
    fn expm1(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn norm_rand() -> libc::c_double;
    fn unif_rand() -> libc::c_double;
    fn exp_rand() -> libc::c_double;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rgamma(
    mut a: libc::c_double,
    mut scale: libc::c_double,
) -> libc::c_double {
    static mut sqrt32: libc::c_double = 5.656854f64;
    static mut exp_m1: libc::c_double = 0.36787944117144232159f64;
    static mut q1: libc::c_double = 0.04166669f64;
    static mut q2: libc::c_double = 0.02083148f64;
    static mut q3: libc::c_double = 0.00801191f64;
    static mut q4: libc::c_double = 0.00144121f64;
    static mut q5: libc::c_double = -7.388e-5f64;
    static mut q6: libc::c_double = 2.4511e-4f64;
    static mut q7: libc::c_double = 2.424e-4f64;
    static mut a1: libc::c_double = 0.3333333f64;
    static mut a2: libc::c_double = -0.250003f64;
    static mut a3: libc::c_double = 0.2000062f64;
    static mut a4: libc::c_double = -0.1662921f64;
    static mut a5: libc::c_double = 0.1423657f64;
    static mut a6: libc::c_double = -0.1367177f64;
    static mut a7: libc::c_double = 0.1233795f64;
    static mut aa: libc::c_double = 0.0f64;
    static mut aaa: libc::c_double = 0.0f64;
    static mut s: libc::c_double = 0.;
    static mut s2: libc::c_double = 0.;
    static mut d: libc::c_double = 0.;
    static mut q0: libc::c_double = 0.;
    static mut b: libc::c_double = 0.;
    static mut si: libc::c_double = 0.;
    static mut c: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    let mut q: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut ret_val: libc::c_double = 0.;
    if a.is_nan() as i32 != 0 as libc::c_int || scale.is_nan() as i32 != 0 as libc::c_int {
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
    if a <= 0.0f64 || scale <= 0.0f64 {
        if scale == 0.0f64 || a == 0.0f64 {
            return 0.0f64;
        }
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
    if R_finite(a) == 0 || R_finite(scale) == 0 {
        return 1.0f64 / 0.0f64;
    }
    if a < 1.0f64 {
        e = 1.0f64 + exp_m1 * a;
        loop {
            p = e * unif_rand();
            if p >= 1.0f64 {
                x = -log((e - p) / a);
                if exp_rand() >= (1.0f64 - a) * log(x) {
                    break;
                }
            } else {
                x = exp(log(p) / a);
                if exp_rand() >= x {
                    break;
                }
            }
        }
        return scale * x;
    }
    if a != aa {
        aa = a;
        s2 = a - 0.5f64;
        s = sqrt(s2);
        d = sqrt32 - s * 12.0f64;
    }
    t = norm_rand();
    x = s + 0.5f64 * t;
    ret_val = x * x;
    if t >= 0.0f64 {
        return scale * ret_val;
    }
    u = unif_rand();
    if d * u <= t * t * t {
        return scale * ret_val;
    }
    if a != aaa {
        aaa = a;
        r = 1.0f64 / a;
        q0 = ((((((q7 * r + q6) * r + q5) * r + q4) * r + q3) * r + q2) * r + q1) * r;
        if a <= 3.686f64 {
            b = 0.463f64 + s + 0.178f64 * s2;
            si = 1.235f64;
            c = 0.195f64 / s - 0.079f64 + 0.16f64 * s;
        } else if a <= 13.022f64 {
            b = 1.654f64 + 0.0076f64 * s2;
            si = 1.68f64 / s + 0.275f64;
            c = 0.062f64 / s + 0.024f64;
        } else {
            b = 1.77f64;
            si = 0.75f64;
            c = 0.1515f64 / s;
        }
    }
    if x > 0.0f64 {
        v = t / (s + s);
        if fabs(v) <= 0.25f64 {
            q = q0
                + 0.5f64
                    * t
                    * t
                    * ((((((a7 * v + a6) * v + a5) * v + a4) * v + a3) * v + a2) * v + a1)
                    * v;
        } else {
            q = q0 - s * t + 0.25f64 * t * t + (s2 + s2) * log(1.0f64 + v);
        }
        if log(1.0f64 - u) <= q {
            return scale * ret_val;
        }
    }
    loop {
        e = exp_rand();
        u = unif_rand();
        u = u + u - 1.0f64;
        if u < 0.0f64 {
            t = b - si * e;
        } else {
            t = b + si * e;
        }
        if !(t >= -0.71874483771719f64) {
            continue;
        }
        v = t / (s + s);
        if fabs(v) <= 0.25f64 {
            q = q0
                + 0.5f64
                    * t
                    * t
                    * ((((((a7 * v + a6) * v + a5) * v + a4) * v + a3) * v + a2) * v + a1)
                    * v;
        } else {
            q = q0 - s * t + 0.25f64 * t * t + (s2 + s2) * log(1.0f64 + v);
        }
        if !(q > 0.0f64) {
            continue;
        }
        w = expm1(q);
        if c * fabs(u) <= w * exp(e - 0.5f64 * t * t) {
            break;
        }
    }
    x = s + 0.5f64 * t;
    return scale * x * x;
}
