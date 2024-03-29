use std::mem::MaybeUninit;

use crate::level::state::block_state::BlockStateId;

pub trait FluidPicker {
    fn compute_fluid(&self, x: i32, y: i32, z: i32) -> FluidStatus;
}

#[derive(Debug, Clone)]
pub struct FluidStatus {
    pub(crate) fluid_level: i32,
    // fluid_type: GlobalRef,
    fluid_type: BlockStateId,
    is_type_air: bool,
}

// pub(crate) static DEFAULT_AIR_STATE: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static mut DEFAULT_AIR_STATE_ID: MaybeUninit<BlockStateId> = MaybeUninit::uninit();

impl FluidStatus {
    pub fn new(fluid_level: i32, fluid_type: BlockStateId, is_type_air: bool) -> Self {
        Self {
            fluid_level,
            fluid_type,
            is_type_air,
        }
    }

    pub fn at(&self, y: i32) -> BlockStateId {
        if y < self.fluid_level {
            self.fluid_type
        } else {
            unsafe { DEFAULT_AIR_STATE_ID.assume_init() }
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
