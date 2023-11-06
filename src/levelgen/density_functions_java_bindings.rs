use jni::{objects::{JObject, JByteArray, JObjectArray}, JNIEnv};

use super::{density_functions::Noise, density_function::NoiseHolder, synth::{normal_noise::NormalNoise, perlin_noise::PerlinNoise, improved_noise::ImprovedNoise}};

    // protected static record Noise(DensityFunction.NoiseHolder noise, double xzScale, double yScale) implements DensityFunction {

pub fn create_noise_from_jobject(env: &mut JNIEnv, obj: JObject) -> Result<Noise, Box<dyn std::error::Error>> {
    let xz_scale = env.get_field(&obj, "xzScale", "D")?.d()?;
    let y_scale = env.get_field(&obj, "yScale", "D")?.d()?;
    let noise_holder = env.get_field(&obj, "noise", "Lnet/minecraft/world/level/levelgen/DensityFunction$NoiseHolder;")?.l()?;
    let noise_holder = create_noise_holder_from_jobject(env, noise_holder)?;
    Ok(Noise::new(noise_holder, xz_scale, y_scale))
}

    // public static record NoiseHolder(Holder<NormalNoise.NoiseParameters> noiseData, @Nullable NormalNoise noise) {

pub fn create_noise_holder_from_jobject(env: &mut JNIEnv, obj: JObject) -> Result<NoiseHolder, Box<dyn std::error::Error>> {
    // let noise_data = env.get_field(obj, "noiseData", "Lnet/minecraft/world/level/levelgen/synth/Holder;")?.l()?;
    // let noise_data = create_noise_parameters_from_jobject(env, noise_data)?;
    let noise = env.get_field(obj, "noise", "Lnet/minecraft/world/level/levelgen/synth/NormalNoise;")?.l()?;
    if noise.is_null() {
        return Ok(NoiseHolder::new(None));
    }
    let noise = create_normal_noise_from_jobject(env, noise)?;
    Ok(NoiseHolder::new(Some(noise)))
}
// public class NormalNoise {
//     private static final double INPUT_FACTOR = 1.0181268882175227D;
//     private static final double TARGET_DEVIATION = 0.3333333333333333D;
//     private final double valueFactor;
//     private final PerlinNoise first;
//     private final PerlinNoise second;
//     private final double maxValue;
//     private final NormalNoise.NoiseParameters parameters;

fn create_normal_noise_from_jobject(env: &mut JNIEnv, noise: JObject) -> Result<NormalNoise, Box<dyn std::error::Error>> {
    let value_factor = env.get_field(&noise, "valueFactor", "D")?.d()?;
    let first = env.get_field(&noise, "first", "Lnet/minecraft/world/level/levelgen/synth/PerlinNoise;")?.l()?;
    let first = create_perlin_noise_from_jobject(env, first)?;
    let second = env.get_field(&noise, "second", "Lnet/minecraft/world/level/levelgen/synth/PerlinNoise;")?.l()?;
    let second = create_perlin_noise_from_jobject(env, second)?;
    let max_value = env.get_field(&noise, "maxValue", "D")?.d()?;
    // let parameters = env.get_field(&noise, "parameters", "Lnet/minecraft/world/level/levelgen/synth/NormalNoise$NoiseParameters;")?.l()?;
    // let parameters = create_noise_parameters_from_jobject(env, parameters)?;
    Ok(NormalNoise::new(value_factor, first, second, max_value))
}

fn create_perlin_noise_from_jobject(env: &mut JNIEnv, first: JObject) -> Result<PerlinNoise, Box<dyn std::error::Error>> {
    let noise_levels = env.get_field(&first, "noiseLevels", "[Lnet/minecraft/world/level/levelgen/synth/ImprovedNoise;")?.l()?;
    let noise_levels = create_option_improved_noise_array_from_jobject(env, noise_levels.into())?;
    let first_octave = env.get_field(&first, "firstOctave", "I")?.i()?;
    // let amplitudes = env.get_field(&first, "amplitudes", "Lnet/minecraft/world/level/levelgen/synth/DoubleList;")?.l()?;
// package it.unimi.dsi.fastutil.doubles;
    let amplitudes = env.get_field(&first, "amplitudes", "Lit/unimi/dsi/fastutil/doubles/DoubleList;")?.l()?;
    let amplitudes = create_double_list_from_jobject(env, amplitudes)?;
    let lowest_freq_input_factor = env.get_field(&first, "lowestFreqInputFactor", "D")?.d()?;
    let lowest_freq_value_factor = env.get_field(&first, "lowestFreqValueFactor", "D")?.d()?;
    let max_value = env.get_field(&first, "maxValue", "D")?.d()?;
    Ok(PerlinNoise::new(noise_levels, first_octave, amplitudes, lowest_freq_input_factor, lowest_freq_value_factor, max_value))
}

fn create_option_improved_noise_array_from_jobject(env: &mut JNIEnv, noise_levels: JObjectArray) -> Result<Vec<Option<ImprovedNoise>>, Box<dyn std::error::Error>> {
    let size = env.get_array_length(&noise_levels)?;
    let mut result = Vec::with_capacity(size as usize);
    for i in 0..size {
        let value = env.get_object_array_element(&noise_levels, i)?;
        if value.is_null() {
            result.push(None);
            continue;
        }
        let value = create_improved_noise_from_jobject(env, value)?;
        result.push(Some(value));
    }
    Ok(result)
}

// public final class ImprovedNoise {
//     private static final float SHIFT_UP_EPSILON = 1.0E-7F;
//     private final byte[] p;
//     public final double xo;
//     public final double yo;
//     public final double zo;

fn create_improved_noise_from_jobject(env: &mut JNIEnv, noise: JObject) -> Result<ImprovedNoise, Box<dyn std::error::Error>> {
    let p = env.get_field(&noise, "p", "[B")?.l()?;
    let p = create_byte_array_from_jobject(env, p.into())?;
    let p: [u8; 256] = p.try_into().map_err(|v: Vec<u8>| format!("Expected 256 bytes, got {}", v.len()))?;
    let xo = env.get_field(&noise, "xo", "D")?.d()?;
    let yo = env.get_field(&noise, "yo", "D")?.d()?;
    let zo = env.get_field(&noise, "zo", "D")?.d()?;
    Ok(ImprovedNoise::new_raw(p, xo, yo, zo))
}

fn create_byte_array_from_jobject(env: &mut JNIEnv, p: JByteArray) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let size = env.get_array_length(&p)?;
    let mut result = vec![0; size as usize];
    env.get_byte_array_region(&p, 0, result.as_mut_slice())?;
    Ok(unsafe { std::mem::transmute(result) } )
}

fn create_double_list_from_jobject(env: &mut JNIEnv, amplitudes: JObject) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let size = env.call_method(&amplitudes, "size", "()I", &[])?.i()?;
    let mut result = Vec::with_capacity(size as usize);
    for i in 0..size {
        let value = env.call_method(&amplitudes, "getDouble", "(I)D", &[i.into()])?.d()?;
        result.push(value);
    }
    Ok(result)
}


