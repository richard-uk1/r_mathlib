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
pub unsafe extern "C" fn Rf_gammalims(
    mut xmin: *mut libc::c_double,
    mut xmax: *mut libc::c_double,
) {
    *xmin = -170.5674972726612f64;
    *xmax = 171.61447887182298f64;
}
