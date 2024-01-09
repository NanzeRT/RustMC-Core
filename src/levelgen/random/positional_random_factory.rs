use super::{
    legacy_random_source::LegacyPositionalRandomFactory, random_source::RandomSource,
    xoroshiro_random_source::XoroshiroPositionalRandomFactory,
};

pub trait PositionalRandomFactory {
    // fn at(&self, pos: BlockPos) -> RandomSource {
    //     self.at(pos.x, pos.y, pos.z)
    // }
    //
    // fn from_hash_of(&self, seed: ResourceLocation) -> RandomSource {
    //     self.from_hash_of(seed.to_string())
    // }
    type Target: RandomSource;

    fn create_from_hash_of(&self, seed: &str) -> Self::Target;

    fn at(&self, x: i32, y: i32, z: i32) -> Self::Target;
}

pub enum PositionalRandomFactoryVariants {
    Xoroshiro(XoroshiroPositionalRandomFactory),
    Legacy(LegacyPositionalRandomFactory),
}
