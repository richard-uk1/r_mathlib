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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn Rlog1p(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_finite(_: libc::c_double) -> libc::c_int;
    fn pnorm5(
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_double,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_double;
    fn lgammafn(_: libc::c_double) -> libc::c_double;
}
unsafe extern "C" fn wprob(
    mut w: libc::c_double,
    mut rr: libc::c_double,
    mut cc: libc::c_double,
) -> libc::c_double {
    static mut C1: libc::c_double = -30.0f64;
    static mut C2: libc::c_double = -50.0f64;
    static mut C3: libc::c_double = 60.0f64;
    static mut bb: libc::c_double = 8.0f64;
    static mut wlar: libc::c_double = 3.0f64;
    static mut wincr1: libc::c_double = 2.0f64;
    static mut wincr2: libc::c_double = 3.0f64;
    static mut xleg: [libc::c_double; 6] = [
        0.981560634246719250690549090149f64,
        0.904117256370474856678465866119f64,
        0.769902674194304687036893833213f64,
        0.587317954286617447296702418941f64,
        0.367831498998180193752691536644f64,
        0.125233408511468915472441369464f64,
    ];
    static mut aleg: [libc::c_double; 6] = [
        0.047175336386511827194615961485f64,
        0.106939325995318430960254718194f64,
        0.160078328543346226334652529543f64,
        0.203167426723065921749064455810f64,
        0.233492536538354808760849898925f64,
        0.249147045813402785000562436043f64,
    ];
    let mut a: libc::c_double = 0.;
    let mut ac: libc::c_double = 0.;
    let mut pr_w: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut binc: libc::c_double = 0.;
    let mut c: libc::c_double = 0.;
    let mut cc1: libc::c_double = 0.;
    let mut pminus: libc::c_double = 0.;
    let mut pplus: libc::c_double = 0.;
    let mut qexpo: libc::c_double = 0.;
    let mut qsqz: libc::c_double = 0.;
    let mut rinsum: libc::c_double = 0.;
    let mut wi: libc::c_double = 0.;
    let mut wincr: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut blb: libc::c_double = 0.;
    let mut bub: libc::c_double = 0.;
    let mut einsum: libc::c_double = 0.;
    let mut elsum: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    qsqz = w * 0.5f64;
    if qsqz >= bb {
        return 1.0f64;
    }
    pr_w = 2 as libc::c_int as libc::c_double
        * pnorm5(qsqz, 0.0f64, 1.0f64, 1 as libc::c_int, 0 as libc::c_int)
        - 1.0f64;
    if pr_w >= exp(C2 / cc) {
        pr_w = pow(pr_w, cc);
    } else {
        pr_w = 0.0f64;
    }
    if w > wlar {
        wincr = wincr1;
    } else {
        wincr = wincr2;
    }
    blb = qsqz;
    binc = (bb - qsqz) / wincr;
    bub = blb + binc;
    einsum = 0.0f64;
    cc1 = cc - 1.0f64;
    wi = 1 as libc::c_int as libc::c_double;
    while wi <= wincr {
        elsum = 0.0f64;
        a = 0.5f64 * (bub + blb);
        b = 0.5f64 * (bub - blb);
        jj = 1 as libc::c_int;
        while jj <= 12 as libc::c_int {
            if (6 as libc::c_int) < jj {
                j = 12 as libc::c_int - jj + 1 as libc::c_int;
                xx = xleg[(j - 1 as libc::c_int) as usize];
            } else {
                j = jj;
                xx = -xleg[(j - 1 as libc::c_int) as usize];
            }
            c = b * xx;
            ac = a + c;
            qexpo = ac * ac;
            if qexpo > C3 {
                break;
            }
            pplus = 2 as libc::c_int as libc::c_double
                * pnorm5(ac, 0.0f64, 1.0f64, 1 as libc::c_int, 0 as libc::c_int);
            pminus = 2 as libc::c_int as libc::c_double
                * pnorm5(ac, w, 1.0f64, 1 as libc::c_int, 0 as libc::c_int);
            rinsum = pplus * 0.5f64 - pminus * 0.5f64;
            if rinsum >= exp(C1 / cc1) {
                rinsum = aleg[(j - 1 as libc::c_int) as usize]
                    * exp(-(0.5f64 * qexpo))
                    * pow(rinsum, cc1);
                elsum += rinsum;
            }
            jj += 1;
        }
        elsum *= 2.0f64 * b * cc * 0.398942280401432677939946059934f64;
        einsum += elsum;
        blb = bub;
        bub += binc;
        wi += 1.;
    }
    pr_w += einsum;
    if pr_w <= exp(C1 / rr) {
        return 0.0f64;
    }
    pr_w = pow(pr_w, rr);
    if pr_w >= 1.0f64 {
        return 1.0f64;
    }
    return pr_w;
}
#[no_mangle]
pub unsafe extern "C" fn ptukey(
    mut q: libc::c_double,
    mut rr: libc::c_double,
    mut cc: libc::c_double,
    mut df: libc::c_double,
    mut lower_tail: libc::c_int,
    mut log_p: libc::c_int,
) -> libc::c_double {
    static mut eps1: libc::c_double = -30.0f64;
    static mut eps2: libc::c_double = 1.0e-14f64;
    static mut dhaf: libc::c_double = 100.0f64;
    static mut dquar: libc::c_double = 800.0f64;
    static mut deigh: libc::c_double = 5000.0f64;
    static mut dlarg: libc::c_double = 25000.0f64;
    static mut ulen1: libc::c_double = 1.0f64;
    static mut ulen2: libc::c_double = 0.5f64;
    static mut ulen3: libc::c_double = 0.25f64;
    static mut ulen4: libc::c_double = 0.125f64;
    static mut xlegq: [libc::c_double; 8] = [
        0.989400934991649932596154173450f64,
        0.944575023073232576077988415535f64,
        0.865631202387831743880467897712f64,
        0.755404408355003033895101194847f64,
        0.617876244402643748446671764049f64,
        0.458016777657227386342419442984f64,
        0.281603550779258913230460501460f64,
        0.950125098376374401853193354250e-1f64,
    ];
    static mut alegq: [libc::c_double; 8] = [
        0.271524594117540948517805724560e-1f64,
        0.622535239386478928628438369944e-1f64,
        0.951585116824927848099251076022e-1f64,
        0.124628971255533872052476282192f64,
        0.149595988816576732081501730547f64,
        0.169156519395002538189312079030f64,
        0.182603415044923588866763667969f64,
        0.189450610455068496285396723208f64,
    ];
    let mut ans: libc::c_double = 0.;
    let mut f2: libc::c_double = 0.;
    let mut f21: libc::c_double = 0.;
    let mut f2lf: libc::c_double = 0.;
    let mut ff4: libc::c_double = 0.;
    let mut otsum: libc::c_double = 0.;
    let mut qsqz: libc::c_double = 0.;
    let mut rotsum: libc::c_double = 0.;
    let mut t1: libc::c_double = 0.;
    let mut twa1: libc::c_double = 0.;
    let mut ulen: libc::c_double = 0.;
    let mut wprb: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut jj: libc::c_int = 0;
    if q.is_nan() as i32 != 0 as libc::c_int
        || rr.is_nan() as i32 != 0 as libc::c_int
        || cc.is_nan() as i32 != 0 as libc::c_int
        || df.is_nan() as i32 != 0 as libc::c_int
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
            printf(msg, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if q <= 0 as libc::c_int as libc::c_double {
        return if lower_tail != 0 {
            if log_p != 0 {
                -1.0f64 / 0.0f64
            } else {
                0.0f64
            }
        } else if log_p != 0 {
            0.0f64
        } else {
            1.0f64
        };
    }
    if df < 2 as libc::c_int as libc::c_double
        || rr < 1 as libc::c_int as libc::c_double
        || cc < 2 as libc::c_int as libc::c_double
    {
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
            printf(msg_0, b"\0" as *const u8 as *const libc::c_char);
        }
        return 0.0f64 / 0.0f64;
    }
    if R_finite(q) == 0 {
        return if lower_tail != 0 {
            if log_p != 0 {
                0.0f64
            } else {
                1.0f64
            }
        } else if log_p != 0 {
            -1.0f64 / 0.0f64
        } else {
            0.0f64
        };
    }
    if df > dlarg {
        return if lower_tail != 0 {
            if log_p != 0 {
                log(wprob(q, rr, cc))
            } else {
                wprob(q, rr, cc)
            }
        } else if log_p != 0 {
            Rlog1p(-wprob(q, rr, cc))
        } else {
            0.5f64 - wprob(q, rr, cc) + 0.5f64
        };
    }
    f2 = df * 0.5f64;
    f2lf = f2 * log(df) - df * 0.693147180559945309417232121458f64 - lgammafn(f2);
    f21 = f2 - 1.0f64;
    ff4 = df * 0.25f64;
    if df <= dhaf {
        ulen = ulen1;
    } else if df <= dquar {
        ulen = ulen2;
    } else if df <= deigh {
        ulen = ulen3;
    } else {
        ulen = ulen4;
    }
    f2lf += log(ulen);
    ans = 0.0f64;
    i = 1 as libc::c_int;
    while i <= 50 as libc::c_int {
        otsum = 0.0f64;
        twa1 = (2 as libc::c_int * i - 1 as libc::c_int) as libc::c_double * ulen;
        jj = 1 as libc::c_int;
        while jj <= 16 as libc::c_int {
            if (8 as libc::c_int) < jj {
                j = jj - 8 as libc::c_int - 1 as libc::c_int;
                t1 = f2lf + f21 * log(twa1 + xlegq[j as usize] * ulen)
                    - (xlegq[j as usize] * ulen + twa1) * ff4;
            } else {
                j = jj - 1 as libc::c_int;
                t1 = f2lf
                    + f21 * log(twa1 - xlegq[j as usize] * ulen)
                    + (xlegq[j as usize] * ulen - twa1) * ff4;
            }
            if t1 >= eps1 {
                if (8 as libc::c_int) < jj {
                    qsqz = q * sqrt((xlegq[j as usize] * ulen + twa1) * 0.5f64);
                } else {
                    qsqz = q * sqrt((-(xlegq[j as usize] * ulen) + twa1) * 0.5f64);
                }
                wprb = wprob(qsqz, rr, cc);
                rotsum = wprb * alegq[j as usize] * exp(t1);
                otsum += rotsum;
            }
            jj += 1;
        }
        if i as libc::c_double * ulen >= 1.0f64 && otsum <= eps2 {
            break;
        }
        ans += otsum;
        i += 1;
    }
    if otsum > eps2 {
        if 8 as libc::c_int > 1 as libc::c_int {
            let mut msg_1: *mut libc::c_char =
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            match 8 as libc::c_int {
                1 => {
                    msg_1 = b"argument out of domain in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                2 => {
                    msg_1 = b"value out of range in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    msg_1 = b"convergence failed in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    msg_1 = b"full precision may not have been achieved in '%s'\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    msg_1 = b"underflow occurred in '%s'\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            printf(msg_1, b"ptukey\0" as *const u8 as *const libc::c_char);
        }
    }
    if ans > 1.0f64 {
        ans = 1.0f64;
    }
    return if lower_tail != 0 {
        if log_p != 0 {
            log(ans)
        } else {
            ans
        }
    } else if log_p != 0 {
        Rlog1p(-ans)
    } else {
        0.5f64 - ans + 0.5f64
    };
}
