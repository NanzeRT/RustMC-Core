use jni::{sys::{jint, jlong, jboolean}, objects::JObject, JNIEnv};

use crate::level::state::block_state::BlockStateId;

use super::{aquifer::{FluidStatus, FluidPicker}, noise_based_chunk_generator::FluidPickerFromNoiseChunk};


// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_FluidPickerFromNoiseChunk
//  * Method:    newNative
//  * Signature: (ILnet/minecraft/world/level/block/state/BlockState;ILnet/minecraft/world/level/block/state/BlockState;I)J
//  */
// JNIEXPORT jlong JNICALL Java_net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_00024FluidPickerFromNoiseChunk_newNative
//   (JNIEnv *, jobject, jint, jobject, jint, jobject, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_00024FluidPickerFromNoiseChunk_newNative(
    _env: JNIEnv,
    _this: JObject,
    aquifer_b_y: jint,
    // aquifer_b_blockstate: JObject,
    aquifer_b_blockstate_id: jint,
    aquifer_b_block_id: jint,
    aquifer_b_is_air: jboolean,
    aquifer_b1_y: jint,
    // aquifer_b1_blockstate: JObject,
    aquifer_b1_blockstate_id: jint,
    aquifer_b1_block_id: jint,
    aquifer_b1_is_air: jboolean,
    i: jint,
) -> jlong {
    let aquifer_b_blockstate = BlockStateId::new(aquifer_b_blockstate_id, aquifer_b_block_id);
    let aquifer_b = FluidStatus::new(aquifer_b_y, aquifer_b_blockstate, aquifer_b_is_air != 0);
    let aquifer_b1_blockstate = BlockStateId::new(aquifer_b1_blockstate_id, aquifer_b1_block_id);
    let aquifer_b1 = FluidStatus::new(aquifer_b1_y, aquifer_b1_blockstate, aquifer_b1_is_air != 0);
    let fluid_picker = super::noise_based_chunk_generator::create_fluid_picker(aquifer_b, aquifer_b1, i);
    Box::into_raw(Box::new(fluid_picker)) as jlong
}
//
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_FluidPickerFromNoiseChunk
//  * Method:    deleteNative
//  * Signature: (J)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_00024FluidPickerFromNoiseChunk_deleteNative
//   (JNIEnv *, jobject, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_00024FluidPickerFromNoiseChunk_deleteNative(
    _env: JNIEnv,
    _this: JObject,
    fluid_picker_ptr: jlong,
) {
    unsafe {
        drop(Box::from_raw(fluid_picker_ptr as *mut FluidPickerFromNoiseChunk))
    }
}


#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseBasedChunkGenerator_00024FluidPickerFromNoiseChunk_computeFluidAtNative(
    _env: JNIEnv,
    _this: JObject,
    fluid_picker_ptr: jlong,
    x: jint,
    y: jint,
    z: jint,
    y1: jint,
) -> jint {
    let fluid_picker = unsafe { &*(fluid_picker_ptr as *const FluidPickerFromNoiseChunk) };
    fluid_picker.compute_fluid(x, y, z).at(y1).id
}
