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
    fn trunc(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn ftrunc(mut x: libc::c_double) -> libc::c_double {
    return trunc(x);
}
