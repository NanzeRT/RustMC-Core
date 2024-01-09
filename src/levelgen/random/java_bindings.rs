// JNIEXPORT jlong JNICALL Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeNew
//   (JNIEnv *, jclass, jlong);

use jni::{objects::JClass, sys::jlong, JNIEnv};

use super::{
    legacy_random_source::LegacyPositionalRandomFactory,
    positional_random_factory::PositionalRandomFactoryVariants,
    xoroshiro_random_source::XoroshiroPositionalRandomFactory,
};

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeNew(
    _env: JNIEnv,
    _class: JClass,
    seed: jlong,
) -> jlong {
    Box::into_raw(Box::new(PositionalRandomFactoryVariants::Legacy(
        LegacyPositionalRandomFactory::new(seed),
    ))) as jlong
}

// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeDelete
//   (JNIEnv *, jclass, jlong);

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { drop(Box::from_raw(ptr as *mut PositionalRandomFactoryVariants)) }
}

// JNIEXPORT jlong JNICALL Java_net_minecraft_world_level_levelgen_XoroshiroRandomSource_00024XoroshiroPositionalRandomFactory_nativeNew
//   (JNIEnv *, jclass, jlong, jlong);

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_XoroshiroRandomSource_00024XoroshiroPositionalRandomFactory_nativeNew(
    _env: JNIEnv,
    _class: JClass,
    seed0: jlong,
    seed1: jlong,
) -> jlong {
    Box::into_raw(Box::new(PositionalRandomFactoryVariants::Xoroshiro(
        XoroshiroPositionalRandomFactory::new((seed0, seed1)),
    ))) as jlong
}

// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_XoroshiroRandomSource_00024XoroshiroPositionalRandomFactory_nativeDelete
//   (JNIEnv *, jclass, jlong);

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_XoroshiroRandomSource_00024XoroshiroPositionalRandomFactory_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { drop(Box::from_raw(ptr as *mut PositionalRandomFactoryVariants)) }
}
