
// public class PerlinNoise {
//     private static final int ROUND_OFF = 33554432;
//     private final ImprovedNoise[] noiseLevels;
//     private final int firstOctave;
//     private final DoubleList amplitudes;
//     private final double lowestFreqValueFactor;
//     private final double lowestFreqInputFactor;
//     private final double maxValue;
//
//     /** @deprecated */
//     @Deprecated
//     public static PerlinNoise createLegacyForBlendedNoise(RandomSource random, IntStream octaves) {
//         return new PerlinNoise(random, makeAmplitudes(new IntRBTreeSet(octaves.boxed().collect(ImmutableList.toImmutableList()))), false);
//     }
//
//     /** @deprecated */
//     @Deprecated
//     public static PerlinNoise createLegacyForLegacyNetherBiome(RandomSource random, int offset, DoubleList amplitudes) {
//         return new PerlinNoise(random, Pair.of(offset, amplitudes), false);
//     }
//
//     public static PerlinNoise create(RandomSource random, IntStream octaves) {
//         return create(random, octaves.boxed().collect(ImmutableList.toImmutableList()));
//     }
//
//     public static PerlinNoise create(RandomSource random, List<Integer> octaves) {
//         return new PerlinNoise(random, makeAmplitudes(new IntRBTreeSet(octaves)), true);
//     }
//
//     public static PerlinNoise create(RandomSource random, int offset, double firstAmplitude, double... amplitudes) {
//         DoubleArrayList doubleArrayList = new DoubleArrayList(amplitudes);
//         doubleArrayList.add(0, firstAmplitude);
//         return new PerlinNoise(random, Pair.of(offset, doubleArrayList), true);
//     }
//
//     public static PerlinNoise create(RandomSource random, int offset, DoubleList amplitudes) {
//         return new PerlinNoise(random, Pair.of(offset, amplitudes), true);
//     }
//
//     private static Pair<Integer, DoubleList> makeAmplitudes(IntSortedSet octaves) {
//         if (octaves.isEmpty()) {
//             throw new IllegalArgumentException("Need some octaves!");
//         } else {
//             int i = -octaves.firstInt();
//             int j = octaves.lastInt();
//             int k = i + j + 1;
//             if (k < 1) {
//                 throw new IllegalArgumentException("Total number of octaves needs to be >= 1");
//             } else {
//                 DoubleList doubleList = new DoubleArrayList(new double[k]);
//                 IntBidirectionalIterator intBidirectionalIterator = octaves.iterator();
//
//                 while(intBidirectionalIterator.hasNext()) {
//                     int l = intBidirectionalIterator.nextInt();
//                     doubleList.set(l + i, 1.0D);
//                 }
//
//                 return Pair.of(-i, doubleList);
//             }
//         }
//     }
//
//     protected PerlinNoise(RandomSource random, Pair<Integer, DoubleList> firstOctaveAndAmplitudes, boolean xoroshiro) {
//         this.firstOctave = firstOctaveAndAmplitudes.getFirst();
//         this.amplitudes = firstOctaveAndAmplitudes.getSecond();
//         int i = this.amplitudes.size();
//         int j = -this.firstOctave;
//         this.noiseLevels = new ImprovedNoise[i];
//         if (xoroshiro) {
//             PositionalRandomFactory positionalRandomFactory = random.forkPositional();
//
//             for(int k = 0; k < i; ++k) {
//                 if (this.amplitudes.getDouble(k) != 0.0D) {
//                     int l = this.firstOctave + k;
//                     this.noiseLevels[k] = new ImprovedNoise(positionalRandomFactory.fromHashOf("octave_" + l));
//                 }
//             }
//         } else {
//             ImprovedNoise improvedNoise = new ImprovedNoise(random);
//             if (j >= 0 && j < i) {
//                 double d = this.amplitudes.getDouble(j);
//                 if (d != 0.0D) {
//                     this.noiseLevels[j] = improvedNoise;
//                 }
//             }
//
//             for(int m = j - 1; m >= 0; --m) {
//                 if (m < i) {
//                     double e = this.amplitudes.getDouble(m);
//                     if (e != 0.0D) {
//                         this.noiseLevels[m] = new ImprovedNoise(random);
//                     } else {
//                         skipOctave(random);
//                     }
//                 } else {
//                     skipOctave(random);
//                 }
//             }
//
//             if (Arrays.stream(this.noiseLevels).filter(Objects::nonNull).count() != this.amplitudes.stream().filter((amplitude) -> {
//                 return amplitude != 0.0D;
//             }).count()) {
//                 throw new IllegalStateException("Failed to create correct number of noise levels for given non-zero amplitudes");
//             }
//
//             if (j < i - 1) {
//                 throw new IllegalArgumentException("Positive octaves are temporarily disabled");
//             }
//         }
//
//         this.lowestFreqInputFactor = Math.pow(2.0D, (double)(-j));
//         this.lowestFreqValueFactor = Math.pow(2.0D, (double)(i - 1)) / (Math.pow(2.0D, (double)i) - 1.0D);
//         this.maxValue = this.edgeValue(2.0D);
//     }
//
//     protected double maxValue() {
//         return this.maxValue;
//     }
//
//     private static void skipOctave(RandomSource random) {
//         random.consumeCount(262);
//     }
//
//     public double getValue(double x, double y, double z) {
//         return this.getValue(x, y, z, 0.0D, 0.0D, false);
//     }
//
//     /** @deprecated */
//     @Deprecated
//     public double getValue(double x, double y, double z, double yScale, double yMax, boolean useOrigin) {
//         double d = 0.0D;
//         double e = this.lowestFreqInputFactor;
//         double f = this.lowestFreqValueFactor;
//
//         for(int i = 0; i < this.noiseLevels.length; ++i) {
//             ImprovedNoise improvedNoise = this.noiseLevels[i];
//             if (improvedNoise != null) {
//                 double g = improvedNoise.noise(wrap(x * e), useOrigin ? -improvedNoise.yo : wrap(y * e), wrap(z * e), yScale * e, yMax * e);
//                 d += this.amplitudes.getDouble(i) * g * f;
//             }
//
//             e *= 2.0D;
//             f /= 2.0D;
//         }
//
//         return d;
//     }
//
//     public double maxBrokenValue(double d) {
//         return this.edgeValue(d + 2.0D);
//     }
//
//     private double edgeValue(double scale) {
//         double d = 0.0D;
//         double e = this.lowestFreqValueFactor;
//
//         for(int i = 0; i < this.noiseLevels.length; ++i) {
//             ImprovedNoise improvedNoise = this.noiseLevels[i];
//             if (improvedNoise != null) {
//                 d += this.amplitudes.getDouble(i) * scale * e;
//             }
//
//             e /= 2.0D;
//         }
//
//         return d;
//     }
//
//     @Nullable
//     public ImprovedNoise getOctaveNoise(int octave) {
//         return this.noiseLevels[this.noiseLevels.length - 1 - octave];
//     }
//
//     public static double wrap(double value) {
//         return value - (double)Mth.lfloor(value / 3.3554432E7D + 0.5D) * 3.3554432E7D;
//     }
//
//     protected int firstOctave() {
//         return this.firstOctave;
//     }
//
//     protected DoubleList amplitudes() {
//         return this.amplitudes;
//     }
//
//     @VisibleForTesting
//     public void parityConfigString(StringBuilder info) {
//         info.append("PerlinNoise{");
//         List<String> list = this.amplitudes.stream().map((double_) -> {
//             return String.format(Locale.ROOT, "%.2f", double_);
//         }).toList();
//         info.append("first octave: ").append(this.firstOctave).append(", amplitudes: ").append((Object)list).append(", noise levels: [");
//
//         for(int i = 0; i < this.noiseLevels.length; ++i) {
//             info.append(i).append(": ");
//             ImprovedNoise improvedNoise = this.noiseLevels[i];
//             if (improvedNoise == null) {
//                 info.append("null");
//             } else {
//                 improvedNoise.parityConfigString(info);
//             }
//
//             info.append(", ");
//         }
//
//         info.append("]");
//         info.append("}");
//     }
// }

