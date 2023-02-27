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
pub unsafe extern "C" fn imax2(mut x: libc::c_int, mut y: libc::c_int) -> libc::c_int {
    return if x < y { y } else { x };
}
