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
pub unsafe extern "C" fn Rf_i1mach(mut i: libc::c_int) -> libc::c_int {
    match i {
        1 => return 5 as libc::c_int,
        2 => return 6 as libc::c_int,
        3 => return 0 as libc::c_int,
        4 => return 0 as libc::c_int,
        5 => {
            return (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int;
        }
        6 => {
            return (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                as libc::c_int;
        }
        7 => return 2 as libc::c_int,
        8 => {
            return (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        9 => return 2147483647 as libc::c_int,
        10 => return 2 as libc::c_int,
        11 => return 24 as libc::c_int,
        12 => return -(125 as libc::c_int),
        13 => return 128 as libc::c_int,
        14 => return 53 as libc::c_int,
        15 => return -(1021 as libc::c_int),
        16 => return 1024 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn i1mach_(mut i: *mut libc::c_int) -> libc::c_int {
    return Rf_i1mach(*i);
}
