use std::sync::atomic::{AtomicI64, Ordering};

use super::{marsaglia_polar_gaussian::MarsagliaPolarGaussian, random_source::{RandomSource, RandomCore}, bits_random_source::BitRandomSource, positional_random_factory::PositionalRandomFactory, math};


pub struct LegacyRandomSource {
    legacy_random_core: LegacyRandomCore,
    gaussian_source: MarsagliaPolarGaussian,
}

impl LegacyRandomSource {
    pub fn new(seed: i64) -> Self {
        Self {
            legacy_random_core: LegacyRandomCore {
                seed: AtomicI64::new(seed),
            },
            gaussian_source: MarsagliaPolarGaussian::new(),
        }
    }
    pub fn from_hash_of(&mut self, seed: &str) -> Self {
        Self {
            legacy_random_core: self.legacy_random_core.clone_from_hash_of(seed),
            gaussian_source: MarsagliaPolarGaussian::new(),
        }
    }
}

impl RandomCore for LegacyRandomSource {
    fn next_int(&mut self) -> i32 {
        self.legacy_random_core.next_int()
    }

    fn next_int_bound(&mut self, bound: i32) -> i32 {
        self.legacy_random_core.next_int_bound(bound)
    }

    fn next_long(&mut self) -> i64 {
        self.legacy_random_core.next_long()
    }

    fn next_boolean(&mut self) -> bool {
        self.legacy_random_core.next_boolean()
    }

    fn next_float(&mut self) -> f32 {
        self.legacy_random_core.next_float()
    }

    fn next_double(&mut self) -> f64 {
        self.legacy_random_core.next_double()
    }
}

impl RandomSource for LegacyRandomSource {
    fn set_seed(&mut self, seed: i64) {
        self.legacy_random_core.set_seed(seed);
        self.gaussian_source.reset();
    }

    fn next_gaussian(&mut self) -> f64 {
        self.gaussian_source.next_gaussian(&mut self.legacy_random_core)
    }
}

impl BitRandomSource for LegacyRandomSource {
    fn next(&mut self, bits: i32) -> i32 {
        self.legacy_random_core.next(bits)
    }
}


struct LegacyRandomCore {
    seed: AtomicI64,
}

impl LegacyRandomCore {
    const FLOAT_MULTIPLIER: f32 = 5.9604645E-8;
    const DOUBLE_MULTIPLIER: f64 = 1.110223E-16;

    fn set_seed(&mut self, seed: i64) {
        self.seed.store((seed ^ 25214903917) & 281474976710655, Ordering::SeqCst);
    }

    fn next(&mut self, bits: i32) -> i32 {
        let l = self.seed.load(Ordering::SeqCst);
        let m = l.wrapping_mul(25214903917).wrapping_add(11) & 281474976710655;
        self.seed.store(m, Ordering::SeqCst);
        (m >> (48 - bits)) as i32
    }
    fn clone_from_hash_of(&mut self, seed: &str) -> Self {
        let i = seed.as_bytes().iter().fold(0, |acc: i64, &x| acc.wrapping_mul(31).wrapping_add(x as i64));
        Self {
            seed: AtomicI64::new(i ^ self.next_long()),
        }
    }
}

impl RandomCore for LegacyRandomCore {


    fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    fn next_int_bound(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("Bound must be positive");
        } else if (bound & (bound - 1)) == 0 {
            ((bound as i64).wrapping_mul(self.next(31) as i64) >> 31) as i32
        } else {
            let mut i;
            let mut j;
            loop {
                i = self.next(31);
                j = i % bound;
                if i - j + (bound - 1) >= 0 {
                    break;
                }
            }
            j
        }
    }

    fn next_long(&mut self) -> i64 {
        let i = self.next(32);
        let j = self.next(32);
        (i as i64) << 32 | (j as i64)
    }

    fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    fn next_float(&mut self) -> f32 {
        self.next(24) as f32 * Self::FLOAT_MULTIPLIER
    }

    fn next_double(&mut self) -> f64 {
        let i = self.next(26);
        let j = self.next(27);
        let l = ((i as i64) << 27) + (j as i64);
        l as f64 * Self::DOUBLE_MULTIPLIER
    }
}

pub struct LegacyPositionalRandomFactory {
    seed: i64,
}

impl LegacyPositionalRandomFactory {
    pub fn new(seed: i64) -> Self {
        Self {
            seed,
        }
    }
}

impl PositionalRandomFactory for LegacyPositionalRandomFactory {
    fn create_from_hash_of(&self, seed: &str) -> Box<dyn RandomSource> {
        let i = seed.as_bytes().iter().fold(0, |acc: i64, &x| acc.wrapping_mul(31).wrapping_add(x as i64));
        Box::new(LegacyRandomSource {
            legacy_random_core: LegacyRandomCore {
                seed: AtomicI64::new(i ^ self.seed),
            },
            gaussian_source: MarsagliaPolarGaussian::new(),
        })
    }

    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        let l = math::get_seed(x, y, z);
        let m = l ^ self.seed;
        Box::new(LegacyRandomSource {
            legacy_random_core: LegacyRandomCore {
                seed: AtomicI64::new(m),
            },
            gaussian_source: MarsagliaPolarGaussian::new(),
        })
    }
}
