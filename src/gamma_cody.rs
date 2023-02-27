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
    fn trunc(_: libc::c_double) -> libc::c_double;
    fn sinpi(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn Rf_gamma_cody(mut x: libc::c_double) -> libc::c_double {
    static mut sqrtpi: libc::c_double = 0.9189385332046727417803297f64;
    static mut xbig: libc::c_double = 171.624f64;
    static mut p: [libc::c_double; 8] = [
        -1.71618513886549492533811f64,
        24.7656508055759199108314f64,
        -379.804256470945635097577f64,
        629.331155312818442661052f64,
        866.966202790413211295064f64,
        -31451.2729688483675254357f64,
        -36144.4134186911729807069f64,
        66456.1438202405440627855f64,
    ];
    static mut q: [libc::c_double; 8] = [
        -30.8402300119738975254353f64,
        315.350626979604161529144f64,
        -1015.15636749021914166146f64,
        -3107.77167157231109440444f64,
        22538.1184209801510330112f64,
        4755.84627752788110767815f64,
        -134659.959864969306392456f64,
        -115132.259675553483497211f64,
    ];
    static mut c: [libc::c_double; 7] = [
        -0.001910444077728f64,
        8.4171387781295e-4f64,
        -5.952379913043012e-4f64,
        7.93650793500350248e-4f64,
        -0.002777777777777681622553f64,
        0.08333333333333333331554247f64,
        0.0057083835261f64,
    ];
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut parity: libc::c_int = 0;
    let mut fact: libc::c_double = 0.;
    let mut xden: libc::c_double = 0.;
    let mut xnum: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut z: libc::c_double = 0.;
    let mut yi: libc::c_double = 0.;
    let mut res: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut ysq: libc::c_double = 0.;
    parity = 0 as libc::c_int;
    fact = 1.0f64;
    n = 0 as libc::c_int;
    y = x;
    if y <= 0.0f64 {
        y = -x;
        yi = trunc(y);
        res = y - yi;
        if res != 0.0f64 {
            if yi != trunc(yi * 0.5f64) * 2.0f64 {
                parity = 1 as libc::c_int;
            }
            fact = -3.141592653589793238462643383280f64 / sinpi(res);
            y += 1.0f64;
        } else {
            return 1.0f64 / 0.0f64;
        }
    }
    if y < 2.2204460492503131e-16f64 {
        if y >= 2.2250738585072014e-308f64 {
            res = 1.0f64 / y;
        } else {
            return 1.0f64 / 0.0f64;
        }
    } else if y < 12.0f64 {
        yi = y;
        if y < 1.0f64 {
            z = y;
            y += 1.0f64;
        } else {
            n = y as libc::c_int - 1 as libc::c_int;
            y -= n as libc::c_double;
            z = y - 1.0f64;
        }
        xnum = 0.0f64;
        xden = 1.0f64;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            xnum = (xnum + p[i as usize]) * z;
            xden = xden * z + q[i as usize];
            i += 1;
        }
        res = xnum / xden + 1.0f64;
        if yi < y {
            res /= yi;
        } else if yi > y {
            i = 0 as libc::c_int;
            while i < n {
                res *= y;
                y += 1.0f64;
                i += 1;
            }
        }
    } else if y <= xbig {
        ysq = y * y;
        sum = c[6 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            sum = sum / ysq + c[i as usize];
            i += 1;
        }
        sum = sum / y - y + sqrtpi;
        sum += (y - 0.5f64) * log(y);
        res = exp(sum);
    } else {
        return 1.0f64 / 0.0f64;
    }
    if parity != 0 {
        res = -res;
    }
    if fact != 1.0f64 {
        res = fact / res;
    }
    return res;
}
