// protected static record Noise(DensityFunction.NoiseHolder noise, double xzScale, double yScale) implements DensityFunction {
//     public static final MapCodec<DensityFunctions.Noise> DATA_CODEC = RecordCodecBuilder.mapCodec((instance) -> {
//         return instance.group(DensityFunction.NoiseHolder.CODEC.fieldOf("noise").forGetter(DensityFunctions.Noise::noise), Codec.DOUBLE.fieldOf("xz_scale").forGetter(DensityFunctions.Noise::xzScale), Codec.DOUBLE.fieldOf("y_scale").forGetter(DensityFunctions.Noise::yScale)).apply(instance, DensityFunctions.Noise::new);
//     });
//     public static final KeyDispatchDataCodec<DensityFunctions.Noise> CODEC = DensityFunctions.makeCodec(DATA_CODEC);
//
//     @Override
//     public double compute(DensityFunction.FunctionContext pos) {
//         return this.noise.getValue((double)pos.blockX() * this.xzScale, (double)pos.blockY() * this.yScale, (double)pos.blockZ() * this.xzScale);
//     }
//
//     @Override
//     public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//         applier.fillAllDirectly(densities, this);
//     }
//
//     @Override
//     public DensityFunction mapAll(DensityFunction.Visitor visitor) {
//         return visitor.apply(new DensityFunctions.Noise(visitor.visitNoise(this.noise), this.xzScale, this.yScale));
//     }
//
//     @Override
//     public double minValue() {
//         return -this.maxValue();
//     }
//
//     @Override
//     public double maxValue() {
//         return this.noise.maxValue();
//     }
//
//     @Override
//     public KeyDispatchDataCodec<? extends DensityFunction> codec() {
//         return CODEC;
//     }
// }

use crate::levelgen::density_function::DensityFunction;
use crate::levelgen::density_function::FunctionContext;
use crate::levelgen::density_function::NoiseHolder;

#[derive(Debug)]
pub struct Noise {
    noise: NoiseHolder,
    xz_scale: f64,
    y_scale: f64,
}

impl Noise {
    pub fn new(noise: NoiseHolder, xz_scale: f64, y_scale: f64) -> Self {
        Self {
            noise,
            xz_scale,
            y_scale,
        }
    }
}

impl DensityFunction for Noise {
    fn compute<T>(&self, pos: &T) -> f64
    where
        T: FunctionContext + ?Sized,
    {
        self.noise.get_value(
            (pos.block_x() as f64) * self.xz_scale,
            (pos.block_y() as f64) * self.y_scale,
            (pos.block_z() as f64) * self.xz_scale,
        )
    }
}
