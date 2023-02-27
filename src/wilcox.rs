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
    fn R_unif_index(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn imax2(_: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn lchoose(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn choose(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
static mut w: *mut *mut *mut libc::c_double =
    0 as *const *mut *mut libc::c_double as *mut *mut *mut libc::c_double;
static mut allocated_m: libc::c_int = 0;
static mut allocated_n: libc::c_int = 0;
unsafe extern "C" fn w_free(mut m: libc::c_int, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = m;
    while i >= 0 as libc::c_int {
        j = n;
        while j >= 0 as libc::c_int {
            if !(*(*w.offset(i as isize)).offset(j as isize)).is_null() {
                free(*(*w.offset(i as isize)).offset(j as isize) as *mut libc::c_void);
            }
            j -= 1;
        }
        free(*w.offset(i as isize) as *mut libc::c_void);
        i -= 1;
    }
    free(w as *mut libc::c_void);
    w = 0 as *mut *mut *mut libc::c_double;
    allocated_n = 0 as libc::c_int;
    allocated_m = allocated_n;
}
unsafe extern "C" fn w_init_maybe(mut m: libc::c_int, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    if m > n {
        i = n;
        n = m;
        m = i;
    }
    if !w.is_null() && (m > allocated_m || n > allocated_n) {
        w_free(allocated_m, allocated_n);
    }
    if w.is_null() {
        m = imax2(m, 50 as libc::c_int);
        n = imax2(n, 50 as libc::c_int);
        w = calloc(
            (m as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::core::mem::size_of::<*mut *mut libc::c_double>() as libc::c_ulong,
        ) as *mut *mut *mut libc::c_double;
        if w.is_null() {
            printf(
                b"wilcox allocation error %d\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i <= m {
            let ref mut fresh0 = *w.offset(i as isize);
            *fresh0 = calloc(
                (n as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
                ::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
            ) as *mut *mut libc::c_double;
            if (*w.offset(i as isize)).is_null() {
                w_free(i - 1 as libc::c_int, n);
                printf(
                    b"wilcox allocation error %d\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
        }
        allocated_m = m;
        allocated_n = n;
    }
}
unsafe extern "C" fn w_free_maybe(mut m: libc::c_int, mut n: libc::c_int) {
    if m > 50 as libc::c_int || n > 50 as libc::c_int {
        w_free(m, n);
    }
}
unsafe extern "C" fn cwilcox(
    mut k: libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
) -> libc::c_double {
    let mut c: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    u = m * n;
    if k < 0 as libc::c_int || k > u {
        return 0 as libc::c_int as libc::c_double;
    }
    c = u / 2 as libc::c_int;
    if k > c {
        k = u - k;
    }
    if m < n {
        i = m;
        j = n;
    } else {
        i = n;
        j = m;
    }
    if j == 0 as libc::c_int {
        return (k == 0 as libc::c_int) as libc::c_int as libc::c_double;
    }
    if j > 0 as libc::c_int && k < j {
        return cwilcox(k, i, k);
    }
    if (*(*w.offset(i as isize)).offset(j as isize)).is_null() {
        let ref mut fresh1 = *(*w.offset(i as isize)).offset(j as isize);
        *fresh1 = calloc(
            (c as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        ) as *mut libc::c_double;
        if (*(*w.offset(i as isize)).offset(j as isize)).is_null() {
            printf(
                b"wilcox allocation error %d\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int,
            );
            exit(1 as libc::c_int);
        }
        l = 0 as libc::c_int;
        while l <= c {
            *(*(*w.offset(i as isize)).offset(j as isize)).offset(l as isize) =
                -(1 as libc::c_int) as libc::c_double;
            l += 1;
        }
    }
    if *(*(*w.offset(i as isize)).offset(j as isize)).offset(k as isize)
        < 0 as libc::c_int as libc::c_double
    {
        if j == 0 as libc::c_int {
            *(*(*w.offset(i as isize)).offset(j as isize)).offset(k as isize) =
                (k == 0 as libc::c_int) as libc::c_int as libc::c_double;
        } else {
            *(*(*w.offset(i as isize)).offset(j as isize)).offset(k as isize) =
                cwilcox(k - j, i - 1 as libc::c_int, j) + cwilcox(k, i, j - 1 as libc::c_int);
        }
    }
    return *(*(*w.offset(i as isize)).offset(j as isize)).offset(k as isize);
}
#[no_mangle]
pub unsafe extern "C" fn dwilcox(
    mut x: libc::c_double,
    mut m: libc::c_double,
    mut n: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || m.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return x + m + n;
    }
    m = round(m);
    n = round(n);
    if m <= 0 as libc::c_int as libc::c_double || n <= 0 as libc::c_int as libc::c_double {
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
    if x < 0 as libc::c_int as libc::c_double || x > m * n {
        return if log_p != 0 { -1.0f64 / 0.0f64 } else { 0.0f64 };
    }
    let mut mm: libc::c_int = m as libc::c_int;
    let mut nn: libc::c_int = n as libc::c_int;
    let mut xx: libc::c_int = x as libc::c_int;
    w_init_maybe(mm, nn);
    d = if log_p != 0 {
        log(cwilcox(xx, mm, nn)) - lchoose(m + n, n)
    } else {
        cwilcox(xx, mm, nn) / choose(m + n, n)
    };
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn pwilcox(
    mut q: libc::c_double,
    mut m: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    if q.is_nan() as i32 != 0 as libc::c_int
        || m.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return q + m + n;
    }
    if R_finite(m) == 0 || R_finite(n) == 0 {
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
    m = round(m);
    n = round(n);
    if m <= 0 as libc::c_int as libc::c_double || n <= 0 as libc::c_int as libc::c_double {
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
    q = floor(q + 1e-7f64);
    if q < 0.0f64 {
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
    if q >= m * n {
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
    let mut mm: libc::c_int = m as libc::c_int;
    let mut nn: libc::c_int = n as libc::c_int;
    w_init_maybe(mm, nn);
    c = choose(m + n, n);
    p = 0 as libc::c_int as libc::c_double;
    if q <= m * n / 2 as libc::c_int as libc::c_double {
        i = 0 as libc::c_int;
        while i as libc::c_double <= q {
            p += cwilcox(i, mm, nn) / c;
            i += 1;
        }
    } else {
        q = m * n - q;
        i = 0 as libc::c_int;
        while (i as libc::c_double) < q {
            p += cwilcox(i, mm, nn) / c;
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
pub unsafe extern "C" fn qwilcox(
    mut x: libc::c_double,
    mut m: libc::c_double,
    mut n: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    let mut c: libc::c_double = 0.;
    let mut p: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int
        || m.is_nan() as i32 != 0 as libc::c_int
        || n.is_nan() as i32 != 0 as libc::c_int
    {
        return x + m + n;
    }
    if R_finite(x) == 0 || R_finite(m) == 0 || R_finite(n) == 0 {
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
    m = round(m);
    n = round(n);
    if m <= 0 as libc::c_int as libc::c_double || n <= 0 as libc::c_int as libc::c_double {
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
        if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        }
    } else {
        if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        }
    }) {
        return 0 as libc::c_int as libc::c_double;
    }
    if x == (if lower_tail != 0 {
        if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        }
    } else {
        if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        }
    }) {
        return m * n;
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
    let mut mm: libc::c_int = m as libc::c_int;
    let mut nn: libc::c_int = n as libc::c_int;
    w_init_maybe(mm, nn);
    c = choose(m + n, n);
    p = 0 as libc::c_int as libc::c_double;
    let mut q: libc::c_int = 0 as libc::c_int;
    if x <= 0.5f64 {
        x = x - 10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
        loop {
            p += cwilcox(q, mm, nn) / c;
            if p >= x {
                break;
            }
            q += 1;
        }
    } else {
        x = 1 as libc::c_int as libc::c_double - x
            + 10 as libc::c_int as libc::c_double * 2.2204460492503131e-16f64;
        loop {
            p += cwilcox(q, mm, nn) / c;
            if p > x {
                q = (m * n - q as libc::c_double) as libc::c_int;
                break;
            } else {
                q += 1;
            }
        }
    }
    return q as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn rwilcox(mut m: libc::c_double, mut n: libc::c_double) -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut r: libc::c_double = 0.;
    if m.is_nan() as i32 != 0 as libc::c_int || n.is_nan() as i32 != 0 as libc::c_int {
        return m + n;
    }
    m = round(m);
    n = round(n);
    if m < 0 as libc::c_int as libc::c_double || n < 0 as libc::c_int as libc::c_double {
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
    if m == 0 as libc::c_int as libc::c_double || n == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    r = 0.0f64;
    k = (m + n) as libc::c_int;
    x = calloc(
        k as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if x.is_null() {
        printf(
            b"wilcox allocation error %d\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
        );
        exit(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < k {
        *x.offset(i as isize) = i;
        i += 1;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_double) < n {
        j = R_unif_index(k as libc::c_double) as libc::c_int;
        r += *x.offset(j as isize) as libc::c_double;
        k -= 1;
        *x.offset(j as isize) = *x.offset(k as isize);
        i += 1;
    }
    free(x as *mut libc::c_void);
    return r - n * (n - 1 as libc::c_int as libc::c_double) / 2 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn wilcox_free() {
    w_free_maybe(allocated_m, allocated_n);
}
