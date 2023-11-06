
// JNIEXPORT jlong JNICALL Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeNew
//   (JNIEnv *, jclass, jlong);

use std::rc::Rc;

use jni::{JNIEnv, objects::JClass, sys::jlong};

use super::{legacy_random_source::LegacyPositionalRandomFactory, xoroshiro_random_source::XoroshiroPositionalRandomFactory, positional_random_factory::PositionalRandomFactory};

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeNew(
    _env: JNIEnv,
    _class: JClass,
    seed: jlong,
) -> jlong {
    Box::into_raw(Box::new(Rc::new(LegacyPositionalRandomFactory::new(seed)) as Rc<dyn PositionalRandomFactory>)) as jlong
}

// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeDelete
//   (JNIEnv *, jclass, jlong);

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_LegacyRandomSource_00024LegacyPositionalRandomFactory_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { drop(Box::from_raw(ptr as *mut Rc<dyn PositionalRandomFactory>)) }
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
    Box::into_raw(Box::new(Rc::new(XoroshiroPositionalRandomFactory::new((seed0, seed1))) as Rc<dyn PositionalRandomFactory>)) as jlong
}

// JNIEXPORT void JNICALL Java_net_minecraft_world_level_levelgen_XoroshiroRandomSource_00024XoroshiroPositionalRandomFactory_nativeDelete
//   (JNIEnv *, jclass, jlong);

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_XoroshiroRandomSource_00024XoroshiroPositionalRandomFactory_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe { drop(Box::from_raw(ptr as *mut Rc<dyn PositionalRandomFactory>)) }
}
