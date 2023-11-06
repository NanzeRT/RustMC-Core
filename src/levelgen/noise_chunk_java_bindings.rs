use jni::{sys::{jlong, jint}, JNIEnv, objects::{JClass, JObject}};

use super::{noise_chunk::NoiseChunk, density_function::FunctionContext};


// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeNew
//  * Signature: (IIIIII)J
//  */
// JNIEXPORT jlong JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeNew
//   (JNIEnv *, jclass, jint, jint, jint, jint, jint, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeNew(
    env: JNIEnv,
    this: JObject,
    min_y: jint,
    height: jint,
    noise_size_horizontal: jint,
    noise_size_vertical: jint,
    cell_start_block_x: jint,
    cell_start_block_z: jint,
) -> jlong {
    let noise_chunk = NoiseChunk::new(
        env.new_weak_ref(this).unwrap().unwrap(),
        min_y,
        height,
        noise_size_horizontal,
        noise_size_vertical,
        cell_start_block_x,
        cell_start_block_z,
    );
    Box::into_raw(Box::new(Box::new(noise_chunk) as Box<dyn FunctionContext>)) as jlong
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeDelete
//  * Signature: (J)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeDelete
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) {
    unsafe { drop(Box::from_raw(noise_chunk_ptr as *mut Box<dyn FunctionContext>)); }
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeSetCellStartBlockY
//  * Signature: (JI)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetCellStartBlockY
//   (JNIEnv *, jclass, jlong, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetCellStartBlockY(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    cell_start_block_y: jint,
) {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.cell_start_block_y = cell_start_block_y;
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeSetCellStartBlockX
//  * Signature: (JI)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetCellStartBlockX
//   (JNIEnv *, jclass, jlong, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetCellStartBlockX(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    cell_start_block_x: jint,
) {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.cell_start_block_x = cell_start_block_x;
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeSetCellStartBlockZ
//  * Signature: (JI)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetCellStartBlockZ
//   (JNIEnv *, jclass, jlong, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetCellStartBlockZ(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    cell_start_block_z: jint,
) {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.cell_start_block_z = cell_start_block_z;
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeSetInCellX
//  * Signature: (JI)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetInCellX
//   (JNIEnv *, jclass, jlong, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetInCellX(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    in_cell_x: jint,
) {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.in_cell_x = in_cell_x;
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeSetInCellY
//  * Signature: (JI)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetInCellY
//   (JNIEnv *, jclass, jlong, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetInCellY(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    in_cell_y: jint,
) {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.in_cell_y = in_cell_y;
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeSetInCellZ
//  * Signature: (JI)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetInCellZ
//   (JNIEnv *, jclass, jlong, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeSetInCellZ(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    in_cell_z: jint,
) {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.in_cell_z = in_cell_z;
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeGetCellStartBlockY
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetCellStartBlockY
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetCellStartBlockY(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.cell_start_block_y
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeGetCellStartBlockX
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetCellStartBlockX
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetCellStartBlockX(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.cell_start_block_x
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeGetCellStartBlockZ
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetCellStartBlockZ
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetCellStartBlockZ(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.cell_start_block_z
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeGetInCellX
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetInCellX
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetInCellX(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.in_cell_x
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeGetInCellY
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetInCellY
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetInCellY(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.in_cell_y
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeGetInCellZ
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetInCellZ
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeGetInCellZ(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.in_cell_z
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeBlockX
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeBlockX
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeBlockX(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.block_x()
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeBlockY
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeBlockY
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeBlockY(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.block_y()
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_NoiseChunk
//  * Method:    nativeBlockZ
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeBlockZ
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativeBlockZ(
    _env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.block_z()
}

fn get_noise_chunk<'a>(noise_chunk_ptr: jlong) -> &'a mut NoiseChunk {
    let noise_chunk_dyn = unsafe { *(noise_chunk_ptr as *mut*mut dyn FunctionContext) };
    // Note: Not sure if this is safe, but it seems to work.
    unsafe { &mut *(noise_chunk_dyn as *mut NoiseChunk) }
}

    // private static native int nativePrecomputePreliminarySurfaceLevel(long nativePtr, int blockX, int blockZ);

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_NoiseChunk_nativePrecomputePreliminarySurfaceLevel(
    mut env: JNIEnv,
    _class: JClass,
    noise_chunk_ptr: jlong,
    block_x: jint,
    block_z: jint,
) -> jint {
    let noise_chunk = get_noise_chunk(noise_chunk_ptr);
    noise_chunk.preliminary_surface_level(&mut env, block_x, block_z)
}
