use core::block_pos;
use std::fmt::Debug;
use std::rc::Rc;

use jni::objects::{GlobalRef, JFieldID, JMethodID, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use jni::sys::{jboolean, jdouble, jint, jlong};
use jni::JNIEnv;

use level::block::BlockId;
use level::state::block_state::BlockStateId;
use levelgen::aquifer::{FluidPicker, FluidStatus};
use levelgen::density_function::{DensityFunction, FunctionContext, FunctionContextVariants};
use levelgen::density_functions::Noise;
use levelgen::density_functions_java_bindings::create_noise_from_jobject;
use levelgen::noise_based_chunk_generator::FluidPickerFromNoiseChunk;
use levelgen::noise_chunk::NoiseChunk;
use once_cell::sync::OnceCell;

use crate::levelgen::random::positional_random_factory::PositionalRandomFactory;

pub mod levelgen {
    pub mod random {
        pub mod bits_random_source;
        pub mod java_bindings;
        pub mod legacy_random_source;
        pub mod marsaglia_polar_gaussian;
        pub mod math;
        pub mod positional_random_factory;
        pub mod random_source;
        pub mod random_support;
        pub mod xoroshiro_random_source;
    }
    pub mod synth {
        pub mod improved_noise;
        pub mod normal_noise;
        pub mod perlin_noise;
        pub mod simplex_noise;
    }
    pub mod aquifer;
    pub mod density_function;
    pub mod density_function_java_bindings;
    pub mod density_functions;
    pub mod density_functions_java_bindings;
    pub mod noise_based_chunk_generator;
    pub mod noise_based_chunk_generator_java_bindings;
    pub mod noise_chunk;
    pub mod noise_chunk_java_bindings;
    pub mod noise_settings;
}

pub mod level {
    pub mod state {
        pub mod block_state;
    }
    pub mod block;
}

pub mod core {
    pub mod block_pos;
}

pub mod util {
    pub mod mth;
}

pub struct NoiseBasedAquifer<BN, FP>
where
    BN: DensityFunction,
    FP: FluidPicker,
{
    description: AquiferDesc,
    barrier_noise: BN,
    global_fluid_picker: FP,
    // noise_chunk: &'a Cell<NoiseChunk>,
    should_schedule_fluid_update: bool,
    position_random_factory: Rc<dyn PositionalRandomFactory>,
    aquifer_location_cache: Vec<i64>,
    aquifer_cache: Box<[OnceCell<FluidStatus>]>,
}

#[derive(Debug)]
struct AquiferDesc {
    min_grid_x: i32,
    min_grid_y: i32,
    min_grid_z: i32,
    grid_size_x: i32,
    grid_size_z: i32,
}

impl AquiferDesc {
    fn get_index(&self, x: i32, y: i32, z: i32) -> i32 {
        let i: i32 = x - self.min_grid_x;
        let j: i32 = y - self.min_grid_y;
        let k: i32 = z - self.min_grid_z;
        (j * self.grid_size_z + k) * self.grid_size_x + i
    }
}

impl<BN, FP> Debug for NoiseBasedAquifer<BN, FP>
where
    BN: DensityFunction + Debug,
    FP: FluidPicker + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoiseBasedAquifer")
            .field("description", &self.description)
            .field(
                "should_schedule_fluid_update",
                &self.should_schedule_fluid_update,
            )
            .finish()
    }
}

