use std::rc::Rc;
use super::synth::normal_noise::NormalNoise;

pub trait FunctionContext {
    fn block_x(&self) -> i32;
    fn block_y(&self) -> i32;
    fn block_z(&self) -> i32;
}

pub struct SinglePointContext {
    x: i32,
    y: i32,
    z: i32,
}

impl SinglePointContext {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl FunctionContext for SinglePointContext {
    fn block_x(&self) -> i32 {
        self.x
    }

    fn block_y(&self) -> i32 {
        self.y
    }

    fn block_z(&self) -> i32 {
        self.z
    }
}

pub trait ContextProvider {
    fn for_index(&mut self, index: i32) -> Rc<dyn FunctionContext>;
    // fn fill_all_directly(&mut self, densities: &mut [f32], density_function: &dyn DensityFunction);
}

pub trait DensityFunction {
    fn compute<T>(&self, pos: &T) -> f64
        where T: FunctionContext + ?Sized;
    // fn fill_array(&self, densities: &mut [f32], applier: &mut dyn ContextProvider);
    // fn map_all(&self, visitor: &mut dyn Visitor) -> Box<dyn DensityFunction>;
    // fn min_value(&self) -> f64;
    // fn max_value(&self) -> f64;
    // fn codec(&self) -> KeyDispatchDataCodec<dyn DensityFunction>;
}

// public static record NoiseHolder(Holder<NormalNoise.NoiseParameters> noiseData, @Nullable NormalNoise noise) {
//     public static final Codec<DensityFunction.NoiseHolder> CODEC = NormalNoise.NoiseParameters.CODEC.xmap((noiseData) -> {
//         return new DensityFunction.NoiseHolder(noiseData, (NormalNoise)null);
//     }, DensityFunction.NoiseHolder::noiseData);
//
//     public NoiseHolder(Holder<NormalNoise.NoiseParameters> noiseData) {
//         this(noiseData, (NormalNoise)null);
//     }
//
//     public double getValue(double x, double y, double z) {
//         return this.noise == null ? 0.0D : this.noise.getValue(x, y, z);
//     }
//
//     public double maxValue() {
//         return this.noise == null ? 2.0D : this.noise.maxValue();
//     }
// }


#[derive(Debug)]
pub struct NoiseHolder {
    // noise_data: NoiseParameters,
    noise: Option<NormalNoise>,
}

impl NoiseHolder {
    pub fn new(noise: Option<NormalNoise>) -> Self {
        Self {
            // noise_data,
            noise,
        }
    }
    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        match &self.noise {
            Some(noise) => noise.get_value(x, y, z),
            None => 0.0,
        }
    }

    pub fn max_value(&self) -> f64 {
        match &self.noise {
            Some(noise) => noise.max_value(),
            None => 2.0,
        }
    }
}
