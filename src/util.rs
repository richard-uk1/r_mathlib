#![allow(non_snake_case)]

pub fn r_dt_0(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d__0(log_p)
    } else {
        r_d__1(log_p)
    }
}

pub fn r_dt_1(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d__1(log_p)
    } else {
        r_d__0(log_p)
    }
}

fn r_d__0(log_p: bool) -> f64 {
    if log_p {
        f64::NEG_INFINITY
    } else {
        0.
    }
}

fn r_d__1(log_p: bool) -> f64 {
    if log_p {
        0.
    } else {
        1.
    }
}

pub fn r_q_p01_boundaries(
    p: f64,
    left: f64,
    right: f64,
    lower_tail: bool,
    log_p: bool,
) -> Option<f64> {
    if log_p {
        if p > 0. {
            Some(f64::NAN)
        } else if p == 0. {
            if lower_tail {
                Some(right)
            } else {
                Some(left)
            }
        } else if p == f64::NEG_INFINITY {
            if lower_tail {
                Some(left)
            } else {
                Some(right)
            }
        } else {
            None
        }
    } else {
        if p < 0. || p > 1. {
            Some(f64::NAN)
        } else if p == 0. {
            if lower_tail {
                Some(left)
            } else {
                Some(right)
            }
        } else if p == 1. {
            if lower_tail {
                Some(right)
            } else {
                Some(left)
            }
        } else {
            None
        }
    }
}