impl<BN, FP> NoiseBasedAquifer<BN, FP>
where
    BN: DensityFunction,
    FP: FluidPicker,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        min_grid_x: i32,
        min_grid_y: i32,
        min_grid_z: i32,
        grid_size_x: i32,
        grid_size_z: i32,
        barrier_noise: BN,
        position_random_factory: &mut Rc<dyn PositionalRandomFactory>,
        fluid_level_sampler: FP,
        // noise_chunk: &'a Cell<NoiseChunk>,
        minimum_y: i32,
        height: i32,
    ) -> Self {
        let j = grid_y(minimum_y + height) + 1;
        let k = j - min_grid_y + 1;
        let aquifer_cache_size = grid_size_x * k * grid_size_z;
        Self {
            description: AquiferDesc {
                min_grid_y,
                min_grid_x,
                min_grid_z,
                grid_size_x,
                grid_size_z,
            },
            barrier_noise,
            global_fluid_picker: fluid_level_sampler,
            should_schedule_fluid_update: false,
            position_random_factory: Rc::clone(position_random_factory),
            aquifer_location_cache: vec![std::i64::MAX; aquifer_cache_size as usize],
            aquifer_cache: vec![OnceCell::new(); aquifer_cache_size as usize].into_boxed_slice(),
            // noise_chunk,
        }
    }

    fn get_index(&self, x: i32, y: i32, z: i32) -> i32 {
        self.description.get_index(x, y, z)
    }
}

fn grid_x(x: i32) -> i32 {
    x.div_euclid(16)
}

fn grid_y(y: i32) -> i32 {
    y.div_euclid(12)
}

fn grid_z(z: i32) -> i32 {
    z.div_euclid(16)
}

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_Aquifer_00024NoiseBasedAquifer_nativeNew<
    'local,
>(
    mut env: JNIEnv<'local>,
    _: JObject<'local>,
    min_grid_x: i32,
    min_grid_y: i32,
    min_grid_z: i32,
    grid_size_x: i32,
    grid_size_z: i32,
    barrier_noise: JObject<'local>,
    position_random_factory_ptr: jlong,
    fluid_level_sampler_ptr: jlong,
    // noise_chunk_ptr: jlong,
    minimum_y: i32,
    height: i32,
) -> jlong {
    let position_random_factory =
        unsafe { &mut *(position_random_factory_ptr as *mut Rc<dyn PositionalRandomFactory>) };
    let fluid_level_sampler =
        unsafe { &*(fluid_level_sampler_ptr as *mut FluidPickerFromNoiseChunk) };
    // let noise_chunk = unsafe { &*(noise_chunk_ptr as *mut Cell<NoiseChunk>) };
    let barrier_noise = match create_noise_from_jobject(&mut env, barrier_noise) {
        Ok(noise) => noise,
        Err(err) => {
            if env.exception_check().unwrap() {
                env.exception_describe().unwrap();
                env.exception_clear().unwrap();
            }
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("create_noise_from_jobject failed: {}", err).as_str(),
            )
            .unwrap();
            return 0;
        }
    };

    Box::into_raw(Box::new(NoiseBasedAquifer::new(
        min_grid_x,
        min_grid_y,
        min_grid_z,
        grid_size_x,
        grid_size_z,
        // create_noise_from_jobject(&mut env, barrier_noise).unwrap(),
        barrier_noise,
        position_random_factory,
        fluid_level_sampler.clone(),
        // noise_chunk,
        minimum_y,
        height,
    ))) as jlong
}

/// assumes that BN is a Noise
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_Aquifer_00024NoiseBasedAquifer_nativeDelete(
    _: JNIEnv,
    _: JObject,
    ptr: jlong,
) {
    unsafe {
        drop(Box::from_raw(
            ptr as *mut NoiseBasedAquifer<Noise, FluidPickerFromNoiseChunk>,
        ));
    }
}

/// assumes that BN is a Noise
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_Aquifer_00024NoiseBasedAquifer_shouldScheduleFluidUpdateNative(
    _: JNIEnv,
    ptr: jlong,
) -> jboolean {
    let this = unsafe { &mut *(ptr as *mut NoiseBasedAquifer<Noise, FluidPickerFromNoiseChunk>) };
    this.should_schedule_fluid_update.into()
}

