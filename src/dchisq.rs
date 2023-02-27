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
    fn dgamma(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn dchisq(
    mut x: libc::c_double,
    mut df: libc::c_double,
    mut log_p: libc::c_int,
) -> libc::c_double {
    return dgamma(x, df / 2.0f64, 2.0f64, log_p);
}
