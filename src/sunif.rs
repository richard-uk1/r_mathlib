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
    fn log2(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
}
pub type __int64_t = libc::c_long;
pub type __int_least64_t = __int64_t;
pub type int_least64_t = __int_least64_t;
static mut I1: libc::c_uint = 1234 as libc::c_int as libc::c_uint;
static mut I2: libc::c_uint = 5678 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn set_seed(mut i1: libc::c_uint, mut i2: libc::c_uint) {
    I1 = i1;
    I2 = i2;
}
#[no_mangle]
pub unsafe extern "C" fn get_seed(mut i1: *mut libc::c_uint, mut i2: *mut libc::c_uint) {
    *i1 = I1;
    *i2 = I2;
}
#[no_mangle]
pub unsafe extern "C" fn unif_rand() -> libc::c_double {
    I1 = (36969 as libc::c_int as libc::c_uint)
        .wrapping_mul(I1 & 0o177777 as libc::c_int as libc::c_uint)
        .wrapping_add(I1 >> 16 as libc::c_int);
    I2 = (18000 as libc::c_int as libc::c_uint)
        .wrapping_mul(I2 & 0o177777 as libc::c_int as libc::c_uint)
        .wrapping_add(I2 >> 16 as libc::c_int);
    return (I1 << 16 as libc::c_int ^ I2 & 0o177777 as libc::c_int as libc::c_uint)
        as libc::c_double
        * 2.328306437080797e-10f64;
}
unsafe extern "C" fn rbits(mut bits: libc::c_int) -> libc::c_double {
    let mut v: int_least64_t = 0 as libc::c_int as int_least64_t;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n <= bits {
        let mut v1: libc::c_int =
            floor(unif_rand() * 65536 as libc::c_int as libc::c_double) as libc::c_int;
        v = 65536 as libc::c_int as libc::c_long * v + v1 as libc::c_long;
        n += 16 as libc::c_int;
    }
    return (v & ((1 as libc::c_long) << bits) - 1 as libc::c_int as libc::c_long)
        as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn R_unif_index(mut dn: libc::c_double) -> libc::c_double {
    if dn <= 0 as libc::c_int as libc::c_double {
        return 0.0f64;
    }
    let mut bits: libc::c_int = ceil(log2(dn)) as libc::c_int;
    let mut dv: libc::c_double = 0.;
    loop {
        dv = rbits(bits);
        if !(dn <= dv) {
            break;
        }
    }
    return dv;
}
