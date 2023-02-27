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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn qnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmin2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmax2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
pub type N01type = libc::c_uint;
pub const KINDERMAN_RAMAGE: N01type = 5;
pub const INVERSION: N01type = 4;
pub const USER_NORM: N01type = 3;
pub const BOX_MULLER: N01type = 2;
pub const AHRENS_DIETER: N01type = 1;
pub const BUGGY_KINDERMAN_RAMAGE: N01type = 0;
static mut BM_norm_keep: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut N01_kind: N01type = INVERSION;
#[no_mangle]
pub unsafe extern "C" fn norm_rand() -> libc::c_double {
    static mut a: [libc::c_double; 32] = [
        0.0000000f64,
        0.03917609f64,
        0.07841241f64,
        0.1177699f64,
        0.1573107f64,
        0.19709910f64,
        0.23720210f64,
        0.2776904f64,
        0.3186394f64,
        0.36012990f64,
        0.40225010f64,
        0.4450965f64,
        0.4887764f64,
        0.53340970f64,
        0.57913220f64,
        0.6260990f64,
        0.6744898f64,
        0.72451440f64,
        0.77642180f64,
        0.8305109f64,
        0.8871466f64,
        0.94678180f64,
        1.00999000f64,
        1.0775160f64,
        1.1503490f64,
        1.22985900f64,
        1.31801100f64,
        1.4177970f64,
        1.5341210f64,
        1.67594000f64,
        1.86273200f64,
        2.1538750f64,
    ];
    static mut d: [libc::c_double; 31] = [
        0.0000000f64,
        0.0000000f64,
        0.0000000f64,
        0.0000000f64,
        0.0000000f64,
        0.2636843f64,
        0.2425085f64,
        0.2255674f64,
        0.2116342f64,
        0.1999243f64,
        0.1899108f64,
        0.1812252f64,
        0.1736014f64,
        0.1668419f64,
        0.1607967f64,
        0.1553497f64,
        0.1504094f64,
        0.1459026f64,
        0.1417700f64,
        0.1379632f64,
        0.1344418f64,
        0.1311722f64,
        0.1281260f64,
        0.1252791f64,
        0.1226109f64,
        0.1201036f64,
        0.1177417f64,
        0.1155119f64,
        0.1134023f64,
        0.1114027f64,
        0.1095039f64,
    ];
    static mut t: [libc::c_double; 31] = [
        7.673828e-4f64,
        0.002306870f64,
        0.003860618f64,
        0.005438454f64,
        0.007050699f64,
        0.008708396f64,
        0.010423570f64,
        0.012209530f64,
        0.014081250f64,
        0.016055790f64,
        0.018152900f64,
        0.020395730f64,
        0.022811770f64,
        0.025434070f64,
        0.028302960f64,
        0.031468220f64,
        0.034992330f64,
        0.038954830f64,
        0.043458780f64,
        0.048640350f64,
        0.054683340f64,
        0.061842220f64,
        0.070479830f64,
        0.081131950f64,
        0.094624440f64,
        0.112300100f64,
        0.136498000f64,
        0.171688600f64,
        0.227624100f64,
        0.330498000f64,
        0.584703100f64,
    ];
    static mut h: [libc::c_double; 31] = [
        0.03920617f64,
        0.03932705f64,
        0.03950999f64,
        0.03975703f64,
        0.04007093f64,
        0.04045533f64,
        0.04091481f64,
        0.04145507f64,
        0.04208311f64,
        0.04280748f64,
        0.04363863f64,
        0.04458932f64,
        0.04567523f64,
        0.04691571f64,
        0.04833487f64,
        0.04996298f64,
        0.05183859f64,
        0.05401138f64,
        0.05654656f64,
        0.05953130f64,
        0.06308489f64,
        0.06737503f64,
        0.07264544f64,
        0.07926471f64,
        0.08781922f64,
        0.09930398f64,
        0.11555990f64,
        0.14043440f64,
        0.18361420f64,
        0.27900160f64,
        0.70104740f64,
    ];
    static mut A: libc::c_double = 2.216035867166471f64;
    let mut s: libc::c_double = 0.;
    let mut u1: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut u2: libc::c_double = 0.;
    let mut u3: libc::c_double = 0.;
    let mut aa: libc::c_double = 0.;
    let mut tt: libc::c_double = 0.;
    let mut theta: libc::c_double = 0.;
    let mut R: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut current_block_139: u64;
    match N01_kind as libc::c_uint {
        1 => {
            u1 = unif_rand();
            s = 0.0f64;
            if u1 > 0.5f64 {
                s = 1.0f64;
            }
            u1 = u1 + u1 - s;
            u1 *= 32.0f64;
            i = u1 as libc::c_int;
            if i == 32 as libc::c_int {
                i = 31 as libc::c_int;
            }
            if i != 0 as libc::c_int {
                u2 = u1 - i as libc::c_double;
                aa = a[(i - 1 as libc::c_int) as usize];
                's_61: loop {
                    if !(u2 <= t[(i - 1 as libc::c_int) as usize]) {
                        current_block_139 = 15089075282327824602;
                        break;
                    }
                    u1 = unif_rand();
                    w = u1 * (a[i as usize] - aa);
                    tt = (w * 0.5f64 + aa) * w;
                    loop {
                        if u2 > tt {
                            current_block_139 = 1639108787650431521;
                            break 's_61;
                        }
                        u1 = unif_rand();
                        if u2 < u1 {
                            break;
                        }
                        tt = u1;
                        u2 = unif_rand();
                    }
                    u2 = unif_rand();
                }
                match current_block_139 {
                    1639108787650431521 => {}
                    _ => {
                        w = (u2 - t[(i - 1 as libc::c_int) as usize])
                            * h[(i - 1 as libc::c_int) as usize];
                    }
                }
            } else {
                i = 6 as libc::c_int;
                aa = a[31 as libc::c_int as usize];
                loop {
                    u1 = u1 + u1;
                    if u1 >= 1.0f64 {
                        break;
                    }
                    aa = aa + d[(i - 1 as libc::c_int) as usize];
                    i = i + 1 as libc::c_int;
                }
                u1 = u1 - 1.0f64;
                's_155: loop {
                    w = u1 * d[(i - 1 as libc::c_int) as usize];
                    tt = (w * 0.5f64 + aa) * w;
                    loop {
                        u2 = unif_rand();
                        if u2 > tt {
                            break 's_155;
                        }
                        u1 = unif_rand();
                        if u2 < u1 {
                            break;
                        }
                        tt = u1;
                    }
                    u1 = unif_rand();
                }
            }
            y = aa + w;
            return if s == 1.0f64 { -y } else { y };
        }
        0 => {
            u1 = unif_rand();
            if u1 < 0.884070402298758f64 {
                u2 = unif_rand();
                return A * (1.13113163544180f64 * u1 + u2 - 1 as libc::c_int as libc::c_double);
            }
            if u1 >= 0.973310954173898f64 {
                loop {
                    u2 = unif_rand();
                    u3 = unif_rand();
                    tt = A * A - 2 as libc::c_int as libc::c_double * log(u3);
                    if u2 * u2 < A * A / tt {
                        return if u1 < 0.986655477086949f64 {
                            sqrt(tt)
                        } else {
                            -sqrt(tt)
                        };
                    }
                }
            }
            if u1 >= 0.958720824790463f64 {
                loop {
                    u2 = unif_rand();
                    u3 = unif_rand();
                    tt = A - 0.630834801921960f64 * fmin2(u2, u3);
                    if fmax2(u2, u3) <= 0.755591531667601f64 {
                        return if u2 < u3 { tt } else { -tt };
                    }
                    if 0.034240503750111f64 * fabs(u2 - u3)
                        <= 0.398942280401433f64 * exp(-tt * tt / 2.0f64)
                            - 0.180025191068563f64 * (A - tt)
                    {
                        return if u2 < u3 { tt } else { -tt };
                    }
                }
            }
            if u1 >= 0.911312780288703f64 {
                loop {
                    u2 = unif_rand();
                    u3 = unif_rand();
                    tt = 0.479727404222441f64 + 1.105473661022070f64 * fmin2(u2, u3);
                    if fmax2(u2, u3) <= 0.872834976671790f64 {
                        return if u2 < u3 { tt } else { -tt };
                    }
                    if 0.049264496373128f64 * fabs(u2 - u3)
                        <= 0.398942280401433f64 * exp(-tt * tt / 2.0f64)
                            - 0.180025191068563f64 * (A - tt)
                    {
                        return if u2 < u3 { tt } else { -tt };
                    }
                }
            }
            loop {
                u2 = unif_rand();
                u3 = unif_rand();
                tt = 0.479727404222441f64 - 0.595507138015940f64 * fmin2(u2, u3);
                if fmax2(u2, u3) <= 0.805577924423817f64 {
                    return if u2 < u3 { tt } else { -tt };
                }
            }
        }
        2 => {
            if BM_norm_keep != 0.0f64 {
                s = BM_norm_keep;
                BM_norm_keep = 0.0f64;
                return s;
            } else {
                theta = 2 as libc::c_int as libc::c_double
                    * 3.141592653589793238462643383280f64
                    * unif_rand();
                R = sqrt(-(2 as libc::c_int) as libc::c_double * log(unif_rand()))
                    + 10 as libc::c_int as libc::c_double * 2.2250738585072014e-308f64;
                BM_norm_keep = R * sin(theta);
                return R * cos(theta);
            }
        }
        4 => {
            u1 = unif_rand();
            u1 = (134217728 as libc::c_int as libc::c_double * u1) as libc::c_int as libc::c_double
                + unif_rand();
            return qnorm5(
                u1 / 134217728 as libc::c_int as libc::c_double,
                0.0f64,
                1.0f64,
                1 as libc::c_int,
                0 as libc::c_int,
            );
        }
        5 => {
            u1 = unif_rand();
            if u1 < 0.884070402298758f64 {
                u2 = unif_rand();
                return A * (1.131131635444180f64 * u1 + u2 - 1 as libc::c_int as libc::c_double);
            }
            if u1 >= 0.973310954173898f64 {
                loop {
                    u2 = unif_rand();
                    u3 = unif_rand();
                    tt = A * A - 2 as libc::c_int as libc::c_double * log(u3);
                    if u2 * u2 < A * A / tt {
                        return if u1 < 0.986655477086949f64 {
                            sqrt(tt)
                        } else {
                            -sqrt(tt)
                        };
                    }
                }
            }
            if u1 >= 0.958720824790463f64 {
                loop {
                    u2 = unif_rand();
                    u3 = unif_rand();
                    tt = A - 0.630834801921960f64 * fmin2(u2, u3);
                    if fmax2(u2, u3) <= 0.755591531667601f64 {
                        return if u2 < u3 { tt } else { -tt };
                    }
                    if 0.034240503750111f64 * fabs(u2 - u3)
                        <= 0.398942280401433f64 * exp(-tt * tt / 2.0f64)
                            - 0.180025191068563f64 * (A - tt)
                    {
                        return if u2 < u3 { tt } else { -tt };
                    }
                }
            }
            if u1 >= 0.911312780288703f64 {
                loop {
                    u2 = unif_rand();
                    u3 = unif_rand();
                    tt = 0.479727404222441f64 + 1.105473661022070f64 * fmin2(u2, u3);
                    if fmax2(u2, u3) <= 0.872834976671790f64 {
                        return if u2 < u3 { tt } else { -tt };
                    }
                    if 0.049264496373128f64 * fabs(u2 - u3)
                        <= 0.398942280401433f64 * exp(-tt * tt / 2.0f64)
                            - 0.180025191068563f64 * (A - tt)
                    {
                        return if u2 < u3 { tt } else { -tt };
                    }
                }
            }
            loop {
                u2 = unif_rand();
                u3 = unif_rand();
                tt = 0.479727404222441f64 - 0.595507138015940f64 * fmin2(u2, u3);
                if tt < 0.0f64 {
                    continue;
                }
                if fmax2(u2, u3) <= 0.805577924423817f64 {
                    return if u2 < u3 { tt } else { -tt };
                }
                if 0.053377549506886f64 * fabs(u2 - u3)
                    <= 0.398942280401433f64 * exp(-tt * tt / 2.0f64)
                        - 0.180025191068563f64 * (A - tt)
                {
                    return if u2 < u3 { tt } else { -tt };
                }
            }
        }
        _ => {
            printf(
                b"norm_rand(): invalid N01_kind: %d\n\0" as *const u8 as *const libc::c_char,
                N01_kind as libc::c_uint,
            );
            exit(1 as libc::c_int);
        }
    };
}
