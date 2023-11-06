use super::aquifer::{FluidPicker, FluidStatus};

#[derive(Debug, Clone)]
pub struct FluidPickerFromNoiseChunk {
    aquifer_b: FluidStatus,
    aquifer_b1: FluidStatus,
    i: i32,
}

pub fn create_fluid_picker(
    aquifer_b: FluidStatus,
    aquifer_b1: FluidStatus,
    i: i32,
) -> FluidPickerFromNoiseChunk {
    FluidPickerFromNoiseChunk {
        aquifer_b,
        aquifer_b1,
        i,
    }
}
// return (j, k, l) -> {
//     return k < Math.min(-54, i) ? aquifer_b : aquifer_b1;
// };

impl FluidPicker for FluidPickerFromNoiseChunk {
    fn compute_fluid(&self, _x: i32, y: i32, _z: i32) -> FluidStatus {
        if y < std::cmp::min(-54, self.i) {
            self.aquifer_b.clone()
        } else {
            self.aquifer_b1.clone()
        }
    }
}
