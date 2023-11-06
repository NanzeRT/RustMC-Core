use std::{sync::atomic::{AtomicI64, Ordering}, time::SystemTime};

pub fn mix_stafford_13(seed: i64) -> i64 {
    let mut seed = seed;
    seed ^= seed >> 30;
    seed = seed.wrapping_mul(-4658895280553007687);
    seed ^= seed >> 27;
    seed = seed.wrapping_mul(-7723592293110705685);
    seed ^= seed >> 31;
    seed
}

pub fn upgrade_seed_to_128bit_unmixed(seed: i64) -> (i64, i64) {
    let l = seed ^ 7640891576956012809;
    let m = l.wrapping_add(-7046029254386353131);
    (l, m)
}

pub fn upgrade_seed_to_128bit(seed: i64) -> (i64, i64) {
    let (l, m) = upgrade_seed_to_128bit_unmixed(seed);
    (mix_stafford_13(l), mix_stafford_13(m))
}

pub fn seed_from_hash_of(seed: &str) -> (i64, i64) {
    let bs = md5::compute(seed.as_bytes());
    let l = i64::from_le_bytes(bs.0[0..8].try_into().unwrap());
    let m = i64::from_le_bytes(bs.0[8..16].try_into().unwrap());
    (l, m)
}

static SEED_UNIQUIFIER: AtomicI64 = AtomicI64::new(8682522807148012);

pub fn generate_unique_seed() -> i64 {
    SEED_UNIQUIFIER.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |seed_uniquifier| {
        seed_uniquifier.wrapping_mul(1181783497276652981).into()
    }).unwrap() ^ SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as i64
}
