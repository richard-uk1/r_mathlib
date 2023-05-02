use crate::{pbinom::pbinom, qnorm::qnorm5, util::r_q_p01_boundaries};

pub type C2RustUnnamed = libc::c_uint;
fn do_search(mut y: f64, z: &mut f64, p: f64, n: f64, pr: f64, incr: f64) -> f64 {
    if *z >= p {
        loop {
            let mut newz = 0.;
            if y == 0. || {
                newz = pbinom(y - incr, n, pr, true, false);
                newz < p
            } {
                return y;
            }
            y = (y - incr).max(0.);
            *z = newz;
        }
    } else {
        loop {
            y = (y + incr).min(n);
            if y == n || {
                *z = pbinom(y, n, pr, true, false);
                *z >= p
            } {
                return y;
            }
        }
    };
}

pub fn qbinom(mut p: f64, n: f64, pr: f64, lower_tail: bool, log_p: bool) -> f64 {
    let mut z;
    let mut y;
    if p.is_nan() || n.is_nan() || pr.is_nan() {
        return p + n + pr;
    }
    if !n.is_finite() || !pr.is_finite() {
        return f64::NAN;
    }
    if !p.is_finite() && !log_p {
        return f64::NAN;
    }
    if n != (n + 0.5).floor() {
        return f64::NAN;
    }
    if pr < 0. || pr > 1. || n < 0. {
        return f64::NAN;
    }

    if let Some(ret) = r_q_p01_boundaries(p, 0., n, lower_tail, log_p) {
        return ret;
    }
    if pr == 0. || n == 0. {
        return 0.;
    }
    if pr == 1. {
        return n;
    }

    let q = 1. - pr;
    let mu = n * pr;
    let sigma = (n * pr * q).sqrt();
    let gamma = (q - pr) / sigma;
    if !lower_tail || log_p {
        p = if log_p {
            if lower_tail {
                p.exp()
            } else {
                -p.exp_m1()
            }
        } else if lower_tail {
            p
        } else {
            0.5 - p + 0.5
        }f64;
        if p == 0.0 {
            return 0.0;
        }
        if p == 1.0 {
            return n;
        }
    }
    if p + 1.01 * 2.2204460492503131e-16 >= 1.0 {
        return n;
    }
    z = qnorm5(p, 0.0, 1.0, true, false);
    y = (mu + sigma * (z + gamma * (z * z - 1.) / 6.) + 0.5).floor();
    if y > n {
        y = n;
    }
    z = pbinom(y, n, pr, true, false);
    p *= 1. - 64. * 2.2204460492503131e-16;
    if n < 1e5 {
        return do_search(y, &mut z, p, n, pr, 1.);
    }
    let mut incr = (n * 0.001).floor();
    let mut oldincr;
    loop {
        oldincr = incr;
        y = do_search(y, &mut z, p, n, pr, incr);
        incr = (incr / 100.).floor().max(1.);
        if !(oldincr > 1. && incr > n * 1e-15) {
            break;
        }
    }
    return y;
}

fn q_discrete_01_checks(p: f64, lower_tail: bool, log_p: bool) {
    // this has some checks in R source that we're gonna skip
}

fn q_discrete_body(p: f64, lower_tail: bool, log_p: bool) {
    // TODO from here
    //let z = qnorm(p, 0., 1., lower_tail, log_p);
    //let y = mu + sigma * (z + gamma * (z * z - 1) / 6);
}

#[cfg(test)]
mod tests {
    #[test]
    fn qbinom() {
        let tests = [
            //(0.1, 10., 0.4, true, false, 2.),
            (0.4, 5., 0.7, true, false, 3),
            (0.025, 465., 0.018, true, false, 3),
            (0.975, 465., 0.018, true, false, 14),
            (0.025, 465., 0.17, true, false, 64),
            (0.975, 465., 0.17, true, false, 95),
            (0.025, 465., 0.103, true, false, 34),
            (0.975, 465., 0.103, true, false, 60),
            (0.0003787879, 465., 0.103, true, false, 27),
            (0.9996212, 465., 0.103, true, false, 71),
            (0.9996212, 465., 0.103, false, false, 27),
            (0.975, 465., 0.051, true, false, 33),
        ];
        for (p, size, prob, lower_tail, log_p, output) in tests {
            let res = super::pbinom(p, size, prob, lower_tail, log_p) as usize;
            assert_eq!(res, output);
        }
    }
}