use super::improved_noise::ImprovedNoise;


#[derive(Debug)]
pub struct PerlinNoise {
    noise_levels: Vec<Option<ImprovedNoise>>,
    _first_octave: i32,
    amplitudes: Vec<f64>,
    lowest_freq_input_factor: f64,
    lowest_freq_value_factor: f64,
    max_value: f64,
}

impl PerlinNoise {
    pub fn new(noise_levels: Vec<Option<ImprovedNoise>>, first_octave: i32, amplitudes: Vec<f64>, lowest_freq_input_factor: f64, lowest_freq_value_factor: f64, max_value: f64) -> Self {
        Self {
            noise_levels,
            _first_octave: first_octave,
            amplitudes,
            lowest_freq_input_factor,
            lowest_freq_value_factor,
            max_value,
        }
    }

    pub(super) fn max_value(&self) -> f64 {
        self.max_value
    }

    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        self.get_value_with_origin(x, y, z, 0.0, 0.0, false)
    }

    pub fn get_value_with_origin(&self, x: f64, y: f64, z: f64, y_scale: f64, y_max: f64, use_origin: bool) -> f64 {
        let mut d = 0.0;
        let mut e = self.lowest_freq_input_factor;
        let mut f = self.lowest_freq_value_factor;

        for i in 0..self.noise_levels.len() {
            let improved_noise = &self.noise_levels[i];
            if let Some(improved_noise) = improved_noise {
                let g = improved_noise.noise_scaled(wrap(x * e), if use_origin { -improved_noise.yo } else { wrap(y * e) }, wrap(z * e), y_scale * e, y_max * e);
                d += self.amplitudes[i] * g * f;
            }

            e *= 2.0;
            f /= 2.0;
        }

        d
    }
}

const ROUND_OFF: f64 = 33554432.0;

fn wrap(value: f64) -> f64 {
    value - (value / ROUND_OFF + 0.5).floor() * ROUND_OFF
}
