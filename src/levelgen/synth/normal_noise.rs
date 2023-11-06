
// public class NormalNoise {
//     private static final double INPUT_FACTOR = 1.0181268882175227D;
//     private static final double TARGET_DEVIATION = 0.3333333333333333D;
//     private final double valueFactor;
//     private final PerlinNoise first;
//     private final PerlinNoise second;
//     private final double maxValue;
//     private final NormalNoise.NoiseParameters parameters;
//
//     /** @deprecated */
//     @Deprecated
//     public static NormalNoise createLegacyNetherBiome(RandomSource random, NormalNoise.NoiseParameters parameters) {
//         return new NormalNoise(random, parameters, false);
//     }
//
//     public static NormalNoise create(RandomSource random, int offset, double... octaves) {
//         return create(random, new NormalNoise.NoiseParameters(offset, new DoubleArrayList(octaves)));
//     }
//
//     public static NormalNoise create(RandomSource random, NormalNoise.NoiseParameters parameters) {
//         return new NormalNoise(random, parameters, true);
//     }
//
//     private NormalNoise(RandomSource random, NormalNoise.NoiseParameters parameters, boolean modern) {
//         int i = parameters.firstOctave;
//         DoubleList doubleList = parameters.amplitudes;
//         this.parameters = parameters;
//         if (modern) {
//             this.first = PerlinNoise.create(random, i, doubleList);
//             this.second = PerlinNoise.create(random, i, doubleList);
//         } else {
//             this.first = PerlinNoise.createLegacyForLegacyNetherBiome(random, i, doubleList);
//             this.second = PerlinNoise.createLegacyForLegacyNetherBiome(random, i, doubleList);
//         }
//
//         int j = Integer.MAX_VALUE;
//         int k = Integer.MIN_VALUE;
//         DoubleListIterator doubleListIterator = doubleList.iterator();
//
//         while(doubleListIterator.hasNext()) {
//             int l = doubleListIterator.nextIndex();
//             double d = doubleListIterator.nextDouble();
//             if (d != 0.0D) {
//                 j = Math.min(j, l);
//                 k = Math.max(k, l);
//             }
//         }
//
//         this.valueFactor = 0.16666666666666666D / expectedDeviation(k - j);
//         this.maxValue = (this.first.maxValue() + this.second.maxValue()) * this.valueFactor;
//     }
//
//     public double maxValue() {
//         return this.maxValue;
//     }
//
//     private static double expectedDeviation(int octaves) {
//         return 0.1D * (1.0D + 1.0D / (double)(octaves + 1));
//     }
//
//     public double getValue(double x, double y, double z) {
//         double d = x * 1.0181268882175227D;
//         double e = y * 1.0181268882175227D;
//         double f = z * 1.0181268882175227D;
//         return (this.first.getValue(x, y, z) + this.second.getValue(d, e, f)) * this.valueFactor;
//     }
//
//     public NormalNoise.NoiseParameters parameters() {
//         return this.parameters;
//     }
//
//     @VisibleForTesting
//     public void parityConfigString(StringBuilder info) {
//         info.append("NormalNoise {");
//         info.append("first: ");
//         this.first.parityConfigString(info);
//         info.append(", second: ");
//         this.second.parityConfigString(info);
//         info.append("}");
//     }
//
//     public static record NoiseParameters(int firstOctave, DoubleList amplitudes) {
//         public static final Codec<NormalNoise.NoiseParameters> DIRECT_CODEC = RecordCodecBuilder.create((instance) -> {
//             return instance.group(Codec.INT.fieldOf("firstOctave").forGetter(NormalNoise.NoiseParameters::firstOctave), Codec.DOUBLE.listOf().fieldOf("amplitudes").forGetter(NormalNoise.NoiseParameters::amplitudes)).apply(instance, NormalNoise.NoiseParameters::new);
//         });
//         public static final Codec<Holder<NormalNoise.NoiseParameters>> CODEC = RegistryFileCodec.create(Registries.NOISE, DIRECT_CODEC);
//
//         public NoiseParameters(int firstOctave, List<Double> amplitudes) {
//             this(firstOctave, new DoubleArrayList(amplitudes));
//         }
//
//         public NoiseParameters(int firstOctave, double firstAmplitude, double... amplitudes) {
//             this(firstOctave, Util.make(new DoubleArrayList(amplitudes), (doubleArrayList) -> {
//                 doubleArrayList.add(0, firstAmplitude);
//             }));
//         }
//     }
// }

use super::perlin_noise::PerlinNoise;

#[derive(Debug)]
pub struct NormalNoise {
    value_factor: f64,
    first: PerlinNoise,
    second: PerlinNoise,
    max_value: f64,
    // parameters: NoiseParameters,
}

impl NormalNoise {
    pub fn new(value_factor: f64, first: PerlinNoise, second: PerlinNoise, max_value: f64) -> Self {
        Self {
            value_factor,
            first,
            second,
            max_value,
        }
    }

    pub fn max_value(&self) -> f64 {
        self.max_value
    }

    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let d = x * INPUT_FACTOR;
        let e = y * INPUT_FACTOR;
        let f = z * INPUT_FACTOR;
        (self.first.get_value(x, y, z) + self.second.get_value(d, e, f)) * self.value_factor
    }
}

const INPUT_FACTOR: f64 = 1.0181268882175227;
