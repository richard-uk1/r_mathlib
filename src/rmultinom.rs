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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn rbinom(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rmultinom(
    mut n: libc::c_int,
    mut prob: *mut libc::c_double,
    mut K: libc::c_int,
    mut rN: *mut libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut pp: libc::c_double = 0.;
    let mut p_tot: libc::c_double = 0.0f64;
    if K < 1 as libc::c_int {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg = b"argument out of domain in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                2 => {
                    msg = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg, b"rmultinom\0" as *const u8 as *const libc::c_char);
        }
        return;
    }
    if n < 0 as libc::c_int {
        if 1 as libc::c_int > 1 as libc::c_int {
            let mut msg_0: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 1 as libc::c_int {
                1 => {
                    msg_0 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_0 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_0 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_0 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_0 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_0, b"rmultinom\0" as *const u8 as *const libc::c_char);
        }
        *rN.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        return;
    }
    k = 0 as libc::c_int;
    while k < K {
        pp = *prob.offset(k as isize);
        if R_finite(pp) == 0 || pp < 0.0f64 || pp > 1.0f64 {
            if 1 as libc::c_int > 1 as libc::c_int {
                let mut msg_1: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 1 as libc::c_int {
                    1 => {
                        msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_1 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_1 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_1 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_1 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_1, b"rmultinom\0" as *const u8 as *const libc::c_char);
            }
            *rN.offset(k as isize) = -(1 as libc::c_int);
            return;
        }
        p_tot += pp;
        *rN.offset(k as isize) = 0 as libc::c_int;
        k += 1;
    }
    if fabs(p_tot - 1.0f64) > 1e-7f64 {
        printf(
            b"rbinom: probability sum should be 1, but is %g\0" as *const u8 as *const libc::c_char,
            p_tot,
        );
        exit(1 as libc::c_int);
    }
    if n == 0 as libc::c_int {
        return;
    }
    if K == 1 as libc::c_int && p_tot == 0.0f64 {
        return;
    }
    k = 0 as libc::c_int;
    while k < K - 1 as libc::c_int {
        if *prob.offset(k as isize) != 0.0f64 {
            pp = *prob.offset(k as isize) / p_tot;
            *rN.offset(k as isize) = if pp < 1.0f64 {
                rbinom(n as libc::c_double, pp) as libc::c_int
            } else {
                n
            };
            n -= *rN.offset(k as isize);
        } else {
            *rN.offset(k as isize) = 0 as libc::c_int;
        }
        if n <= 0 as libc::c_int {
            return;
        }
        p_tot -= *prob.offset(k as isize);
        k += 1;
    }
    *rN.offset((K - 1 as libc::c_int) as isize) = n;
}
