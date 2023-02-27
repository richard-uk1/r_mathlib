pub fn rf_i1mach(i: i32) -> i32 {
    match i {
        1 => 5,
        2 => 6,
        3 => 0,
        4 => 0,
        5 => 8u64.wrapping_mul(::core::mem::size_of::<i32>() as u64) as i32,
        6 => (::core::mem::size_of::<i32>() as u64)
            .wrapping_div(::core::mem::size_of::<u8>() as u64) as i32,
        7 => 2,
        8 => 8u64
            .wrapping_mul(::core::mem::size_of::<i32>() as u64)
            .wrapping_sub(1) as i32,
        9 => 2147483647,
        10 => 2,
        11 => 24,
        12 => -(125),
        13 => 128,
        14 => 53,
        15 => -(1021),
        16 => 1024,
        _ => 0,
    }
}

pub fn i1mach_(i: &i32) -> i32 {
    return rf_i1mach(*i);
}
