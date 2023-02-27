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
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn Rf_lgammacor(_: libc::c_double) -> libc::c_double;
    fn Rf_stirlerr(_: libc::c_double) -> libc::c_double;
    fn Rf_chebyshev_eval(
        _: libc::c_double,
        _: *const libc::c_double,
        _: libc::c_int,
    ) -> libc::c_double;
    fn sinpi(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gammafn(mut x: libc::c_double) -> libc::c_double {
    static mut gamcs: [libc::c_double; 42] = [
        0.8571195590989331421920062399942e-2f64,
        0.4415381324841006757191315771652e-2f64,
        0.5685043681599363378632664588789e-1f64,
        -0.4219835396418560501012500186624e-2f64,
        0.1326808181212460220584006796352e-2f64,
        -0.1893024529798880432523947023886e-3f64,
        0.3606925327441245256578082217225e-4f64,
        -0.6056761904460864218485548290365e-5f64,
        0.1055829546302283344731823509093e-5f64,
        -0.1811967365542384048291855891166e-6f64,
        0.3117724964715322277790254593169e-7f64,
        -0.5354219639019687140874081024347e-8f64,
        0.9193275519859588946887786825940e-9f64,
        -0.1577941280288339761767423273953e-9f64,
        0.2707980622934954543266540433089e-10f64,
        -0.4646818653825730144081661058933e-11f64,
        0.7973350192007419656460767175359e-12f64,
        -0.1368078209830916025799499172309e-12f64,
        0.2347319486563800657233471771688e-13f64,
        -0.4027432614949066932766570534699e-14f64,
        0.6910051747372100912138336975257e-15f64,
        -0.1185584500221992907052387126192e-15f64,
        0.2034148542496373955201026051932e-16f64,
        -0.3490054341717405849274012949108e-17f64,
        0.5987993856485305567135051066026e-18f64,
        -0.1027378057872228074490069778431e-18f64,
        0.1762702816060529824942759660748e-19f64,
        -0.3024320653735306260958772112042e-20f64,
        0.5188914660218397839717833550506e-21f64,
        -0.8902770842456576692449251601066e-22f64,
        0.1527474068493342602274596891306e-22f64,
        -0.2620731256187362900257328332799e-23f64,
        0.4496464047830538670331046570666e-24f64,
        -0.7714712731336877911703901525333e-25f64,
        0.1323635453126044036486572714666e-25f64,
        -0.2270999412942928816702313813333e-26f64,
        0.3896418998003991449320816639999e-27f64,
        -0.6685198115125953327792127999999e-28f64,
        0.1146998663140024384347613866666e-28f64,
        -0.1967938586345134677295103999999e-29f64,
        0.3376448816585338090334890666666e-30f64,
        -0.5793070335782135784625493333333e-31f64,
    ];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut y: libc::c_double = 0.;
    let mut sinpiy: libc::c_double = 0.;
    let mut value: libc::c_double = 0.;
    if x.is_nan() as i32 != 0 as libc::c_int {
        return x;
    }
    if x == 0 as libc::c_int as libc::c_double
        || x < 0 as libc::c_int as libc::c_double && x == round(x)
    {
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
            printf(msg, b"gammafn\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    y = fabs(x);
    if y <= 10 as libc::c_int as libc::c_double {
        n = x as libc::c_int;
        if x < 0 as libc::c_int as libc::c_double {
            n -= 1;
        }
        y = x - n as libc::c_double;
        n -= 1;
        value = Rf_chebyshev_eval(
            y * 2 as libc::c_int as libc::c_double - 1 as libc::c_int as libc::c_double,
            gamcs.as_ptr(),
            22 as libc::c_int,
        ) + 0.9375f64;
        if n == 0 as libc::c_int {
            return value;
        }
        if n < 0 as libc::c_int {
            if x < -0.5f64
                && fabs(x - (x - 0.5f64) as libc::c_int as libc::c_double / x)
                    < 1.490116119384765696e-8f64
            {
                if 8 as libc::c_int > 1 as libc::c_int {
                    let mut msg_0: *mut libc::c_char =
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    match 8 as libc::c_int {
                        1 => {
                            msg_0 = b"argument out of domain in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        2 => {
                            msg_0 = b"value out of range in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        4 => {
                            msg_0 = b"convergence failed in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        8 => {
                            msg_0 = b"full precision may not have been achieved in '%s'\n\0"
                                as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        16 => {
                            msg_0 = b"underflow occurred in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        _ => {}
                    }
                    printf(msg_0, b"gammafn\0" as *const u8 as *const libc::c_char);
                }
            }
            if y < 2.2474362225598545e-308f64 {
                if 2 as libc::c_int > 1 as libc::c_int {
                    let mut msg_1: *mut libc::c_char =
                        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                    match 2 as libc::c_int {
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
                                as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        16 => {
                            msg_1 = b"underflow occurred in '%s'\n\0" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char;
                        }
                        _ => {}
                    }
                    printf(msg_1, b"gammafn\0" as *const u8 as *const libc::c_char);
                }
                if x > 0 as libc::c_int as libc::c_double {
                    return 1.0f64 / 0.0f64;
                } else {
                    return -1.0f64 / 0.0f64;
                }
            }
            n = -n;
            i = 0 as libc::c_int;
            while i < n {
                value /= x + i as libc::c_double;
                i += 1;
            }
            return value;
        } else {
            i = 1 as libc::c_int;
            while i <= n {
                value *= y + i as libc::c_double;
                i += 1;
            }
            return value;
        }
    } else {
        if x > 171.61447887182298f64 {
            return 1.0f64 / 0.0f64;
        }
        if x < -170.5674972726612f64 {
            return 0.0f64;
        }
        if y <= 50 as libc::c_int as libc::c_double && y == y as libc::c_int as libc::c_double {
            value = 1.0f64;
            i = 2 as libc::c_int;
            while (i as libc::c_double) < y {
                value *= i as libc::c_double;
                i += 1;
            }
        } else {
            value = exp((y - 0.5f64) * log(y) - y
                + 0.918938533204672741780329736406f64
                + (if 2 as libc::c_int as libc::c_double * y
                    == 2 as libc::c_int as libc::c_double * y
                {
                    Rf_stirlerr(y)
                } else {
                    Rf_lgammacor(y)
                }));
        }
        if x > 0 as libc::c_int as libc::c_double {
            return value;
        }
        if fabs((x - (x - 0.5f64) as libc::c_int as libc::c_double) / x)
            < 1.490116119384765696e-8f64
        {
            if 8 as libc::c_int > 1 as libc::c_int {
                let mut msg_2: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 8 as libc::c_int {
                    1 => {
                        msg_2 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_2 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_2 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_2 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_2 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_2, b"gammafn\0" as *const u8 as *const libc::c_char);
            }
        }
        sinpiy = sinpi(y);
        if sinpiy == 0 as libc::c_int as libc::c_double {
            if 2 as libc::c_int > 1 as libc::c_int {
                let mut msg_3: *mut libc::c_char =
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                match 2 as libc::c_int {
                    1 => {
                        msg_3 = b"argument out of domain in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    2 => {
                        msg_3 = b"value out of range in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    4 => {
                        msg_3 = b"convergence failed in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    8 => {
                        msg_3 = b"full precision may not have been achieved in '%s'\n\0"
                            as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    16 => {
                        msg_3 = b"underflow occurred in '%s'\n\0" as *const u8
                            as *const libc::c_char
                            as *mut libc::c_char;
                    }
                    _ => {}
                }
                printf(msg_3, b"gammafn\0" as *const u8 as *const libc::c_char);
            }
            return 1.0f64 / 0.0f64;
        }
        return -3.141592653589793238462643383280f64 / (y * sinpiy * value);
    };
}
