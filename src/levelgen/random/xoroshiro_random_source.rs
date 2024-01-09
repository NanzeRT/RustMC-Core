
// public class XoroshiroRandomSource implements RandomSource {
//     private static final float FLOAT_UNIT = 5.9604645E-8F;
//     private static final double DOUBLE_UNIT = (double)1.110223E-16F;
//     public static final Codec<XoroshiroRandomSource> CODEC = Xoroshiro128PlusPlus.CODEC.xmap((implementation) -> {
//         return new XoroshiroRandomSource(implementation);
//     }, (random) -> {
//         return random.randomNumberGenerator;
//     });
//     private Xoroshiro128PlusPlus randomNumberGenerator;
//     private final MarsagliaPolarGaussian gaussianSource = new MarsagliaPolarGaussian(this);
//
//     public XoroshiroRandomSource(long seed) {
//         this.randomNumberGenerator = new Xoroshiro128PlusPlus(RandomSupport.upgradeSeedTo128bit(seed));
//     }
//
//     public XoroshiroRandomSource(RandomSupport.Seed128bit seed) {
//         this.randomNumberGenerator = new Xoroshiro128PlusPlus(seed);
//     }
//
//     public XoroshiroRandomSource(long seedLo, long seedHi) {
//         this.randomNumberGenerator = new Xoroshiro128PlusPlus(seedLo, seedHi);
//     }
//
//     private XoroshiroRandomSource(Xoroshiro128PlusPlus implementation) {
//         this.randomNumberGenerator = implementation;
//     }
//
//     @Override
//     public RandomSource fork() {
//         return new XoroshiroRandomSource(this.randomNumberGenerator.nextLong(), this.randomNumberGenerator.nextLong());
//     }
//
//     @Override
//     public PositionalRandomFactory forkPositional() {
//         return new XoroshiroRandomSource.XoroshiroPositionalRandomFactory(this.randomNumberGenerator.nextLong(), this.randomNumberGenerator.nextLong());
//     }
//
//     @Override
//     public void setSeed(long seed) {
//         this.randomNumberGenerator = new Xoroshiro128PlusPlus(RandomSupport.upgradeSeedTo128bit(seed));
//         this.gaussianSource.reset();
//     }
//
//     @Override
//     public int nextInt() {
//         return (int)this.randomNumberGenerator.nextLong();
//     }
//
//     @Override
//     public int nextInt(int bound) {
//         if (bound <= 0) {
//             throw new IllegalArgumentException("Bound must be positive");
//         } else {
//             long l = Integer.toUnsignedLong(this.nextInt());
//             long m = l * (long)bound;
//             long n = m & 4294967295L;
//             if (n < (long)bound) {
//                 for(int i = Integer.remainderUnsigned(~bound + 1, bound); n < (long)i; n = m & 4294967295L) {
//                     l = Integer.toUnsignedLong(this.nextInt());
//                     m = l * (long)bound;
//                 }
//             }
//
//             long o = m >> 32;
//             return (int)o;
//         }
//     }
//
//     @Override
//     public long nextLong() {
//         return this.randomNumberGenerator.nextLong();
//     }
//
//     @Override
//     public boolean nextBoolean() {
//         return (this.randomNumberGenerator.nextLong() & 1L) != 0L;
//     }
//
//     @Override
//     public float nextFloat() {
//         return (float)this.nextBits(24) * 5.9604645E-8F;
//     }
//
//     @Override
//     public double nextDouble() {
//         return (double)this.nextBits(53) * (double)1.110223E-16F;
//     }
//
//     @Override
//     public double nextGaussian() {
//         return this.gaussianSource.nextGaussian();
//     }
//
//     @Override
//     public void consumeCount(int count) {
//         for(int i = 0; i < count; ++i) {
//             this.randomNumberGenerator.nextLong();
//         }
//
//     }
//
//     private long nextBits(int bits) {
//         return this.randomNumberGenerator.nextLong() >>> 64 - bits;
//     }
//
//     public static class XoroshiroPositionalRandomFactory implements PositionalRandomFactory {
//         private final long seedLo;
//         private final long seedHi;
//
//         public XoroshiroPositionalRandomFactory(long seedLo, long seedHi) {
//             this.seedLo = seedLo;
//             this.seedHi = seedHi;
//         }
//
//         @Override
//         public RandomSource at(int x, int y, int z) {
//             long l = Mth.getSeed(x, y, z);
//             long m = l ^ this.seedLo;
//             return new XoroshiroRandomSource(m, this.seedHi);
//         }
//
//         @Override
//         public RandomSource fromHashOf(String seed) {
//             RandomSupport.Seed128bit seed128bit = RandomSupport.seedFromHashOf(seed);
//             return new XoroshiroRandomSource(seed128bit.xor(this.seedLo, this.seedHi));
//         }
//
//         @VisibleForTesting
//         @Override
//         public void parityConfigString(StringBuilder info) {
//             info.append("seedLo: ").append(this.seedLo).append(", seedHi: ").append(this.seedHi);
//         }
//     }
// }


