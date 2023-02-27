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
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_chebyshev_eval(
        _: libc::c_double,
        _: *const libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn Rlog1p(mut x: libc::c_double) -> libc::c_double {
    static mut alnrcs: [libc::c_double; 43] = [
        0.10378693562743769800686267719098e+1f64,
        -0.13364301504908918098766041553133e+0f64,
        0.19408249135520563357926199374750e-1f64,
        -0.30107551127535777690376537776592e-2f64,
        0.48694614797154850090456366509137e-3f64,
        -0.81054881893175356066809943008622e-4f64,
        0.13778847799559524782938251496059e-4f64,
        -0.23802210894358970251369992914935e-5f64,
        0.41640416213865183476391859901989e-6f64,
        -0.73595828378075994984266837031998e-7f64,
        0.13117611876241674949152294345011e-7f64,
        -0.23546709317742425136696092330175e-8f64,
        0.42522773276034997775638052962567e-9f64,
        -0.77190894134840796826108107493300e-10f64,
        0.14075746481359069909215356472191e-10f64,
        -0.25769072058024680627537078627584e-11f64,
        0.47342406666294421849154395005938e-12f64,
        -0.87249012674742641745301263292675e-13f64,
        0.16124614902740551465739833119115e-13f64,
        -0.29875652015665773006710792416815e-14f64,
        0.55480701209082887983041321697279e-15f64,
        -0.10324619158271569595141333961932e-15f64,
        0.19250239203049851177878503244868e-16f64,
        -0.35955073465265150011189707844266e-17f64,
        0.67264542537876857892194574226773e-18f64,
        -0.12602624168735219252082425637546e-18f64,
        0.23644884408606210044916158955519e-19f64,
        -0.44419377050807936898878389179733e-20f64,
        0.83546594464034259016241293994666e-21f64,
        -0.15731559416479562574899253521066e-21f64,
        0.29653128740247422686154369706666e-22f64,
        -0.55949583481815947292156013226666e-23f64,
        0.10566354268835681048187284138666e-23f64,
        -0.19972483680670204548314999466666e-24f64,
        0.37782977818839361421049855999999e-25f64,
        -0.71531586889081740345038165333333e-26f64,
        0.13552488463674213646502024533333e-26f64,
        -0.25694673048487567430079829333333e-27f64,
        0.48747756066216949076459519999999e-28f64,
        -0.92542112530849715321132373333333e-29f64,
        0.17578597841760239233269760000000e-29f64,
        -0.33410026677731010351377066666666e-30f64,
        0.63533936180236187354180266666666e-31f64,
    ];
    static mut xmin: libc::c_double = -0.999999985f64;
    if x == 0.0f64 {
        return 0.0f64;
    }
    if x == -(1 as libc::c_int) as libc::c_double {
        return -1.0f64 / 0.0f64;
    }
    if x < -(1 as libc::c_int) as libc::c_double {
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
            printf(msg, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if fabs(x) <= 0.375f64 {
        if fabs(x) < 0.5f64 * 2.2204460492503131e-16f64 {
            return x;
        }
        if (0 as libc::c_int as libc::c_double) < x && x < 1e-8f64
            || -1e-9f64 < x && x < 0 as libc::c_int as libc::c_double
        {
            return x * (1 as libc::c_int as libc::c_double - 0.5f64 * x);
        }
        return x
            * (1 as libc::c_int as libc::c_double
                - x * Rf_chebyshev_eval(x / 0.375f64, alnrcs.as_ptr(), 22 as libc::c_int));
    }
    if x < xmin {
        if 8 as libc::c_int > 1 as libc::c_int {
            let mut msg_0: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 8 as libc::c_int {
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
            printf(msg_0, b"log1p\0" as *const u8 as *const libc::c_char);
        }
    }
    return log(1 as libc::c_int as libc::c_double + x);
}
