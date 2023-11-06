use once_cell::sync::OnceCell;

use jni::objects::GlobalRef;


pub trait FluidPicker {
    fn compute_fluid(&self, x: i32, y: i32, z: i32) -> FluidStatus;
}

#[derive(Debug, Clone)]
pub struct FluidStatus {
    fluid_level: i32,
    fluid_type: GlobalRef,
    is_type_air: bool,
}

pub(crate) static DEFAULT_AIR_STATE: OnceCell<GlobalRef> = OnceCell::new();

impl FluidStatus {
    pub fn new(fluid_level: i32, fluid_type: GlobalRef, is_type_air: bool) -> Self {
        Self {
            fluid_level,
            fluid_type,
            is_type_air,
        }
    }

    pub fn at(&self, y: i32) -> &GlobalRef {
        if y < self.fluid_level {
            &self.fluid_type
        } else {
            DEFAULT_AIR_STATE.get().unwrap()
        }
    }

    pub fn is_air_at(&self, y: i32) -> bool {
        if y < self.fluid_level {
            self.is_type_air
        } else {
            true
        }
    }
}
