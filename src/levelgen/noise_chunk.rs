
// public class NoiseChunk implements DensityFunction.ContextProvider, DensityFunction.FunctionContext {
//     private final NoiseSettings noiseSettings;
//     final int cellCountXZ;
//     final int cellCountY;
//     final int cellNoiseMinY;
//     private final int firstCellX;
//     private final int firstCellZ;
//     final int firstNoiseX;
//     final int firstNoiseZ;
//     final List<NoiseChunk.NoiseInterpolator> interpolators;
//     final List<NoiseChunk.CacheAllInCell> cellCaches;
//     private final Map<DensityFunction, DensityFunction> wrapped = new HashMap<>();
//     private final Long2IntMap preliminarySurfaceLevel = new Long2IntOpenHashMap();
//     private final Aquifer aquifer;
//     private final DensityFunction initialDensityNoJaggedness;
//     private final NoiseChunk.BlockStateFiller blockStateRule;
//     private final Blender blender;
//     private final NoiseChunk.FlatCache blendAlpha;
//     private final NoiseChunk.FlatCache blendOffset;
//     private final DensityFunctions.BeardifierOrMarker beardifier;
//     private long lastBlendingDataPos = ChunkPos.INVALID_CHUNK_POS;
//     private Blender.BlendingOutput lastBlendingOutput = new Blender.BlendingOutput(1.0D, 0.0D);
//     final int noiseSizeXZ;
//     final int cellWidth;
//     final int cellHeight;
//     boolean interpolating;
//     boolean fillingCell;
//     private int cellStartBlockX;
//     int cellStartBlockY;
//     private int cellStartBlockZ;
//     int inCellX;
//     int inCellY;
//     int inCellZ;
//     long interpolationCounter;
//     long arrayInterpolationCounter;
//     int arrayIndex;
//     private final DensityFunction.ContextProvider sliceFillingContextProvider = new DensityFunction.ContextProvider() {
//         @Override
//         public DensityFunction.FunctionContext forIndex(int index) {
//             NoiseChunk.this.cellStartBlockY = (index + NoiseChunk.this.cellNoiseMinY) * NoiseChunk.this.cellHeight;
//             ++NoiseChunk.this.interpolationCounter;
//             NoiseChunk.this.inCellY = 0;
//             NoiseChunk.this.arrayIndex = index;
//             return NoiseChunk.this;
//         }
//
//         @Override
//         public void fillAllDirectly(double[] densities, DensityFunction densityFunction) {
//             for(int i = 0; i < NoiseChunk.this.cellCountY + 1; ++i) {
//                 NoiseChunk.this.cellStartBlockY = (i + NoiseChunk.this.cellNoiseMinY) * NoiseChunk.this.cellHeight;
//                 ++NoiseChunk.this.interpolationCounter;
//                 NoiseChunk.this.inCellY = 0;
//                 NoiseChunk.this.arrayIndex = i;
//                 densities[i] = densityFunction.compute(NoiseChunk.this);
//             }
//
//         }
//     };
//
//     public static NoiseChunk forChunk(ChunkAccess chunk, RandomState noiseConfig, DensityFunctions.BeardifierOrMarker beardifying, NoiseGeneratorSettings chunkGeneratorSettings, Aquifer.FluidPicker fluidLevelSampler, Blender blender) {
//         NoiseSettings noiseSettings = chunkGeneratorSettings.noiseSettings().clampToHeightAccessor(chunk);
//         ChunkPos chunkPos = chunk.getPos();
//         int i = 16 / noiseSettings.getCellWidth();
//         return new NoiseChunk(i, noiseConfig, chunkPos.getMinBlockX(), chunkPos.getMinBlockZ(), noiseSettings, beardifying, chunkGeneratorSettings, fluidLevelSampler, blender);
//     }
//
//     public NoiseChunk(int horizontalCellCount, RandomState noiseConfig, int startBlockX, int startBlockZ, NoiseSettings generationShapeConfig, DensityFunctions.BeardifierOrMarker beardifying, NoiseGeneratorSettings chunkGeneratorSettings, Aquifer.FluidPicker fluidLevelSampler, Blender blender) {
//         this.noiseSettings = generationShapeConfig;
//         this.cellWidth = generationShapeConfig.getCellWidth();
//         this.cellHeight = generationShapeConfig.getCellHeight();
//         this.cellCountXZ = horizontalCellCount;
//         this.cellCountY = Mth.floorDiv(generationShapeConfig.height(), this.cellHeight);
//         this.cellNoiseMinY = Mth.floorDiv(generationShapeConfig.minY(), this.cellHeight);
//         this.firstCellX = Math.floorDiv(startBlockX, this.cellWidth);
//         this.firstCellZ = Math.floorDiv(startBlockZ, this.cellWidth);
//         this.interpolators = Lists.newArrayList();
//         this.cellCaches = Lists.newArrayList();
//         this.firstNoiseX = QuartPos.fromBlock(startBlockX);
//         this.firstNoiseZ = QuartPos.fromBlock(startBlockZ);
//         this.noiseSizeXZ = QuartPos.fromBlock(horizontalCellCount * this.cellWidth);
//         this.blender = blender;
//         this.beardifier = beardifying;
//         this.blendAlpha = new NoiseChunk.FlatCache(new NoiseChunk.BlendAlpha(), false);
//         this.blendOffset = new NoiseChunk.FlatCache(new NoiseChunk.BlendOffset(), false);
//
//         for(int i = 0; i <= this.noiseSizeXZ; ++i) {
//             int j = this.firstNoiseX + i;
//             int k = QuartPos.toBlock(j);
//
//             for(int l = 0; l <= this.noiseSizeXZ; ++l) {
//                 int m = this.firstNoiseZ + l;
//                 int n = QuartPos.toBlock(m);
//                 Blender.BlendingOutput blendingOutput = blender.blendOffsetAndFactor(k, n);
//                 this.blendAlpha.values[i][l] = blendingOutput.alpha();
//                 this.blendOffset.values[i][l] = blendingOutput.blendingOffset();
//             }
//         }
//
//         NoiseRouter noiseRouter = noiseConfig.router();
//         NoiseRouter noiseRouter2 = noiseRouter.mapAll(this::wrap);
//         if (!chunkGeneratorSettings.isAquifersEnabled()) {
//             this.aquifer = Aquifer.createDisabled(fluidLevelSampler);
//         } else {
//             int o = SectionPos.blockToSectionCoord(startBlockX);
//             int p = SectionPos.blockToSectionCoord(startBlockZ);
//             this.aquifer = Aquifer.create(this, new ChunkPos(o, p), noiseRouter2, noiseConfig.aquiferRandom(), generationShapeConfig.minY(), generationShapeConfig.height(), fluidLevelSampler);
//         }
//
//         ImmutableList.Builder<NoiseChunk.BlockStateFiller> builder = ImmutableList.builder();
//         DensityFunction densityFunction = DensityFunctions.cacheAllInCell(DensityFunctions.add(noiseRouter2.finalDensity(), DensityFunctions.BeardifierMarker.INSTANCE)).mapAll(this::wrap);
//         builder.add((pos) -> {
//             return this.aquifer.computeSubstance(pos, densityFunction.compute(pos));
//         });
//         if (chunkGeneratorSettings.oreVeinsEnabled()) {
//             builder.add(OreVeinifier.create(noiseRouter2.veinToggle(), noiseRouter2.veinRidged(), noiseRouter2.veinGap(), noiseConfig.oreRandom()));
//         }
//
//         this.blockStateRule = new MaterialRuleList(builder.build());
//         this.initialDensityNoJaggedness = noiseRouter2.initialDensityWithoutJaggedness();
//     }
//
//     protected Climate.Sampler cachedClimateSampler(NoiseRouter noiseRouter, List<Climate.ParameterPoint> spawnTarget) {
//         return new Climate.Sampler(noiseRouter.temperature().mapAll(this::wrap), noiseRouter.vegetation().mapAll(this::wrap), noiseRouter.continents().mapAll(this::wrap), noiseRouter.erosion().mapAll(this::wrap), noiseRouter.depth().mapAll(this::wrap), noiseRouter.ridges().mapAll(this::wrap), spawnTarget);
//     }
//
//     @Nullable
//     protected BlockState getInterpolatedState() {
//         return this.blockStateRule.calculate(this);
//     }
//
//     @Override
//     public int blockX() {
//         return this.cellStartBlockX + this.inCellX;
//     }
//
//     @Override
//     public int blockY() {
//         return this.cellStartBlockY + this.inCellY;
//     }
//
//     @Override
//     public int blockZ() {
//         return this.cellStartBlockZ + this.inCellZ;
//     }
//
//     public int preliminarySurfaceLevel(int blockX, int blockZ) {
//         int i = QuartPos.toBlock(QuartPos.fromBlock(blockX));
//         int j = QuartPos.toBlock(QuartPos.fromBlock(blockZ));
//         return this.preliminarySurfaceLevel.computeIfAbsent(ColumnPos.asLong(i, j), this::computePreliminarySurfaceLevel);
//     }
//
//     private int computePreliminarySurfaceLevel(long columnPos) {
//         int i = ColumnPos.getX(columnPos);
//         int j = ColumnPos.getZ(columnPos);
//         int k = this.noiseSettings.minY();
//
//         for(int l = k + this.noiseSettings.height(); l >= k; l -= this.cellHeight) {
//             if (this.initialDensityNoJaggedness.compute(new DensityFunction.SinglePointContext(i, l, j)) > 0.390625D) {
//                 return l;
//             }
//         }
//
//         return Integer.MAX_VALUE;
//     }
//
//     @Override
//     public Blender getBlender() {
//         return this.blender;
//     }
//
//     private void fillSlice(boolean start, int cellX) {
//         this.cellStartBlockX = cellX * this.cellWidth;
//         this.inCellX = 0;
//
//         for(int i = 0; i < this.cellCountXZ + 1; ++i) {
//             int j = this.firstCellZ + i;
//             this.cellStartBlockZ = j * this.cellWidth;
//             this.inCellZ = 0;
//             ++this.arrayInterpolationCounter;
//
//             for(NoiseChunk.NoiseInterpolator noiseInterpolator : this.interpolators) {
//                 double[] ds = (start ? noiseInterpolator.slice0 : noiseInterpolator.slice1)[i];
//                 noiseInterpolator.fillArray(ds, this.sliceFillingContextProvider);
//             }
//         }
//
//         ++this.arrayInterpolationCounter;
//     }
//
//     public void initializeForFirstCellX() {
//         if (this.interpolating) {
//             throw new IllegalStateException("Staring interpolation twice");
//         } else {
//             this.interpolating = true;
//             this.interpolationCounter = 0L;
//             this.fillSlice(true, this.firstCellX);
//         }
//     }
//
//     public void advanceCellX(int cellX) {
//         this.fillSlice(false, this.firstCellX + cellX + 1);
//         this.cellStartBlockX = (this.firstCellX + cellX) * this.cellWidth;
//     }
//
//     @Override
//     public NoiseChunk forIndex(int i) {
//         int j = Math.floorMod(i, this.cellWidth);
//         int k = Math.floorDiv(i, this.cellWidth);
//         int l = Math.floorMod(k, this.cellWidth);
//         int m = this.cellHeight - 1 - Math.floorDiv(k, this.cellWidth);
//         this.inCellX = l;
//         this.inCellY = m;
//         this.inCellZ = j;
//         this.arrayIndex = i;
//         return this;
//     }
//
//     @Override
//     public void fillAllDirectly(double[] densities, DensityFunction densityFunction) {
//         this.arrayIndex = 0;
//
//         for(int i = this.cellHeight - 1; i >= 0; --i) {
//             this.inCellY = i;
//
//             for(int j = 0; j < this.cellWidth; ++j) {
//                 this.inCellX = j;
//
//                 for(int k = 0; k < this.cellWidth; ++k) {
//                     this.inCellZ = k;
//                     densities[this.arrayIndex++] = densityFunction.compute(this);
//                 }
//             }
//         }
//
//     }
//
//     public void selectCellYZ(int cellY, int cellZ) {
//         this.interpolators.forEach((interpolator) -> {
//             interpolator.selectCellYZ(cellY, cellZ);
//         });
//         this.fillingCell = true;
//         this.cellStartBlockY = (cellY + this.cellNoiseMinY) * this.cellHeight;
//         this.cellStartBlockZ = (this.firstCellZ + cellZ) * this.cellWidth;
//         ++this.arrayInterpolationCounter;
//
//         for(NoiseChunk.CacheAllInCell cacheAllInCell : this.cellCaches) {
//             cacheAllInCell.noiseFiller.fillArray(cacheAllInCell.values, this);
//         }
//
//         ++this.arrayInterpolationCounter;
//         this.fillingCell = false;
//     }
//
//     public void updateForY(int blockY, double deltaY) {
//         this.inCellY = blockY - this.cellStartBlockY;
//         this.interpolators.forEach((interpolator) -> {
//             interpolator.updateForY(deltaY);
//         });
//     }
//
//     public void updateForX(int blockX, double deltaX) {
//         this.inCellX = blockX - this.cellStartBlockX;
//         this.interpolators.forEach((interpolator) -> {
//             interpolator.updateForX(deltaX);
//         });
//     }
//
//     public void updateForZ(int blockZ, double deltaZ) {
//         this.inCellZ = blockZ - this.cellStartBlockZ;
//         ++this.interpolationCounter;
//         this.interpolators.forEach((interpolator) -> {
//             interpolator.updateForZ(deltaZ);
//         });
//     }
//
//     public void stopInterpolation() {
//         if (!this.interpolating) {
//             throw new IllegalStateException("Staring interpolation twice");
//         } else {
//             this.interpolating = false;
//         }
//     }
//
//     public void swapSlices() {
//         this.interpolators.forEach(NoiseChunk.NoiseInterpolator::swapSlices);
//     }
//
//     public Aquifer aquifer() {
//         return this.aquifer;
//     }
//
//     protected int cellWidth() {
//         return this.cellWidth;
//     }
//
//     protected int cellHeight() {
//         return this.cellHeight;
//     }
//
//     Blender.BlendingOutput getOrComputeBlendingOutput(int blockX, int blockZ) {
//         long l = ChunkPos.asLong(blockX, blockZ);
//         if (this.lastBlendingDataPos == l) {
//             return this.lastBlendingOutput;
//         } else {
//             this.lastBlendingDataPos = l;
//             Blender.BlendingOutput blendingOutput = this.blender.blendOffsetAndFactor(blockX, blockZ);
//             this.lastBlendingOutput = blendingOutput;
//             return blendingOutput;
//         }
//     }
//
//     protected DensityFunction wrap(DensityFunction function) {
//         return this.wrapped.computeIfAbsent(function, this::wrapNew);
//     }
//
//     private DensityFunction wrapNew(DensityFunction function) {
//         if (function instanceof DensityFunctions.Marker) {
//             DensityFunctions.Marker marker = (DensityFunctions.Marker)function;
//             Object var10000;
//             switch (marker.type()) {
//                 case Interpolated:
//                     var10000 = new NoiseChunk.NoiseInterpolator(marker.wrapped());
//                     break;
//                 case FlatCache:
//                     var10000 = new NoiseChunk.FlatCache(marker.wrapped(), true);
//                     break;
//                 case Cache2D:
//                     var10000 = new NoiseChunk.Cache2D(marker.wrapped());
//                     break;
//                 case CacheOnce:
//                     var10000 = new NoiseChunk.CacheOnce(marker.wrapped());
//                     break;
//                 case CacheAllInCell:
//                     var10000 = new NoiseChunk.CacheAllInCell(marker.wrapped());
//                     break;
//                 default:
//                     throw new IncompatibleClassChangeError();
//             }
//
//             return (DensityFunction)var10000;
//         } else {
//             if (this.blender != Blender.empty()) {
//                 if (function == DensityFunctions.BlendAlpha.INSTANCE) {
//                     return this.blendAlpha;
//                 }
//
//                 if (function == DensityFunctions.BlendOffset.INSTANCE) {
//                     return this.blendOffset;
//                 }
//             }
//
//             if (function == DensityFunctions.BeardifierMarker.INSTANCE) {
//                 return this.beardifier;
//             } else if (function instanceof DensityFunctions.HolderHolder) {
//                 DensityFunctions.HolderHolder holderHolder = (DensityFunctions.HolderHolder)function;
//                 return holderHolder.function().value();
//             } else {
//                 return function;
//             }
//         }
//     }
//
//     class BlendAlpha implements NoiseChunk.NoiseChunkDensityFunction {
//         @Override
//         public DensityFunction wrapped() {
//             return DensityFunctions.BlendAlpha.INSTANCE;
//         }
//
//         @Override
//         public DensityFunction mapAll(DensityFunction.Visitor visitor) {
//             return this.wrapped().mapAll(visitor);
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             return NoiseChunk.this.getOrComputeBlendingOutput(pos.blockX(), pos.blockZ()).alpha();
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             applier.fillAllDirectly(densities, this);
//         }
//
//         @Override
//         public double minValue() {
//             return 0.0D;
//         }
//
//         @Override
//         public double maxValue() {
//             return 1.0D;
//         }
//
//         @Override
//         public KeyDispatchDataCodec<? extends DensityFunction> codec() {
//             return DensityFunctions.BlendAlpha.CODEC;
//         }
//     }
//
//     class BlendOffset implements NoiseChunk.NoiseChunkDensityFunction {
//         @Override
//         public DensityFunction wrapped() {
//             return DensityFunctions.BlendOffset.INSTANCE;
//         }
//
//         @Override
//         public DensityFunction mapAll(DensityFunction.Visitor visitor) {
//             return this.wrapped().mapAll(visitor);
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             return NoiseChunk.this.getOrComputeBlendingOutput(pos.blockX(), pos.blockZ()).blendingOffset();
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             applier.fillAllDirectly(densities, this);
//         }
//
//         @Override
//         public double minValue() {
//             return Double.NEGATIVE_INFINITY;
//         }
//
//         @Override
//         public double maxValue() {
//             return Double.POSITIVE_INFINITY;
//         }
//
//         @Override
//         public KeyDispatchDataCodec<? extends DensityFunction> codec() {
//             return DensityFunctions.BlendOffset.CODEC;
//         }
//     }
//
//     @FunctionalInterface
//     public interface BlockStateFiller {
//         @Nullable
//         BlockState calculate(DensityFunction.FunctionContext pos);
//     }
//
//     static class Cache2D implements DensityFunctions.MarkerOrMarked, NoiseChunk.NoiseChunkDensityFunction {
//         private final DensityFunction function;
//         private long lastPos2D = ChunkPos.INVALID_CHUNK_POS;
//         private double lastValue;
//
//         Cache2D(DensityFunction delegate) {
//             this.function = delegate;
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             int i = pos.blockX();
//             int j = pos.blockZ();
//             long l = ChunkPos.asLong(i, j);
//             if (this.lastPos2D == l) {
//                 return this.lastValue;
//             } else {
//                 this.lastPos2D = l;
//                 double d = this.function.compute(pos);
//                 this.lastValue = d;
//                 return d;
//             }
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             this.function.fillArray(densities, applier);
//         }
//
//         @Override
//         public DensityFunction wrapped() {
//             return this.function;
//         }
//
//         @Override
//         public DensityFunctions.Marker.Type type() {
//             return DensityFunctions.Marker.Type.Cache2D;
//         }
//     }
//
//     class CacheAllInCell implements DensityFunctions.MarkerOrMarked, NoiseChunk.NoiseChunkDensityFunction {
//         final DensityFunction noiseFiller;
//         final double[] values;
//
//         CacheAllInCell(DensityFunction delegate) {
//             this.noiseFiller = delegate;
//             this.values = new double[NoiseChunk.this.cellWidth * NoiseChunk.this.cellWidth * NoiseChunk.this.cellHeight];
//             NoiseChunk.this.cellCaches.add(this);
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             if (pos != NoiseChunk.this) {
//                 return this.noiseFiller.compute(pos);
//             } else if (!NoiseChunk.this.interpolating) {
//                 throw new IllegalStateException("Trying to sample interpolator outside the interpolation loop");
//             } else {
//                 int i = NoiseChunk.this.inCellX;
//                 int j = NoiseChunk.this.inCellY;
//                 int k = NoiseChunk.this.inCellZ;
//                 return i >= 0 && j >= 0 && k >= 0 && i < NoiseChunk.this.cellWidth && j < NoiseChunk.this.cellHeight && k < NoiseChunk.this.cellWidth ? this.values[((NoiseChunk.this.cellHeight - 1 - j) * NoiseChunk.this.cellWidth + i) * NoiseChunk.this.cellWidth + k] : this.noiseFiller.compute(pos);
//             }
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             applier.fillAllDirectly(densities, this);
//         }
//
//         @Override
//         public DensityFunction wrapped() {
//             return this.noiseFiller;
//         }
//
//         @Override
//         public DensityFunctions.Marker.Type type() {
//             return DensityFunctions.Marker.Type.CacheAllInCell;
//         }
//     }
//
//     class CacheOnce implements DensityFunctions.MarkerOrMarked, NoiseChunk.NoiseChunkDensityFunction {
//         private final DensityFunction function;
//         private long lastCounter;
//         private long lastArrayCounter;
//         private double lastValue;
//         @Nullable
//         private double[] lastArray;
//
//         CacheOnce(DensityFunction delegate) {
//             this.function = delegate;
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             if (pos != NoiseChunk.this) {
//                 return this.function.compute(pos);
//             } else if (this.lastArray != null && this.lastArrayCounter == NoiseChunk.this.arrayInterpolationCounter) {
//                 return this.lastArray[NoiseChunk.this.arrayIndex];
//             } else if (this.lastCounter == NoiseChunk.this.interpolationCounter) {
//                 return this.lastValue;
//             } else {
//                 this.lastCounter = NoiseChunk.this.interpolationCounter;
//                 double d = this.function.compute(pos);
//                 this.lastValue = d;
//                 return d;
//             }
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             if (this.lastArray != null && this.lastArrayCounter == NoiseChunk.this.arrayInterpolationCounter) {
//                 System.arraycopy(this.lastArray, 0, densities, 0, densities.length);
//             } else {
//                 this.wrapped().fillArray(densities, applier);
//                 if (this.lastArray != null && this.lastArray.length == densities.length) {
//                     System.arraycopy(densities, 0, this.lastArray, 0, densities.length);
//                 } else {
//                     this.lastArray = (double[])densities.clone();
//                 }
//
//                 this.lastArrayCounter = NoiseChunk.this.arrayInterpolationCounter;
//             }
//         }
//
//         @Override
//         public DensityFunction wrapped() {
//             return this.function;
//         }
//
//         @Override
//         public DensityFunctions.Marker.Type type() {
//             return DensityFunctions.Marker.Type.CacheOnce;
//         }
//     }
//
//     class FlatCache implements DensityFunctions.MarkerOrMarked, NoiseChunk.NoiseChunkDensityFunction {
//         private final DensityFunction noiseFiller;
//         final double[][] values;
//
//         FlatCache(DensityFunction delegate, boolean sample) {
//             this.noiseFiller = delegate;
//             this.values = new double[NoiseChunk.this.noiseSizeXZ + 1][NoiseChunk.this.noiseSizeXZ + 1];
//             if (sample) {
//                 for(int i = 0; i <= NoiseChunk.this.noiseSizeXZ; ++i) {
//                     int j = NoiseChunk.this.firstNoiseX + i;
//                     int k = QuartPos.toBlock(j);
//
//                     for(int l = 0; l <= NoiseChunk.this.noiseSizeXZ; ++l) {
//                         int m = NoiseChunk.this.firstNoiseZ + l;
//                         int n = QuartPos.toBlock(m);
//                         this.values[i][l] = delegate.compute(new DensityFunction.SinglePointContext(k, 0, n));
//                     }
//                 }
//             }
//
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             int i = QuartPos.fromBlock(pos.blockX());
//             int j = QuartPos.fromBlock(pos.blockZ());
//             int k = i - NoiseChunk.this.firstNoiseX;
//             int l = j - NoiseChunk.this.firstNoiseZ;
//             int m = this.values.length;
//             return k >= 0 && l >= 0 && k < m && l < m ? this.values[k][l] : this.noiseFiller.compute(pos);
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             applier.fillAllDirectly(densities, this);
//         }
//
//         @Override
//         public DensityFunction wrapped() {
//             return this.noiseFiller;
//         }
//
//         @Override
//         public DensityFunctions.Marker.Type type() {
//             return DensityFunctions.Marker.Type.FlatCache;
//         }
//     }
//
//     interface NoiseChunkDensityFunction extends DensityFunction {
//         DensityFunction wrapped();
//
//         @Override
//         default double minValue() {
//             return this.wrapped().minValue();
//         }
//
//         @Override
//         default double maxValue() {
//             return this.wrapped().maxValue();
//         }
//     }
//
//     public class NoiseInterpolator implements DensityFunctions.MarkerOrMarked, NoiseChunk.NoiseChunkDensityFunction {
//         double[][] slice0;
//         double[][] slice1;
//         private final DensityFunction noiseFiller;
//         private double noise000;
//         private double noise001;
//         private double noise100;
//         private double noise101;
//         private double noise010;
//         private double noise011;
//         private double noise110;
//         private double noise111;
//         private double valueXZ00;
//         private double valueXZ10;
//         private double valueXZ01;
//         private double valueXZ11;
//         private double valueZ0;
//         private double valueZ1;
//         private double value;
//
//         NoiseInterpolator(DensityFunction delegate) {
//             this.noiseFiller = delegate;
//             this.slice0 = this.allocateSlice(NoiseChunk.this.cellCountY, NoiseChunk.this.cellCountXZ);
//             this.slice1 = this.allocateSlice(NoiseChunk.this.cellCountY, NoiseChunk.this.cellCountXZ);
//             NoiseChunk.this.interpolators.add(this);
//         }
//
//         private double[][] allocateSlice(int sizeZ, int sizeX) {
//             int i = sizeX + 1;
//             int j = sizeZ + 1;
//             double[][] ds = new double[i][j];
//
//             for(int k = 0; k < i; ++k) {
//                 ds[k] = new double[j];
//             }
//
//             return ds;
//         }
//
//         void selectCellYZ(int cellY, int cellZ) {
//             this.noise000 = this.slice0[cellZ][cellY];
//             this.noise001 = this.slice0[cellZ + 1][cellY];
//             this.noise100 = this.slice1[cellZ][cellY];
//             this.noise101 = this.slice1[cellZ + 1][cellY];
//             this.noise010 = this.slice0[cellZ][cellY + 1];
//             this.noise011 = this.slice0[cellZ + 1][cellY + 1];
//             this.noise110 = this.slice1[cellZ][cellY + 1];
//             this.noise111 = this.slice1[cellZ + 1][cellY + 1];
//         }
//
//         void updateForY(double deltaY) {
//             this.valueXZ00 = Mth.lerp(deltaY, this.noise000, this.noise010);
//             this.valueXZ10 = Mth.lerp(deltaY, this.noise100, this.noise110);
//             this.valueXZ01 = Mth.lerp(deltaY, this.noise001, this.noise011);
//             this.valueXZ11 = Mth.lerp(deltaY, this.noise101, this.noise111);
//         }
//
//         void updateForX(double deltaX) {
//             this.valueZ0 = Mth.lerp(deltaX, this.valueXZ00, this.valueXZ10);
//             this.valueZ1 = Mth.lerp(deltaX, this.valueXZ01, this.valueXZ11);
//         }
//
//         void updateForZ(double deltaZ) {
//             this.value = Mth.lerp(deltaZ, this.valueZ0, this.valueZ1);
//         }
//
//         @Override
//         public double compute(DensityFunction.FunctionContext pos) {
//             if (pos != NoiseChunk.this) {
//                 return this.noiseFiller.compute(pos);
//             } else if (!NoiseChunk.this.interpolating) {
//                 throw new IllegalStateException("Trying to sample interpolator outside the interpolation loop");
//             } else {
//                 return NoiseChunk.this.fillingCell ? Mth.lerp3((double)NoiseChunk.this.inCellX / (double)NoiseChunk.this.cellWidth, (double)NoiseChunk.this.inCellY / (double)NoiseChunk.this.cellHeight, (double)NoiseChunk.this.inCellZ / (double)NoiseChunk.this.cellWidth, this.noise000, this.noise100, this.noise010, this.noise110, this.noise001, this.noise101, this.noise011, this.noise111) : this.value;
//             }
//         }
//
//         @Override
//         public void fillArray(double[] densities, DensityFunction.ContextProvider applier) {
//             if (NoiseChunk.this.fillingCell) {
//                 applier.fillAllDirectly(densities, this);
//             } else {
//                 this.wrapped().fillArray(densities, applier);
//             }
//         }
//
//         @Override
//         public DensityFunction wrapped() {
//             return this.noiseFiller;
//         }
//
//         private void swapSlices() {
//             double[][] ds = this.slice0;
//             this.slice0 = this.slice1;
//             this.slice1 = ds;
//         }
//
//         @Override
//         public DensityFunctions.Marker.Type type() {
//             return DensityFunctions.Marker.Type.Interpolated;
//         }
//     }
// }

use std::{collections::HashMap, fmt::{Formatter, Debug}};

use jni::{JNIEnv, objects::{WeakRef, JMethodID, JValue}, signature::{Primitive, ReturnType}};
use once_cell::sync::OnceCell;

use super::density_function::FunctionContext;

pub struct NoiseChunk {
    pub cell_start_block_x: i32,
    pub cell_start_block_y: i32,
    pub cell_start_block_z: i32,
    pub in_cell_x: i32,
    pub in_cell_y: i32,
    pub in_cell_z: i32,
    preliminary_surface_level: HashMap<(i32, i32), i32>,
    this: WeakRef,
}

impl Debug for NoiseChunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NoiseChunk")
            .field("cell_start_block_x", &self.cell_start_block_x)
            .field("cell_start_block_y", &self.cell_start_block_y)
            .field("cell_start_block_z", &self.cell_start_block_z)
            .field("in_cell_x", &self.in_cell_x)
            .field("in_cell_y", &self.in_cell_y)
            .field("in_cell_z", &self.in_cell_z)
            .field("preliminary_surface_level", &self.preliminary_surface_level)
            .finish()
    }
}

impl NoiseChunk {
    pub fn new(
        this: WeakRef,
        cell_start_block_x: i32,
        cell_start_block_y: i32,
        cell_start_block_z: i32,
        in_cell_x: i32,
        in_cell_y: i32,
        in_cell_z: i32,
    ) -> Self {
        Self {
            this,
            cell_start_block_x,
            cell_start_block_y,
            cell_start_block_z,
            in_cell_x,
            in_cell_y,
            in_cell_z,
            preliminary_surface_level: HashMap::new(),
        }
    }

    pub fn preliminary_surface_level(&mut self, env: &mut JNIEnv, block_x: i32, block_z: i32) -> i32 {
        let block_x = block_x & !0b11;
        let block_z = block_z & !0b11;
        let key = (block_x, block_z);
        *self.preliminary_surface_level.entry(key).or_insert_with(|| {
            let (block_x, block_z) = key;
            // convert to long 
            let block_pos = (block_x as i64 & u32::MAX as i64) | ((block_z as i64 & u32::MAX as i64) << 32);
            // env.call_method(self.this.upgrade_local(env).unwrap().unwrap(), "computePreliminarySurfaceLevel", "(J)I", &[block_pos.into()]).unwrap().i().unwrap()
            let method_id = COMPUTE_PERLIMINARY_SURFACE_LEVEL_METHOD_ID.get_or_init(|| {
                env.get_method_id(
                    "net/minecraft/world/level/levelgen/NoiseChunk",
                    "computePreliminarySurfaceLevel",
                    "(J)I",
                ).unwrap()
            });
            unsafe {
                env.call_method_unchecked(
                    self.this.upgrade_local(env).unwrap().unwrap(),
                    *method_id,
                    ReturnType::Primitive(Primitive::Int),
                    &[JValue::from(block_pos).as_jni()],
                ).unwrap().i().unwrap()
            }
        })
    }
}

pub(crate) static COMPUTE_PERLIMINARY_SURFACE_LEVEL_METHOD_ID: OnceCell<JMethodID> = OnceCell::new();

impl FunctionContext for NoiseChunk {
    fn block_x(&self) -> i32 {
        self.cell_start_block_x + self.in_cell_x
    }

    fn block_y(&self) -> i32 {
        self.cell_start_block_y + self.in_cell_y
    }

    fn block_z(&self) -> i32 {
        self.cell_start_block_z + self.in_cell_z
    }
}

