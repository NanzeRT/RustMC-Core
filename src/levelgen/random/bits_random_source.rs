use super::random_source::RandomSource;

pub trait BitRandomSource: RandomSource {

    fn next(&mut self, bits: i32) -> i32;

    // fn next_int(&mut self) -> i32 {
    //     self.next(32)
    // }

    // fn next_int_bound(&mut self, bound: i32) -> i32 {
    //     if bound <= 0 {
    //         panic!("Bound must be positive");
    //     } else if (bound & (bound - 1)) == 0 {
    //         ((bound as i64).wrapping_mul(self.next(31) as i64) >> 31) as i32
    //     } else {
    //         let mut i;
    //         let mut j;
    //         loop {
    //             i = self.next(31);
    //             j = i % bound;
    //             if i - j + (bound - 1) >= 0 {
    //                 break;
    //             }
    //         }
    //         j
    //     }
    // }

    // fn next_long(&mut self) -> i64 {
    //     let i = self.next(32);
    //     let j = self.next(32);
    //     (i as i64) << 32 | (j as i64)
    // }

    // fn next_boolean(&mut self) -> bool {
    //     self.next(1) != 0
    // }

    // fn next_float(&mut self) -> f32 {
    //     self.next(24) as f32 * Self::FLOAT_MULTIPLIER
    // }

    // fn next_double(&mut self) -> f64 {
    //     let i = self.next(26);
    //     let j = self.next(27);
    //     let l = ((i as i64) << 27) + (j as i64);
    //     l as f64 * Self::DOUBLE_MULTIPLIER
    // }
}