static BLOCK_LAVA: OnceCell<GlobalRef> = OnceCell::new();
static BLOCK_WATER: OnceCell<GlobalRef> = OnceCell::new();
static BLOCK_LAVA_ID: OnceCell<BlockId> = OnceCell::new();
static BLOCK_WATER_ID: OnceCell<BlockId> = OnceCell::new();
static DEFAULT_LAVA_STATE_ID: OnceCell<BlockStateId> = OnceCell::new();
// static COMPUTE_FLUID_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();
static COMPUTE_FLUID_TO_NATIVE_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();
// static FLUID_STATUS_AT_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();
static BLOCK_STATE_IS_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();
static BLOCK_DEFAULT_BLOCK_STATE_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();
// static GET_AQUIFER_STATUS_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();
static BLOCK_STATE_FLUID_LEVEL_FIELD_ID: OnceCell<JFieldID> = OnceCell::new();

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_Aquifer_00024NoiseBasedAquifer_setup(
    mut env: JNIEnv,
    _: JObject,
) {
    let tmp1 = env
        .find_class("net/minecraft/world/level/block/Blocks")
        .unwrap();
    let tmp2 = env
        .get_static_field(&tmp1, "LAVA", "Lnet/minecraft/world/level/block/Block;")
        .unwrap()
        .l()
        .unwrap();
    let tmp3 = env.new_global_ref(tmp2).unwrap();
    BLOCK_LAVA.set(tmp3).unwrap();
    let tmp4 = env
        .get_static_field(&tmp1, "WATER", "Lnet/minecraft/world/level/block/Block;")
        .unwrap()
        .l()
        .unwrap();
    let tmp5 = env.new_global_ref(tmp4).unwrap();
    BLOCK_WATER.set(tmp5).unwrap();
    let lava_id = env
        .call_static_method(
            "io/rustmc/Glue",
            "blockToId",
            "(Lnet/minecraft/world/level/block/Block;)I",
            &[JValue::from(BLOCK_LAVA.get().unwrap())],
        )
        .unwrap()
        .i()
        .unwrap();
    BLOCK_LAVA_ID.set(BlockId::new(lava_id)).unwrap();
    let water_id = env
        .call_static_method(
            "io/rustmc/Glue",
            "blockToId",
            "(Lnet/minecraft/world/level/block/Block;)I",
            &[JValue::from(BLOCK_WATER.get().unwrap())],
        )
        .unwrap()
        .i()
        .unwrap();
    BLOCK_WATER_ID.set(BlockId::new(water_id)).unwrap();
    let default_lava_state = env
        .call_method(
            BLOCK_LAVA.get().unwrap().as_obj(),
            "defaultBlockState",
            "()Lnet/minecraft/world/level/block/state/BlockState;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();
    let default_lava_state_id = BlockStateId::new(
        env.call_static_method(
            "io/rustmc/Glue",
            "blockStateToId",
            "(Lnet/minecraft/world/level/block/state/BlockState;)I",
            &[JValue::from(&default_lava_state)],
        )
        .unwrap()
        .i()
        .unwrap(),
        lava_id,
    );
    DEFAULT_LAVA_STATE_ID.set(default_lava_state_id).unwrap();
    // default air state
    let air = env
        .get_static_field(&tmp1, "AIR", "Lnet/minecraft/world/level/block/Block;")
        .unwrap()
        .l()
        .unwrap();
    let air_state = env
        .call_method(
            &air,
            "defaultBlockState",
            "()Lnet/minecraft/world/level/block/state/BlockState;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();
    let air_state = env.new_global_ref(air_state).unwrap();
    let air_id = env
        .call_static_method(
            "io/rustmc/Glue",
            "blockToId",
            "(Lnet/minecraft/world/level/block/Block;)I",
            &[JValue::from(&air)],
        )
        .unwrap()
        .i()
        .unwrap();
    let air_state_id = BlockStateId::new(
        env.call_static_method(
            "io/rustmc/Glue",
            "blockStateToId",
            "(Lnet/minecraft/world/level/block/state/BlockState;)I",
            &[JValue::from(&air_state)],
        )
        .unwrap()
        .i()
        .unwrap(),
        air_id,
    );
    levelgen::aquifer::DEFAULT_AIR_STATE.set(air_state).unwrap();
    levelgen::aquifer::DEFAULT_AIR_STATE_ID
        .set(air_state_id)
        .unwrap();
    // let method_id = env
    //     .get_method_id(
    //         "net/minecraft/world/level/levelgen/Aquifer$NoiseBasedAquifer",
    //         "computeFluid",
    //         "(III)Lnet/minecraft/world/level/levelgen/Aquifer$FluidStatus;",
    //     )
    //     .unwrap();
    // COMPUTE_FLUID_METHOD_ID.set(method_id).unwrap();
    let method_id = env
        .get_method_id(
            "net/minecraft/world/level/levelgen/Aquifer$NoiseBasedAquifer",
            "computeFluidToNative",
            "(III)J",
        )
        .unwrap();
    COMPUTE_FLUID_TO_NATIVE_METHOD_ID.set(method_id).unwrap();
    // let method_id = env
    //     .get_method_id(
    //         "net/minecraft/world/level/levelgen/Aquifer$FluidStatus",
    //         "at",
    //         "(I)Lnet/minecraft/world/level/block/state/BlockState;",
    //     )
    //     .unwrap();
    // FLUID_STATUS_AT_METHOD_ID.set(method_id).unwrap();
    let method_id = env
        .get_method_id(
            "net/minecraft/world/level/block/state/BlockState",
            "is",
            "(Lnet/minecraft/world/level/block/Block;)Z",
        )
        .unwrap();
    BLOCK_STATE_IS_METHOD_ID.set(method_id).unwrap();
    let method_id = env
        .get_method_id(
            "net/minecraft/world/level/block/Block",
            "defaultBlockState",
            "()Lnet/minecraft/world/level/block/state/BlockState;",
        )
        .unwrap();
    BLOCK_DEFAULT_BLOCK_STATE_METHOD_ID.set(method_id).unwrap();
    // let method_id = env
    //     .get_method_id(
    //         "net/minecraft/world/level/levelgen/Aquifer$NoiseBasedAquifer",
    //         "getAquiferStatus",
    //         "(J)Lnet/minecraft/world/level/levelgen/Aquifer$FluidStatus;",
    //     )
    //     .unwrap();
    // GET_AQUIFER_STATUS_METHOD_ID.set(method_id).unwrap();
    let field_id = env
        .get_field_id(
            "net/minecraft/world/level/levelgen/Aquifer$FluidStatus",
            "fluidLevel",
            "I",
        )
        .unwrap();
    BLOCK_STATE_FLUID_LEVEL_FIELD_ID
        .set(field_id)
        .unwrap_or_else(|_| panic!("BLOCK_STATE_FLUID_LEVEL_FIELD_ID.set(field_id) failed"));
}

/// assumes that BN is a Noise
/// assumes that FP is a FluidPickerFromNoiseChunk
#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_Aquifer_00024NoiseBasedAquifer_computeSubstanceNative<
    'local,
>(
    env: JNIEnv<'local>,
    jthis: JObject<'local>,
    pos_ptr: jlong,
    density: jdouble,
    this_ptr: jlong,
    this_noise_chunk_ptr: jlong,
) -> jint {
    // println!("NoiseBasedAquifer::computeSubstanceNative");
    let this =
        unsafe { &mut *(this_ptr as *mut NoiseBasedAquifer<Noise, FluidPickerFromNoiseChunk>) };
    let pos = unsafe { &*(pos_ptr as *mut FunctionContextVariants) };
    let this_noise_chunk = unsafe { &*(this_noise_chunk_ptr as *mut NoiseChunk) };

    let res = compute_substance(this, jthis, this_noise_chunk, env, pos, density).unwrap_or(None);
    // let res = match pos {
    //     FunctionContextVariants::SinglePointContext(pos) => {
    //         compute_substance(this, jthis, this_noise_chunk, env, pos, density).unwrap_or(None)
    //     }
    //     FunctionContextVariants::NoiseChunk(pos) => {
    //         compute_substance(this, jthis, this_noise_chunk, env, pos, density).unwrap_or(None)
    //     }
    // };
    match res {
        Some(block_state) => block_state.id,
        None => -1,
    }
}

pub fn compute_substance<BN, FP>(
    this: &mut NoiseBasedAquifer<BN, FP>,
    jthis: JObject,
    _this_noise_chunk: &NoiseChunk,
    mut env: JNIEnv,
    pos: &(impl FunctionContext + ?Sized),
    density: f64,
) -> Result<Option<BlockStateId>, jni::errors::Error>
where
    BN: DensityFunction,
    FP: FluidPicker,
{
    let i: i32 = pos.block_x();
    let j: i32 = pos.block_y();
    let k: i32 = pos.block_z();

    if density > 0.0 {
        this.should_schedule_fluid_update = false;
        Ok(None)
    } else {
        let fluid_status = compute_fluid_placeholder(&mut env, &jthis, i, j, k);
        if fluid_status.at(j).is_block(*BLOCK_LAVA_ID.get().unwrap()) {
            this.should_schedule_fluid_update = false;
            return Ok(Some(*DEFAULT_LAVA_STATE_ID.get().unwrap()));
        } else {
            let l: i32 = grid_x(i - 5);
            let m: i32 = grid_y(j + 1);
            let n: i32 = grid_z(k - 5);
            let mut o: i32 = std::i32::MAX;
            let mut p: i32 = std::i32::MAX;
            let mut q: i32 = std::i32::MAX;
            let mut r: i64 = 0;
            let mut s: i64 = 0;
            let mut t: i64 = 0;

            for u in 0..=1 {
                for v in -1..=1 {
                    for w in 0..=1 {
                        let x: i32 = l + u;
                        let y: i32 = m + v;
                        let z: i32 = n + w;
                        let aa: i32 = this.get_index(x, y, z);
                        let ab: i64 = this.aquifer_location_cache[aa as usize];
                        let ac: i64;
                        if ab != std::i64::MAX {
                            ac = ab;
                        } else {
                            let mut random_source = this.position_random_factory.at(x, y, z);
                            ac = core::block_pos::as_long(
                                x * 16 + random_source.next_int_bound(10),
                                y * 12 + random_source.next_int_bound(9),
                                z * 16 + random_source.next_int_bound(10),
                            );
                            this.aquifer_location_cache[aa as usize] = ac;
                        }

                        let ae: i32 = core::block_pos::get_x_long(ac) - i;
                        let af: i32 = core::block_pos::get_y_long(ac) - j;
                        let ag: i32 = core::block_pos::get_z_long(ac) - k;
                        let ah: i32 = ae * ae + af * af + ag * ag;
                        if o >= ah {
                            t = s;
                            s = r;
                            r = ac;
                            q = p;
                            p = o;
                            o = ah;
                        } else if p >= ah {
                            t = s;
                            s = ac;
                            q = p;
                            p = ah;
                        } else if q >= ah {
                            t = ac;
                            q = ah;
                        }
                    }
                }
            }

            let fluid_status2 = get_aquifer_status(
                &mut env,
                &jthis,
                &this.description,
                this.aquifer_cache.as_ref(),
                r,
            );
            let d: f64 = similarity(o, p);
            let block_state = fluid_status2.at(j);
            if d <= 0.0 {
                this.should_schedule_fluid_update = d >= similarity(100, 144);
                Ok(Some(block_state))
            } else if block_state.is_block(*BLOCK_WATER_ID.get().unwrap())
                && this
                    .global_fluid_picker
                    .compute_fluid(i, j - 1, k)
                    .at(j - 1)
                    .is_block(*BLOCK_LAVA_ID.get().unwrap())
            {
                this.should_schedule_fluid_update = true;
                Ok(Some(block_state))
            } else {
                let mut mutable_double: f64 = std::f64::NAN;
                let fluid_status3 = get_aquifer_status(
                    &mut env,
                    &jthis,
                    &this.description,
                    this.aquifer_cache.as_ref(),
                    s,
                );
                let e: f64 = d * calculate_pressure(
                    &mut env,
                    this,
                    pos,
                    &mut mutable_double,
                    fluid_status2,
                    fluid_status3,
                )?;
                if density + e > 0.0 {
                    this.should_schedule_fluid_update = false;
                    Ok(None)
                } else {
                    let fluid_status4 = get_aquifer_status(
                        &mut env,
                        &jthis,
                        &this.description,
                        this.aquifer_cache.as_ref(),
                        t,
                    );
                    let f: f64 = similarity(o, q);
                    if f > 0.0 {
                        let g: f64 = d
                            * f
                            * calculate_pressure(
                                &mut env,
                                this,
                                pos,
                                &mut mutable_double,
                                fluid_status2,
                                fluid_status4,
                            )?;
                        if density + g > 0.0 {
                            this.should_schedule_fluid_update = false;
                            return Ok(None);
                        }
                    }

                    let h: f64 = similarity(p, q);
                    if h > 0.0 {
                        let ai: f64 = d
                            * h
                            * calculate_pressure(
                                &mut env,
                                this,
                                pos,
                                &mut mutable_double,
                                fluid_status3,
                                fluid_status4,
                            )?;
                        if density + ai > 0.0 {
                            this.should_schedule_fluid_update = false;
                            return Ok(None);
                        }
                    }

                    this.should_schedule_fluid_update = false;
                    Ok(Some(block_state))
                }
            }
        }
    }
}

fn similarity(i: i32, a: i32) -> f64 {
    1.0 - (a - i).abs() as f64 / 25.0
}

fn calculate_pressure<BN, FP>(
    _env: &mut JNIEnv,
    this: &NoiseBasedAquifer<BN, FP>,
    pos: &(impl FunctionContext + ?Sized),
    mutable_double: &mut f64,
    fluid_status: &FluidStatus,
    fluid_status2: &FluidStatus,
) -> Result<f64, jni::errors::Error>
where
    BN: DensityFunction,
    FP: FluidPicker,
{
    let i: i32 = pos.block_y();
    // let block_state: JObject = unsafe {
    //     env.call_method_unchecked(
    //         fluid_status,
    //         FLUID_STATUS_AT_METHOD_ID.get().unwrap(),
    //         ReturnType::Object,
    //         &[JValue::from(i).as_jni()],
    //     )?
    //     .l()?
    // };
    let block_state = fluid_status.at(i);
    // let block_state2: JObject = unsafe {
    //     env.call_method_unchecked(
    //         fluid_status2,
    //         FLUID_STATUS_AT_METHOD_ID.get().unwrap(),
    //         ReturnType::Object,
    //         &[JValue::from(i).as_jni()],
    //     )?
    //     .l()?
    // };
    let block_state2 = fluid_status2.at(i);
    if !block_state.is_block(*BLOCK_LAVA_ID.get().unwrap())
        || !block_state2.is_block(*BLOCK_WATER_ID.get().unwrap())
            && !block_state.is_block(*BLOCK_WATER_ID.get().unwrap())
        || !block_state2.is_block(*BLOCK_LAVA_ID.get().unwrap())
    {
        let fluid_level: i32 = fluid_status.fluid_level;
        let fluid_level2: i32 = fluid_status2.fluid_level;
        let j: i32 = (fluid_level - fluid_level2).abs();
        if j == 0 {
            Ok(0.0)
        } else {
            let d: f64 = 0.5 * (fluid_level + fluid_level2) as f64;
            let e: f64 = i as f64 + 0.5 - d;
            let f: f64 = j as f64 / 2.0;
            let _g: f64 = 0.0;
            let _h: f64 = 2.5;
            let _k: f64 = 1.5;
            let _l: f64 = 3.0;
            let _m: f64 = 10.0;
            let _n: f64 = 3.0;
            let o: f64 = f - e.abs();
            let q: f64;
            if e > 0.0 {
                let p: f64 = 0.0 + o;
                if p > 0.0 {
                    q = p / 1.5;
                } else {
                    q = p / 2.5;
                }
            } else {
                let s: f64 = 3.0 + o;
                if s > 0.0 {
                    q = s / 3.0;
                } else {
                    q = s / 10.0;
                }
            }

            let _v: f64 = 2.0;
            let z: f64;
            if (-2.0..=2.0).contains(&q) {
                let x: f64 = *mutable_double;
                if mutable_double.is_nan() {
                    *mutable_double = this.barrier_noise.compute(pos);
                    z = *mutable_double;
                } else {
                    z = x;
                }
            } else {
                z = 0.0;
            }

            Ok(2.0 * (z + q))
        }
    } else {
        Ok(2.0)
    }
}

fn get_aquifer_status<'a>(
    env: &mut JNIEnv,
    jthis: &JObject,
    description: &AquiferDesc,
    aquifer_cache: &'a [OnceCell<FluidStatus>],
    pos: i64,
) -> &'a FluidStatus {
    let i: i32 = block_pos::get_x_long(pos);
    let j: i32 = block_pos::get_y_long(pos);
    let k: i32 = block_pos::get_z_long(pos);
    let l: i32 = grid_x(i);
    let m: i32 = grid_y(j);
    let n: i32 = grid_z(k);
    let o: i32 = description.get_index(l, m, n);
    aquifer_cache[o as usize].get_or_init(|| compute_fluid_placeholder(env, jthis, i, j, k))
}

