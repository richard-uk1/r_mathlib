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
pub unsafe extern "C" fn Rf_d1mach(mut i: libc::c_int) -> libc::c_double {
    match i {
        1 => return 2.2250738585072014e-308f64,
        2 => return 1.7976931348623157e+308f64,
        3 => return 0.5f64 * 2.2204460492503131e-16f64,
        4 => return 2.2204460492503131e-16f64,
        5 => return 0.301029995663981195213738894724f64,
        _ => return 0.0f64,
    };
}
#[no_mangle]
pub unsafe extern "C" fn d1mach_(mut i: *mut libc::c_int) -> libc::c_double {
    return Rf_d1mach(*i);
}
