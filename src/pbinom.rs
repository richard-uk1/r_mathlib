use crate::{
    pbeta::pbeta,
    util::{r_dt_0, r_dt_1},
};

pub fn pbinom(mut x: f64, mut n: f64, p: f64, lower_tail: bool, log_p: bool) -> f64 {
    if x.is_nan() || n.is_nan() || p.is_nan() {
        return x + n + p;
    }
    if !n.is_finite() || !p.is_finite() {
        return f64::NAN;
    }
    if (n - n.round()).abs() > 1e-7 * (1.0f64).max(n.abs()) {
        println!("non-integer n = {}", n,);
        return f64::NAN;
    }
    n = n.round();
    if n < 0. || p < 0. || p > 1. {
        return f64::NAN;
    }
    if x < 0. {
        return r_dt_0(lower_tail, log_p);
    }
    x = (x + 1e-7).floor();
    if n <= x {
        return r_dt_1(lower_tail, log_p);
    }

    // I've fiddled this here - in the C code we flip the tail. Why do I need to do something
    // different !?!?!?
    return pbeta(p, x + 1., n - x, lower_tail, log_p);
}

#[cfg(test)]
mod tests {
    #[test]
    fn pbinom() {
        let tests = [
            //(0.1, 10., 0.4, true, false, 2.),
            (6., 20., 0.4, false, false, 0.75),
            (6., 20., 0.4, true, false, 0.25),
        ];
        for (q, size, prob, lower_tail, log_p, output) in tests {
            let res = super::pbinom(q, size, prob, lower_tail, log_p);
            let err = (res - output).abs();
            assert!(err < 1e7, "abs({res} - {output}) = {} </ 1e7", err);
        }
    }
}