#[no_mangle]
pub extern "system" fn Java_net_minecraft_world_level_levelgen_Aquifer_00024NoiseBasedAquifer_makeFluidStatusNative(
    _: JNIEnv,
    _: JObject,
    fluid_level: i32,
    fluid_type_state_id: jint,
    fluid_type_block_id: jint,
    is_air: jboolean,
) -> jlong {
    // let fluid_type = env.new_global_ref(fluid_type).unwrap();
    Box::into_raw(Box::new(FluidStatus::new(
        fluid_level,
        BlockStateId::new(fluid_type_state_id, fluid_type_block_id),
        is_air == 1,
    ))) as jlong
}

fn compute_fluid_placeholder(
    env: &mut JNIEnv,
    jthis: &JObject,
    i: i32,
    j: i32,
    k: i32,
) -> FluidStatus {
    let ptr = unsafe {
        env.call_method_unchecked(
            jthis,
            COMPUTE_FLUID_TO_NATIVE_METHOD_ID.get().unwrap(),
            ReturnType::Primitive(Primitive::Long),
            &[
                JValue::from(i).as_jni(),
                JValue::from(j).as_jni(),
                JValue::from(k).as_jni(),
            ],
        )
        .unwrap()
        .j()
        .unwrap()
    };
    *unsafe { Box::from_raw(ptr as *mut FluidStatus) }
}

