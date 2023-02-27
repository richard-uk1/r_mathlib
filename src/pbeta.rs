use crate::{
    toms708::rf_bratio,
    util::{r_dt_0, r_dt_1},
};

pub fn pbeta_raw(x: f64, a: f64, b: f64, lower_tail: bool, log_p: bool) -> f64 {
    if a == 0. || b == 0. || !a.is_finite() || !b.is_finite() {
        if a == 0. && b == 0. {
            return if log_p {
                -0.693147180559945309417232121458 // M_LN2
            } else {
                0.5
            };
        }
        if a == 0. || a / b == 0. {
            return r_dt_1(lower_tail, log_p);
        }
        if b == 0. || b / a == 0. {
            return r_dt_0(lower_tail, log_p);
        }
        if x < 0.5 {
            return r_dt_0(lower_tail, log_p);
        } else {
            return r_dt_1(lower_tail, log_p);
        }
    }
    if x >= 1. {
        return r_dt_1(lower_tail, log_p);
    }

    let x1 = 0.5 - x + 0.5;
    let mut w = 0.;
    let mut wc = 0.;
    let mut ierr = 0;
    rf_bratio(a, b, x, x1, &mut w, &mut wc, &mut ierr, log_p);
    if ierr != 0 && ierr != 11 && ierr != 14 {
        println!(
            "pbeta_raw({}, a={}, b={}, ..) -> bratio() gave error code {}",
            x, a, b, ierr,
        );
    }
    return if lower_tail { w } else { wc };
}

pub fn pbeta(x: f64, a: f64, b: f64, lower_tail: bool, log_p: bool) -> f64 {
    if x.is_nan() || a.is_nan() || b.is_nan() {
        return x + a + b;
    }
    if a < 0. || b < 0. {
        return f64::NAN;
    }

    if x <= 0. {
        return r_dt_0(lower_tail, log_p);
    }
    return pbeta_raw(x, a, b, lower_tail, log_p);
}

#[cfg(test)]
mod tests {
    #[test]
    fn pbeta() {
        let tests = [
            //(0.1, 10., 0.4, true, false, 2.),
            (0.1, 2., 4., true, false, 0.08146),
            (0.4, 7., 14., false, false, 0.25),
            (0.4, 7., 14., true, false, 0.75),
        ];
        for (x, a, b, lower_tail, log_p, output) in tests {
            let res = super::pbeta(x, a, b, lower_tail, log_p);
            let err = (res - output).abs();
            assert!(err < 1e7, "abs({res} - {output}) = {} </ 1e7", err);
        }
    }
}
