use super::random_source::RandomSource;

pub trait PositionalRandomFactory {
    // fn at(&self, pos: BlockPos) -> RandomSource {
    //     self.at(pos.x, pos.y, pos.z)
    // }
    //
    // fn from_hash_of(&self, seed: ResourceLocation) -> RandomSource {
    //     self.from_hash_of(seed.to_string())
    // }

    fn create_from_hash_of(&self, seed: &str) -> Box<dyn RandomSource>;

    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource>;
    
}