// public class Xoroshiro128PlusPlus {
//     private long seedLo;
//     private long seedHi;
//     public static final Codec<Xoroshiro128PlusPlus> CODEC = Codec.LONG_STREAM.comapFlatMap((stream) -> {
//         return Util.fixedSize(stream, 2).map((seeds) -> {
//             return new Xoroshiro128PlusPlus(seeds[0], seeds[1]);
//         });
//     }, (random) -> {
//         return LongStream.of(random.seedLo, random.seedHi);
//     });
//
//     public Xoroshiro128PlusPlus(RandomSupport.Seed128bit seed) {
//         this(seed.seedLo(), seed.seedHi());
//     }
//
//     public Xoroshiro128PlusPlus(long seedLo, long seedHi) {
//         this.seedLo = seedLo;
//         this.seedHi = seedHi;
//         if ((this.seedLo | this.seedHi) == 0L) {
//             this.seedLo = -7046029254386353131L;
//             this.seedHi = 7640891576956012809L;
//         }
//
//     }
//
//     public long nextLong() {
//         long l = this.seedLo;
//         long m = this.seedHi;
//         long n = Long.rotateLeft(l + m, 17) + l;
//         m ^= l;
//         this.seedLo = Long.rotateLeft(l, 49) ^ m ^ m << 21;
//         this.seedHi = Long.rotateLeft(m, 28);
//         return n;
//     }
// }

use super::{marsaglia_polar_gaussian::MarsagliaPolarGaussian, random_source::{RandomCore, RandomSource}, math::get_seed, random_support::seed_from_hash_of, positional_random_factory::PositionalRandomFactory};

pub struct XoroshiroRandomSource {
    random_number_generator: XoroshiroRandomCore,
    gaussian_source: MarsagliaPolarGaussian,
}

struct  XoroshiroRandomCore {
    random_number_generator: Xoroshiro128PlusPlus,
}

struct Xoroshiro128PlusPlus {
    seed_lo: i64,
    seed_hi: i64,
}

impl Xoroshiro128PlusPlus {
    fn new(seed: (i64, i64)) -> Self {
        Self {
            seed_lo: seed.0,
            seed_hi: seed.1,
        }
    }

    fn next_long(&mut self) -> i64 {
        let l = self.seed_lo;
        let m = self.seed_hi;
        let n = l.wrapping_add(m).rotate_left(17).wrapping_add(l);
        let m = m ^ l;
        self.seed_lo = l.rotate_left(49) ^ m ^ m.rotate_left(21);
        self.seed_hi = m.rotate_left(28);
        n
    }
}

impl XoroshiroRandomCore {
    const FLOAT_UNIT: f32 = 5.9604645E-8;
    const DOUBLE_UNIT: f64 = 1.110223E-16;

    fn new(seed: (i64, i64)) -> Self {
        Self {
            random_number_generator: Xoroshiro128PlusPlus::new(seed),
        }
    }

    fn next_bits(&mut self, bits: i32) -> i64 {
        self.random_number_generator.next_long() >> (64 - bits)
    }
}

impl RandomCore for XoroshiroRandomCore {
    fn next_int(&mut self) -> i32 {
        self.random_number_generator.next_long() as i32
    }

    fn next_int_bound(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("Bound must be positive");
        }
        let mut l = self.next_int() as i64;
        let mut m = l * bound as i64;
        let n = m & 4294967295;
        if n < bound as i64 {
            let i = !bound + 1;
            for _ in 0..i {
                l = self.next_int() as i64;
                m = l * bound as i64;
            }
        }
        let o = m >> 32;
        o as i32
    }

    fn next_long(&mut self) -> i64 {
        self.random_number_generator.next_long()
    }

    fn next_boolean(&mut self) -> bool {
        (self.random_number_generator.next_long() & 1) != 0
    }

    fn next_float(&mut self) -> f32 {
        self.next_bits(24) as f32 * Self::FLOAT_UNIT
    }

    fn next_double(&mut self) -> f64 {
        self.next_bits(53) as f64 * Self::DOUBLE_UNIT
    }
}

impl XoroshiroRandomSource {
    fn new(seed: (i64, i64)) -> Self {
        Self {
            random_number_generator: XoroshiroRandomCore::new(seed),
            gaussian_source: MarsagliaPolarGaussian::new(),
        }
    }
}

impl RandomCore for XoroshiroRandomSource {
    fn next_int(&mut self) -> i32 {
        self.random_number_generator.next_int()
    }

    fn next_int_bound(&mut self, bound: i32) -> i32 {
        self.random_number_generator.next_int_bound(bound)
    }

    fn next_long(&mut self) -> i64 {
        self.random_number_generator.next_long()
    }

    fn next_boolean(&mut self) -> bool {
        self.random_number_generator.next_boolean()
    }

    fn next_float(&mut self) -> f32 {
        self.random_number_generator.next_float()
    }

    fn next_double(&mut self) -> f64 {
        self.random_number_generator.next_double()
    }

    fn consume_count(&mut self, count: i32) {
        for _ in 0..count {
            self.random_number_generator.next_long();
        }
    }
}

impl RandomSource for XoroshiroRandomSource {
    fn set_seed(&mut self, seed: i64) {
        self.random_number_generator = XoroshiroRandomCore::new((seed, seed));
        self.gaussian_source.reset();
    }

    fn next_gaussian(&mut self) -> f64 {
        self.gaussian_source.next_gaussian(&mut self.random_number_generator)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct XoroshiroPositionalRandomFactory {
    seed_lo: i64,
    seed_hi: i64,
}

impl XoroshiroPositionalRandomFactory {
    pub fn new(seed: (i64, i64)) -> Self {
        Self {
            seed_lo: seed.0,
            seed_hi: seed.1,
        }
    }
}

impl PositionalRandomFactory for XoroshiroPositionalRandomFactory {
    type Target = XoroshiroRandomSource;
    fn at(&self, x: i32, y: i32, z: i32) -> Self::Target {
        let l = get_seed(x, y, z);
        let m = l ^ self.seed_lo;
        XoroshiroRandomSource::new((m, self.seed_hi))
    }

    fn create_from_hash_of(&self, seed: &str) -> Self::Target {
        let seed128bit = seed_from_hash_of(seed);
        XoroshiroRandomSource::new((self.seed_lo ^ seed128bit.0, self.seed_hi ^ seed128bit.1))
    }
}
