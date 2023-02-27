#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn fmin2(mut x: libc::c_double, mut y: libc::c_double) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int || y.is_nan() as i32 != 0 as libc::c_int {
        return x + y;
    }
    return if x < y { x } else { y };
}
