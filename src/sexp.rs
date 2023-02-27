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
    fn unif_rand() -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn exp_rand() -> libc::c_double {
    static mut q: [libc::c_double; 16] = [
        0.6931471805599453f64,
        0.9333736875190459f64,
        0.9888777961838675f64,
        0.9984959252914960f64,
        0.9998292811061389f64,
        0.9999833164100727f64,
        0.9999985691438767f64,
        0.9999998906925558f64,
        0.9999999924734159f64,
        0.9999999995283275f64,
        0.9999999999728814f64,
        0.9999999999985598f64,
        0.9999999999999289f64,
        0.9999999999999968f64,
        0.9999999999999999f64,
        1.0000000000000000f64,
    ];
    let mut a: libc::c_double = 0.0f64;
    let mut u: libc::c_double = unif_rand();
    while u <= 0.0f64 || u >= 1.0f64 {
        u = unif_rand();
    }
    loop {
        u += u;
        if u > 1.0f64 {
            break;
        }
        a += q[0 as libc::c_int as usize];
    }
    u -= 1.0f64;
    if u <= q[0 as libc::c_int as usize] {
        return a + u;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ustar: libc::c_double = unif_rand();
    let mut umin: libc::c_double = ustar;
    loop {
        ustar = unif_rand();
        if umin > ustar {
            umin = ustar;
        }
        i += 1;
        if !(u > q[i as usize]) {
            break;
        }
    }
    return a + umin * q[0 as libc::c_int as usize];
}
