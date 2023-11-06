pub trait RandomSource: RandomCore {
    fn set_seed(&mut self, seed: i64);
    fn next_gaussian(&mut self) -> f64;
    fn triangle(&mut self, mode: f64, deviation: f64) -> f64 {
        mode + deviation * (self.next_double() - self.next_double())
    }
}

pub trait RandomCore {
    fn next_int(&mut self) -> i32;
    fn next_int_bound(&mut self, bound: i32) -> i32;
    fn next_int_between_inclusive(&mut self, min: i32, max: i32) -> i32 {
        self.next_int_bound(max - min + 1) + min
    }
    fn next_long(&mut self) -> i64;
    fn next_boolean(&mut self) -> bool;
    fn next_float(&mut self) -> f32;
    fn next_double(&mut self) -> f64;
    fn consume_count(&mut self, count: i32) {
        for _ in 0..count {
            self.next_int();
        }
    }
    fn next_int_between(&mut self, min: i32, max: i32) -> i32 {
        if min >= max {
            panic!("bound - origin is non positive");
        }
        min + self.next_int_bound(max - min)
    }
}
