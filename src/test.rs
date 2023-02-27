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
    fn norm_rand() -> libc::c_double;
    fn set_seed(_: libc::c_uint, _: libc::c_uint);
    fn qnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    static mut N01_kind: libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type C2RustUnnamed = libc::c_uint;
pub const KINDERMAN_RAMAGE: C2RustUnnamed = 5;
pub const INVERSION: C2RustUnnamed = 4;
pub const USER_NORM: C2RustUnnamed = 3;
pub const BOX_MULLER: C2RustUnnamed = 2;
pub const AHRENS_DIETER: C2RustUnnamed = 1;
pub const BUGGY_KINDERMAN_RAMAGE: C2RustUnnamed = 0;
unsafe fn main_0(mut _argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    qnorm5(0.7f64, 0.0f64, 1.0f64, 0 as libc::c_int, 0 as libc::c_int);
    printf(
        b"*** loaded '%s'\n\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    set_seed(
        123 as libc::c_int as libc::c_uint,
        456 as libc::c_int as libc::c_uint,
    );
    N01_kind = AHRENS_DIETER as libc::c_int;
    printf(
        b"one normal %f\n\0" as *const u8 as *const libc::c_char,
        norm_rand(),
    );
    set_seed(
        123 as libc::c_int as libc::c_uint,
        456 as libc::c_int as libc::c_uint,
    );
    N01_kind = BOX_MULLER as libc::c_int;
    printf(
        b"normal via BM %f\n\0" as *const u8 as *const libc::c_char,
        norm_rand(),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
