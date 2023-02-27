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
    fn round(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn unif_rand() -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn imin2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
static mut w: *mut libc::c_double = 0 as *const libc::c_double as *mut libc::c_double;
static mut allocated_n: libc::c_int = 0;
unsafe extern "C" fn w_free() {
    if w.is_null() {
        return;
    }
    free(w as *mut libc::c_void);
    w = 0 as *mut libc::c_double;
    allocated_n = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn signrank_free() {
    w_free();
}
unsafe extern "C" fn w_init_maybe(mut n: libc::c_int) {
    let mut u: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    u = n * (n + 1 as libc::c_int) / 2 as libc::c_int;
    c = u / 2 as libc::c_int;
    if !w.is_null() {
        if n != allocated_n {
            w_free();
        } else {
            return;
        }
    }
    if w.is_null() {
        w = calloc(
            (c as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if w.is_null() {
            printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                b"signrank allocation error\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        allocated_n = n;
    }
}
unsafe extern "C" fn csignrank(mut k: libc::c_int, mut n: libc::c_int) -> libc::c_double {
    let mut c: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    u = n * (n + 1 as libc::c_int) / 2 as libc::c_int;
    c = u / 2 as libc::c_int;
    if k < 0 as libc::c_int || k > u {
        return 0 as libc::c_int as libc::c_double;
    }
    if k > c {
        k = u - k;
    }
    if n == 1 as libc::c_int {
        return 1.0f64;
    }
    if *w.offset(0 as libc::c_int as isize) == 1.0f64 {
        return *w.offset(k as isize);
    }
    let ref mut fresh0 = *w.offset(1 as libc::c_int as isize);
    *fresh0 = 1.0f64;
    *w.offset(0 as libc::c_int as isize) = *fresh0;
    j = 2 as libc::c_int;
    while j < n + 1 as libc::c_int {
        let mut i: libc::c_int = 0;
        let mut end: libc::c_int = imin2(j * (j + 1 as libc::c_int) / 2 as libc::c_int, c);
        i = end;
        while i >= j {
            *w.offset(i as isize) += *w.offset((i - j) as isize);
            i -= 1;
        }
        j += 1;
    }
    return *w.offset(k as isize);
}
#[no_mangle]
pub unsafe extern "C" fn dsignrank(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int || n.is_nan() as i32 != 0 as libc::c_int {
        return x + n;
    }
    n = round(n);
    if n <= 0 as libc::c_int as libc::c_double {
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
    if fabs(x - round(x)) > 1e-7f64 {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    x = round(x);
    if x < 0 as libc::c_int as libc::c_double
        || x > n * (n + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double
    {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    let mut nn: libc::c_int = n as libc::c_int;
    w_init_maybe(nn);
    d = if log_p != 0 {
        log(csignrank(x as libc::c_int, nn)) - n * 0.693147180559945309417232121458f64
    } else {
        exp(log(csignrank(x as libc::c_int, nn)) - n * 0.693147180559945309417232121458f64)
    };
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn psignrank(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut f: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int || n.is_nan() as i32 != 0 as libc::c_int {
        return x + n;
    }
    if R_finite(n) == 0 {
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
    n = round(n);
    if n <= 0 as libc::c_int as libc::c_double {
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
    x = round(x + 1e-7f64);
    if x < 0.0f64 {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    if x >= n * (n + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double {
        return if lower_tail != 0 {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    let mut nn: libc::c_int = n as libc::c_int;
    w_init_maybe(nn);
    f = exp(-n * 0.693147180559945309417232121458f64);
    p = 0 as libc::c_int as libc::c_double;
    if x <= n * (n + 1 as libc::c_int as libc::c_double) / 4 as libc::c_int as libc::c_double {
        i = 0 as libc::c_int;
        while i as libc::c_double <= x {
            p += csignrank(i, nn) * f;
            i += 1;
        }
    } else {
        x = n * (n + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double - x;
        i = 0 as libc::c_int;
        while (i as libc::c_double) < x {
            p += csignrank(i, nn) * f;
            i += 1;
        }
        lower_tail = (lower_tail == 0) as libc::c_int;
    }
    return if lower_tail != 0 {
        if log_p != 0 {
            log(p)
        } else {
            p
        }
    } else if log_p != 0 {
        Rlog1p(-p)
    } else {
        0.5f64 - p + 0.5f64
    };
}
#[no_mangle]
pub unsafe extern "C" fn qsignrank(
    mut x: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut f: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int || n.is_nan() as i32 != 0 as libc::c_int {
        return x + n;
    }
    if R_finite(x) == 0 || R_finite(n) == 0 {
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
    if log_p != 0 && x > 0 as libc::c_int as libc::c_double
        || log_p == 0
            && (x < 0 as libc::c_int as libc::c_double || x > 1 as libc::c_int as libc::c_double)
    {
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
    n = round(n);
    if n <= 0 as libc::c_int as libc::c_double {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg_1: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_1 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_1 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_1 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_1 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_1, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if x == (if lower_tail != 0 {
        if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 }
    } else {
        if log_p != 0 { 0.0f64 } else { 1.0f64 }
    }) {
        return 0 as libc::c_int as libc::c_double;
    }
    if x == (if lower_tail != 0 {
        if log_p != 0 { 0.0f64 } else { 1.0f64 }
    } else {
        if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 }
    }) {
        return n * (n + 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double;
    }
    if log_p != 0 || lower_tail == 0 {
        x = if log_p != 0 {
            if lower_tail != 0 {
                exp(x)
            } else {
                -expm1(x)
            }
        } else if lower_tail != 0 {
            x
        } else {
            0.5f64 - x + 0.5f64
        };
    }
    let mut nn: libc::c_int = n as libc::c_int;
    w_init_maybe(nn);
    f = exp(-n * 0.693147180559945309417232121458f64);
    p = 0 as libc::c_int as libc::c_double;
    let mut q: libc::c_int = 0 as libc::c_int;
    if x <= 0.5f64 {
        x = x - 10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
        loop {
            p += csignrank(q, nn) * f;
            if p >= x {
                break;
            }
            q += 1;
        }
    } else {
        x = 1 as libc::c_int as libc::c_double - x
            + 10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
        loop {
            p += csignrank(q, nn) * f;
            if p > x {
                q = (n * (n + 1 as libc::c_int as libc::c_double)
                    / 2 as libc::c_int as libc::c_double
                    - q as libc::c_double) as libc::c_int;
                break;
            } else {
                q += 1;
            }
        }
    }
    return q as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rsignrank(mut n: libc::c_double) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut r: libc::c_double = 0.;
    if n.is_nan() as i32 != 0 as libc::c_int {
        return n;
    }
    n = round(n);
    if n < 0 as libc::c_int as libc::c_double {
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
    if n == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    r = 0.0f64;
    k = n as libc::c_int;
    i = 0 as libc::c_int;
    while i < k {
        i += 1;
        r += i as libc::c_double * floor(unif_rand() + 0.5f64);
    }
    return r;
}
