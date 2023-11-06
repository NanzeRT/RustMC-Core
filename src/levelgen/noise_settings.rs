
// public record NoiseSettings(int minY, int height, int noiseSizeHorizontal, int noiseSizeVertical) {
//     public static final Codec<NoiseSettings> CODEC = RecordCodecBuilder.<NoiseSettings>create((instance) -> {
//         return instance.group(Codec.intRange(DimensionType.MIN_Y, DimensionType.MAX_Y).fieldOf("min_y").forGetter(NoiseSettings::minY), Codec.intRange(0, DimensionType.Y_SIZE).fieldOf("height").forGetter(NoiseSettings::height), Codec.intRange(1, 4).fieldOf("size_horizontal").forGetter(NoiseSettings::noiseSizeHorizontal), Codec.intRange(1, 4).fieldOf("size_vertical").forGetter(NoiseSettings::noiseSizeVertical)).apply(instance, NoiseSettings::new);
//     }).comapFlatMap(NoiseSettings::guardY, Function.identity());
//     protected static final NoiseSettings OVERWORLD_NOISE_SETTINGS = create(-64, 384, 1, 2);
//     protected static final NoiseSettings NETHER_NOISE_SETTINGS = create(0, 128, 1, 2);
//     protected static final NoiseSettings END_NOISE_SETTINGS = create(0, 128, 2, 1);
//     protected static final NoiseSettings CAVES_NOISE_SETTINGS = create(-64, 192, 1, 2);
//     protected static final NoiseSettings FLOATING_ISLANDS_NOISE_SETTINGS = create(0, 256, 2, 1);
//
//     private static DataResult<NoiseSettings> guardY(NoiseSettings config) {
//         if (config.minY() + config.height() > DimensionType.MAX_Y + 1) {
//             return DataResult.error(() -> {
//                 return "min_y + height cannot be higher than: " + (DimensionType.MAX_Y + 1);
//             });
//         } else if (config.height() % 16 != 0) {
//             return DataResult.error(() -> {
//                 return "height has to be a multiple of 16";
//             });
//         } else {
//             return config.minY() % 16 != 0 ? DataResult.error(() -> {
//                 return "min_y has to be a multiple of 16";
//             }) : DataResult.success(config);
//         }
//     }
//
//     public static NoiseSettings create(int minimumY, int height, int horizontalSize, int verticalSize) {
//         NoiseSettings noiseSettings = new NoiseSettings(minimumY, height, horizontalSize, verticalSize);
//         guardY(noiseSettings).error().ifPresent((result) -> {
//             throw new IllegalStateException(result.message());
//         });
//         return noiseSettings;
//     }
//
//     public int getCellHeight() {
//         return QuartPos.toBlock(this.noiseSizeVertical());
//     }
//
//     public int getCellWidth() {
//         return QuartPos.toBlock(this.noiseSizeHorizontal());
//     }
//
//     public NoiseSettings clampToHeightAccessor(LevelHeightAccessor world) {
//         int i = Math.max(this.minY, world.getMinBuildHeight());
//         int j = Math.min(this.minY + this.height, world.getMaxBuildHeight()) - i;
//         return new NoiseSettings(i, j, this.noiseSizeHorizontal, this.noiseSizeVertical);
//     }
// }

// pub struct NoiseSettings {
//     min_y: i32,
//     height: i32,
//     noise_size_horizontal: i32,
//     noise_size_vertical: i32,
// }
//
// impl NoiseSettings {
//     pub fn new(min_y: i32, height: i32, noise_size_horizontal: i32, noise_size_vertical: i32) -> Self {
//         Self {
//             min_y,
//             height,
//             noise_size_horizontal,
//             noise_size_vertical,
//         }
//     }
//
//     // pub fn get_cell_height(&self) -> i32 {
//     //     self.noise_size_vertical
//     // }
//     //
//     // pub fn get_cell_width(&self) -> i32 {
//     //     self.noise_size_horizontal
//     // }
//     //
//     // pub fn clamp_to_height_accessor(&self, world: &dyn LevelHeightAccessor) -> Self {
//     //     let i = std::cmp::max(self.min_y, world.get_min_build_height());
//     //     let j = std::cmp::min(self.min_y + self.height, world.get_max_build_height()) - i;
//     //     Self::new(i, j, self.noise_size_horizontal, self.noise_size_vertical)
//     // }
// }
