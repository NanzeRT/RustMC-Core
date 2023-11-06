pub fn get_seed(x: i32, y: i32, z: i32) -> i64 {
    let l = (x as i64 * 3129871) ^ (z as i64 * 116129781) ^ (y as i64);
    // (l * l * 42317861 + l * 11) >> 16
    (l.wrapping_mul(l).wrapping_mul(42317861).wrapping_add(l.wrapping_mul(11))) >> 16
}