// private Aquifer.FluidStatus computeFluid(int blockX, int blockY, int blockZ) {
//     Aquifer.FluidStatus fluidStatus =
//             this.globalFluidPicker.computeFluid(blockX, blockY, blockZ);
//     int i = Integer.MAX_VALUE;
//     int j = blockY + 12;
//     int k = blockY - 12;
//     boolean bl = false;
//
//     for (int[] is : SURFACE_SAMPLING_OFFSETS_IN_CHUNKS) {
//         int l = blockX + SectionPos.sectionToBlockCoord(is[0]);
//         int m = blockZ + SectionPos.sectionToBlockCoord(is[1]);
//         int n = this.noiseChunk.preliminarySurfaceLevel(l, m);
//         int o = n + 8;
//         boolean bl2 = is[0] == 0 && is[1] == 0;
//         if (bl2 && k > o) {
//             return fluidStatus;
//         }
//
//         boolean bl3 = j > o;
//         if (bl3 || bl2) {
//             Aquifer.FluidStatus fluidStatus2 = this.globalFluidPicker.computeFluid(l, o, m);
//             if (!fluidStatus2.at(o).isAir()) {
//                 if (bl2) {
//                     bl = true;
//                 }
//
//                 if (bl3) {
//                     return fluidStatus2;
//                 }
//             }
//         }
//
//         i = Math.min(i, n);
//     }
//
//     int p = this.computeSurfaceLevel(blockX, blockY, blockZ, fluidStatus, i, bl);
//     return new Aquifer.FluidStatus(
//             p, this.computeFluidType(blockX, blockY, blockZ, fluidStatus, p));
// }

