
// pub struct BlockPos {
//     x: i32,
//     y: i32,
//     z: i32,
// }

pub fn as_long(x: i32, y: i32, z: i32) -> i64 {
    ((x as i64 & 67108863) << 38) | (y as i64 & 4095) | ((z as i64 & 67108863) << 12)
}

pub fn get_x_long(packed_pos: i64) -> i32 {
    (packed_pos >> 38) as i32
}

pub fn get_y_long(packed_pos: i64) -> i32 {
    ((packed_pos << 52) >> 52) as i32
}

pub fn get_z_long(packed_pos: i64) -> i32 {
    ((packed_pos << 26) >> 38) as i32
}
