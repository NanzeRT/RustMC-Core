use jni::{JNIEnv, objects::JClass, sys::{jint, jlong}};

use super::density_function::{SinglePointContext, FunctionContext};


// /*
//  * Class:     net_minecraft_world_level_levelgen_DensityFunction_SinglePointContext
//  * Method:    nativeNew
//  * Signature: (III)J
//  */
// JNIEXPORT jlong JNICALL Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeNew
//   (JNIEnv *, jclass, jint, jint, jint);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeNew(
    _env: JNIEnv,
    _class: JClass,
    x: jint,
    y: jint,
    z: jint,
) -> jlong {
    Box::into_raw(Box::new(Box::new(SinglePointContext::new(x, y, z)) as Box<dyn FunctionContext>)) as jlong
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_DensityFunction_SinglePointContext
//  * Method:    nativeDelete
//  * Signature: (J)V
//  */
// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeDelete
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { drop(Box::from_raw(ptr as *mut Box<dyn FunctionContext>)); }
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_DensityFunction_SinglePointContext
//  * Method:    nativeBlockX
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeBlockX
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeBlockX(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let context = unsafe { &**(ptr as *const*const SinglePointContext) };
    context.block_x() as jint
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_DensityFunction_SinglePointContext
//  * Method:    nativeBlockY
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeBlockY
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeBlockY(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let context = unsafe { &**(ptr as *const*const SinglePointContext) };
    context.block_y() as jint
}
// /*
//  * Class:     net_minecraft_world_level_levelgen_DensityFunction_SinglePointContext
//  * Method:    nativeBlockZ
//  * Signature: (J)I
//  */
// JNIEXPORT jint JNICALL Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeBlockZ
//   (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_DensityFunction_00024SinglePointContext_nativeBlockZ(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    let context = unsafe { &**(ptr as *const*const SinglePointContext) };
    context.block_z() as jint
}