// fn compute_fluid<BN, FP>(env: &mut JNIEnv, jthis: JObject, this: &NoiseBasedAquifer<BN, FP>, this_noise_chunk: &mut NoiseChunk, block_x: i32, block_y: i32, block_z: i32) -> FluidStatus
// where
//     BN: DensityFunction,
//     FP: FluidPicker,
// {
//     let fluid_status = this.global_fluid_picker.compute_fluid(block_x, block_y, block_z);
//     let mut i: i32 = std::i32::MAX;
//     let j: i32 = block_y + 12;
//     let k: i32 = block_y - 12;
//     let mut bl = false;
//
//     for is in SURFACE_SAMPLING_OFFSETS_IN_CHUNKS {
//         let l: i32 = block_x + section_to_block_coord(is[0]);
//         let m: i32 = block_z + section_to_block_coord(is[1]);
//         let n: i32 = this_noise_chunk.preliminary_surface_level(env, l, m);
//         let o: i32 = n + 8;
//         let bl2: bool = is[0] == 0 && is[1] == 0;
//         if bl2 && k > o {
//             return fluid_status;
//         }
//
//         let bl3: bool = j > o;
//         if bl3 || bl2 {
//             let fluid_status2 = this.global_fluid_picker.compute_fluid(l, o, m);
//             if !fluid_status2.is_air_at(o) {
//                 if bl2 {
//                     bl = true;
//                 }
//
//                 if bl3 {
//                     return fluid_status2;
//                 }
//             }
//         }
//
//         i = i.min(n);
//     }
//
//     let p: i32 = compute_surface_level(&env, &jthis, this, block_x, block_y, block_z, &fluid_status, i, bl);
//     let fluid_type = compute_fluid_type(&env, &jthis, this, block_x, block_y, block_z, &fluid_status, p);
//     let fluid_status = env.new_object(
//         "net/minecraft/world/level/levelgen/Aquifer$FluidStatus",
//         "(ILnet/minecraft/world/level/block/state/BlockState;)V",
//         &[JValue::from(p).as_jni(), JValue::from(fluid_type).as_jni()],
//     ).unwrap();
//     fluid_status
// }
//
//         // private static final int[][] SURFACE_SAMPLING_OFFSETS_IN_CHUNKS =
//         //         new int[][] {
//         //             {0, 0}, {-2, -1}, {-1, -1}, {0, -1}, {1, -1}, {-3, 0}, {-2, 0}, {-1, 0}, {1, 0},
//         //             {-2, 1}, {-1, 1}, {0, 1}, {1, 1}
//         //         };
//
// const SURFACE_SAMPLING_OFFSETS_IN_CHUNKS: [[i32; 2]; 13] = [
//     [0, 0],
//     [-2, -1],
//     [-1, -1],
//     [0, -1],
//     [1, -1],
//     [-3, 0],
//     [-2, 0],
//     [-1, 0],
//     [1, 0],
//     [-2, 1],
//     [-1, 1],
//     [0, 1],
//     [1, 1],
// ];

// public static int sectionToBlockCoord(int sectionCoord) {
//     return sectionCoord << 4;
// }
