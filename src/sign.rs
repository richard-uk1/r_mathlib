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
pub unsafe extern "C" fn sign(mut x: libc::c_double) -> libc::c_double {
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    return (if x > 0 as libc::c_int as libc::c_double {
        1 as libc::c_int
    } else if x == 0 as libc::c_int as libc::c_double {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as libc::c_double;
}
