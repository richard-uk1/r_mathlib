#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn R_finite(mut x: libc::c_double) -> libc::c_int {
    return (x.is_nan() as i32 == 0) as libc::c_int
        & (x != 1.0f64 / 0.0f64) as libc::c_int
        & (x != -1.0f64 / 0.0f64) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_isnancpp(mut x: libc::c_double) -> libc::c_int {
    return (x.is_nan() as i32 != 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn myfmod(mut x1: libc::c_double, mut x2: libc::c_double) -> libc::c_double {
    let mut q: libc::c_double = x1 / x2;
    return x1 - q.floor() * x2;
}
#[no_mangle]
pub unsafe extern "C" fn R_pow(mut x: libc::c_double, mut y: libc::c_double) -> libc::c_double {
    if x == 1.0f64 || y == 0.0f64 {
        return 1.0f64;
    }
    if x == 0.0f64 {
        if y > 0.0f64 {
            return 0.0f64;
        }
        return 1.0f64 / 0.0f64;
    }
    if R_finite(x) != 0 && R_finite(y) != 0 {
        return x.powf(y);
    }
    if x.is_nan() as i32 != 0 as libc::c_int || y.is_nan() as i32 != 0 as libc::c_int {
        return x + y;
    }
    if R_finite(x) == 0 {
        if x > 0 as libc::c_int as libc::c_double {
            return if y < 0.0f64 { 0.0f64 } else { 1.0f64 / 0.0f64 };
        } else {
            if R_finite(y) != 0 && y == y.floor() {
                return if y < 0.0f64 {
                    0.0f64
                } else if myfmod(y, 2.0f64) != 0. {
                    x
                } else {
                    -x
                };
            }
        }
    }
    if R_finite(y) == 0 {
        if x >= 0 as libc::c_int as libc::c_double {
            if y > 0 as libc::c_int as libc::c_double {
                return if x >= 1 as libc::c_int as libc::c_double {
                    1.0f64 / 0.0f64
                } else {
                    0.0f64
                };
            } else {
                return if x < 1 as libc::c_int as libc::c_double {
                    1.0f64 / 0.0f64
                } else {
                    0.0f64
                };
            }
        }
    }
    return 0.0f64 / 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn R_pow_di(mut x: libc::c_double, mut n: libc::c_int) -> libc::c_double {
    let mut pow_0: libc::c_double = 1.0f64;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if n != 0 as libc::c_int {
        if R_finite(x) == 0 {
            return R_pow(x, n as libc::c_double);
        }
        if n < 0 as libc::c_int {
            n = -n;
            x = 1 as libc::c_int as libc::c_double / x;
        }
        loop {
            if n & 0o1 as libc::c_int != 0 {
                pow_0 *= x;
            }
            n >>= 1 as libc::c_int;
            if !(n != 0) {
                break;
            }
            x *= x;
        }
    }
    return pow_0;
}
#[no_mangle]
pub static mut NA_REAL: libc::c_double = 0.0f64 / 0.0f64;
#[no_mangle]
pub static mut R_PosInf: libc::c_double = 1.0f64 / 0.0f64;
#[no_mangle]
pub static mut R_NegInf: libc::c_double = -1.0f64 / 0.0f64;
