use r_mathlib::qbinom::qbinom;

fn main() {
    let z = qbinom(0.1, 10., 0.4, false, false);
    println!("z = {z}");
}
