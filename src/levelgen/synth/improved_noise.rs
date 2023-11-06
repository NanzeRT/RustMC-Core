// public final class ImprovedNoise {
//     private static final float SHIFT_UP_EPSILON = 1.0E-7F;
//     private final byte[] p;
//     public final double xo;
//     public final double yo;
//     public final double zo;
//
//     public ImprovedNoise(RandomSource random) {
//         this.xo = random.nextDouble() * 256.0D;
//         this.yo = random.nextDouble() * 256.0D;
//         this.zo = random.nextDouble() * 256.0D;
//         this.p = new byte[256];
//
//         for(int i = 0; i < 256; ++i) {
//             this.p[i] = (byte)i;
//         }
//
//         for(int j = 0; j < 256; ++j) {
//             int k = random.nextInt(256 - j);
//             byte b = this.p[j];
//             this.p[j] = this.p[j + k];
//             this.p[j + k] = b;
//         }
//
//     }
//
//     public double noise(double x, double y, double z) {
//         return this.noise(x, y, z, 0.0D, 0.0D);
//     }
//
//     /** @deprecated */
//     @Deprecated
//     public double noise(double x, double y, double z, double yScale, double yMax) {
//         double d = x + this.xo;
//         double e = y + this.yo;
//         double f = z + this.zo;
//         int i = Mth.floor(d);
//         int j = Mth.floor(e);
//         int k = Mth.floor(f);
//         double g = d - (double)i;
//         double h = e - (double)j;
//         double l = f - (double)k;
//         double o;
//         if (yScale != 0.0D) {
//             double m;
//             if (yMax >= 0.0D && yMax < h) {
//                 m = yMax;
//             } else {
//                 m = h;
//             }
//
//             o = (double)Mth.floor(m / yScale + (double)1.0E-7F) * yScale;
//         } else {
//             o = 0.0D;
//         }
//
//         return this.sampleAndLerp(i, j, k, g, h - o, l, h);
//     }
//
//     public double noiseWithDerivative(double x, double y, double z, double[] ds) {
//         double d = x + this.xo;
//         double e = y + this.yo;
//         double f = z + this.zo;
//         int i = Mth.floor(d);
//         int j = Mth.floor(e);
//         int k = Mth.floor(f);
//         double g = d - (double)i;
//         double h = e - (double)j;
//         double l = f - (double)k;
//         return this.sampleWithDerivative(i, j, k, g, h, l, ds);
//     }
//
//     private static double gradDot(int hash, double x, double y, double z) {
//         return SimplexNoise.dot(SimplexNoise.GRADIENT[hash & 15], x, y, z);
//     }
//
//     private int p(int input) {
//         return this.p[input & 255] & 255;
//     }
//
//     private double sampleAndLerp(int sectionX, int sectionY, int sectionZ, double localX, double localY, double localZ, double fadeLocalY) {
//         int i = this.p(sectionX);
//         int j = this.p(sectionX + 1);
//         int k = this.p(i + sectionY);
//         int l = this.p(i + sectionY + 1);
//         int m = this.p(j + sectionY);
//         int n = this.p(j + sectionY + 1);
//         double d = gradDot(this.p(k + sectionZ), localX, localY, localZ);
//         double e = gradDot(this.p(m + sectionZ), localX - 1.0D, localY, localZ);
//         double f = gradDot(this.p(l + sectionZ), localX, localY - 1.0D, localZ);
//         double g = gradDot(this.p(n + sectionZ), localX - 1.0D, localY - 1.0D, localZ);
//         double h = gradDot(this.p(k + sectionZ + 1), localX, localY, localZ - 1.0D);
//         double o = gradDot(this.p(m + sectionZ + 1), localX - 1.0D, localY, localZ - 1.0D);
//         double p = gradDot(this.p(l + sectionZ + 1), localX, localY - 1.0D, localZ - 1.0D);
//         double q = gradDot(this.p(n + sectionZ + 1), localX - 1.0D, localY - 1.0D, localZ - 1.0D);
//         double r = Mth.smoothstep(localX);
//         double s = Mth.smoothstep(fadeLocalY);
//         double t = Mth.smoothstep(localZ);
//         return Mth.lerp3(r, s, t, d, e, f, g, h, o, p, q);
//     }
//
//     private double sampleWithDerivative(int sectionX, int sectionY, int sectionZ, double localX, double localY, double localZ, double[] ds) {
//         int i = this.p(sectionX);
//         int j = this.p(sectionX + 1);
//         int k = this.p(i + sectionY);
//         int l = this.p(i + sectionY + 1);
//         int m = this.p(j + sectionY);
//         int n = this.p(j + sectionY + 1);
//         int o = this.p(k + sectionZ);
//         int p = this.p(m + sectionZ);
//         int q = this.p(l + sectionZ);
//         int r = this.p(n + sectionZ);
//         int s = this.p(k + sectionZ + 1);
//         int t = this.p(m + sectionZ + 1);
//         int u = this.p(l + sectionZ + 1);
//         int v = this.p(n + sectionZ + 1);
//         int[] is = SimplexNoise.GRADIENT[o & 15];
//         int[] js = SimplexNoise.GRADIENT[p & 15];
//         int[] ks = SimplexNoise.GRADIENT[q & 15];
//         int[] ls = SimplexNoise.GRADIENT[r & 15];
//         int[] ms = SimplexNoise.GRADIENT[s & 15];
//         int[] ns = SimplexNoise.GRADIENT[t & 15];
//         int[] os = SimplexNoise.GRADIENT[u & 15];
//         int[] ps = SimplexNoise.GRADIENT[v & 15];
//         double d = SimplexNoise.dot(is, localX, localY, localZ);
//         double e = SimplexNoise.dot(js, localX - 1.0D, localY, localZ);
//         double f = SimplexNoise.dot(ks, localX, localY - 1.0D, localZ);
//         double g = SimplexNoise.dot(ls, localX - 1.0D, localY - 1.0D, localZ);
//         double h = SimplexNoise.dot(ms, localX, localY, localZ - 1.0D);
//         double w = SimplexNoise.dot(ns, localX - 1.0D, localY, localZ - 1.0D);
//         double x = SimplexNoise.dot(os, localX, localY - 1.0D, localZ - 1.0D);
//         double y = SimplexNoise.dot(ps, localX - 1.0D, localY - 1.0D, localZ - 1.0D);
//         double z = Mth.smoothstep(localX);
//         double aa = Mth.smoothstep(localY);
//         double ab = Mth.smoothstep(localZ);
//         double ac = Mth.lerp3(z, aa, ab, (double)is[0], (double)js[0], (double)ks[0], (double)ls[0], (double)ms[0], (double)ns[0], (double)os[0], (double)ps[0]);
//         double ad = Mth.lerp3(z, aa, ab, (double)is[1], (double)js[1], (double)ks[1], (double)ls[1], (double)ms[1], (double)ns[1], (double)os[1], (double)ps[1]);
//         double ae = Mth.lerp3(z, aa, ab, (double)is[2], (double)js[2], (double)ks[2], (double)ls[2], (double)ms[2], (double)ns[2], (double)os[2], (double)ps[2]);
//         double af = Mth.lerp2(aa, ab, e - d, g - f, w - h, y - x);
//         double ag = Mth.lerp2(ab, z, f - d, x - h, g - e, y - w);
//         double ah = Mth.lerp2(z, aa, h - d, w - e, x - f, y - g);
//         double ai = Mth.smoothstepDerivative(localX);
//         double aj = Mth.smoothstepDerivative(localY);
//         double ak = Mth.smoothstepDerivative(localZ);
//         double al = ac + ai * af;
//         double am = ad + aj * ag;
//         double an = ae + ak * ah;
//         ds[0] += al;
//         ds[1] += am;
//         ds[2] += an;
//         return Mth.lerp3(z, aa, ab, d, e, f, g, h, w, x, y);
//     }
//
//     @VisibleForTesting
//     public void parityConfigString(StringBuilder info) {
//         NoiseUtils.parityNoiseOctaveConfigString(info, this.xo, this.yo, this.zo, this.p);
//     }
// }

use crate::levelgen::random::random_source::RandomSource;

use super::simplex_noise;

#[derive(Debug)]
pub struct ImprovedNoise {
    p: [u8; 256],
    pub(super) xo: f64,
    pub(super) yo: f64,
    pub(super) zo: f64,
}

impl ImprovedNoise {
    pub fn new(random: &mut impl RandomSource) -> Self {
        let mut p = [0u8; 256];
        for (i, x) in p.iter_mut().enumerate() {
            *x = i as u8;
        }
        for i in 0..256 {
            let k = random.next_int_bound(256 - i as i32) as usize;
            p.swap(i, i + k);
        }
        Self {
            p,
            xo: random.next_double() * 256.0,
            yo: random.next_double() * 256.0,
            zo: random.next_double() * 256.0,
        }
    }

    pub fn new_raw(p: [u8; 256], xo: f64, yo: f64, zo: f64) -> Self {
        Self { p, xo, yo, zo }
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        self.noise_scaled(x, y, z, 0.0, 0.0)
    }

    pub fn noise_scaled(&self, x: f64, y: f64, z: f64, y_scale: f64, y_max: f64) -> f64 {
        let d = x + self.xo;
        let e = y + self.yo;
        let f = z + self.zo;
        let i = d.floor() as i32;
        let j = e.floor() as i32;
        let k = f.floor() as i32;
        let g = d - i as f64;
        let h = e - j as f64;
        let l = f - k as f64;
        let o = if y_scale != 0.0 {
            let m = if y_max >= 0.0 && y_max < h { y_max } else { h };
            (m / y_scale + SHIFT_UP_EPSILON).floor() * y_scale
        } else {
            0.0
        };
        self.sample_and_lerp(i, j, k, g, h - o, l, h)
    }

    pub fn noise_with_derivative(&self, x: f64, y: f64, z: f64, ds: &mut [f64; 3]) -> f64 {
        let d = x + self.xo;
        let e = y + self.yo;
        let f = z + self.zo;
        let i = d.floor() as i32;
        let j = e.floor() as i32;
        let k = f.floor() as i32;
        let g = d - i as f64;
        let h = e - j as f64;
        let l = f - k as f64;
        self.sample_with_derivative(i, j, k, g, h, l, ds)
    }

    fn p(&self, input: i32) -> u8 {
        self.p[(input & 255) as usize]
    }

    fn sample_and_lerp(
        &self,
        section_x: i32,
        section_y: i32,
        section_z: i32,
        local_x: f64,
        local_y: f64,
        local_z: f64,
        fade_local_y: f64,
    ) -> f64 {
        let i = self.p(section_x) as i32;
        let j = self.p(section_x + 1) as i32;
        let k = self.p(i + section_y) as i32;
        let l = self.p(i + section_y + 1) as i32;
        let m = self.p(j + section_y) as i32;
        let n = self.p(j + section_y + 1) as i32;
        let d = grad_dot(self.p(k + section_z) as i32, local_x, local_y, local_z);
        let e = grad_dot(
            self.p(m + section_z) as i32,
            local_x - 1.0,
            local_y,
            local_z,
        );
        let f = grad_dot(
            self.p(l + section_z) as i32,
            local_x,
            local_y - 1.0,
            local_z,
        );
        let g = grad_dot(
            self.p(n + section_z) as i32,
            local_x - 1.0,
            local_y - 1.0,
            local_z,
        );
        let h = grad_dot(
            self.p(k + section_z + 1) as i32,
            local_x,
            local_y,
            local_z - 1.0,
        );
        let o = grad_dot(
            self.p(m + section_z + 1) as i32,
            local_x - 1.0,
            local_y,
            local_z - 1.0,
        );
        let p = grad_dot(
            self.p(l + section_z + 1) as i32,
            local_x,
            local_y - 1.0,
            local_z - 1.0,
        );
        let q = grad_dot(
            self.p(n + section_z + 1) as i32,
            local_x - 1.0,
            local_y - 1.0,
            local_z - 1.0,
        );
        let r = smoothstep(local_x);
        let s = smoothstep(fade_local_y);
        let t = smoothstep(local_z);
        lerp3(r, s, t, d, e, f, g, h, o, p, q)
    }

    fn sample_with_derivative(
        &self,
        section_x: i32,
        section_y: i32,
        section_z: i32,
        local_x: f64,
        local_y: f64,
        local_z: f64,
        ds: &mut [f64; 3],
    ) -> f64 {
        let i = self.p(section_x) as i32;
        let j = self.p(section_x + 1) as i32;
        let k = self.p(i + section_y) as i32;
        let l = self.p(i + section_y + 1) as i32;
        let m = self.p(j + section_y) as i32;
        let n = self.p(j + section_y + 1) as i32;
        let o = self.p(k + section_z) as i32;
        let p = self.p(m + section_z) as i32;
        let q = self.p(l + section_z) as i32;
        let r = self.p(n + section_z) as i32;
        let s = self.p(k + section_z + 1) as i32;
        let t = self.p(m + section_z + 1) as i32;
        let u = self.p(l + section_z + 1) as i32;
        let v = self.p(n + section_z + 1) as i32;
        let is = simplex_noise::GRADIENT[(o & 15) as usize];
        let js = simplex_noise::GRADIENT[(p & 15) as usize];
        let ks = simplex_noise::GRADIENT[(q & 15) as usize];
        let ls = simplex_noise::GRADIENT[(r & 15) as usize];
        let ms = simplex_noise::GRADIENT[(s & 15) as usize];
        let ns = simplex_noise::GRADIENT[(t & 15) as usize];
        let os = simplex_noise::GRADIENT[(u & 15) as usize];
        let ps = simplex_noise::GRADIENT[(v & 15) as usize];
        let d = simplex_noise::dot(is, local_x, local_y, local_z);
        let e = simplex_noise::dot(js, local_x - 1.0, local_y, local_z);
        let f = simplex_noise::dot(ks, local_x, local_y - 1.0, local_z);
        let g = simplex_noise::dot(ls, local_x - 1.0, local_y - 1.0, local_z);
        let h = simplex_noise::dot(ms, local_x, local_y, local_z - 1.0);
        let w = simplex_noise::dot(ns, local_x - 1.0, local_y, local_z - 1.0);
        let x = simplex_noise::dot(os, local_x, local_y - 1.0, local_z - 1.0);
        let y = simplex_noise::dot(ps, local_x - 1.0, local_y - 1.0, local_z - 1.0);
        let z = smoothstep(local_x);
        let aa = smoothstep(local_y);
        let ab = smoothstep(local_z);
        let ac = lerp3(
            z,
            aa,
            ab,
            is[0] as f64,
            js[0] as f64,
            ks[0] as f64,
            ls[0] as f64,
            ms[0] as f64,
            ns[0] as f64,
            os[0] as f64,
            ps[0] as f64,
        );
        let ad = lerp3(
            z,
            aa,
            ab,
            is[1] as f64,
            js[1] as f64,
            ks[1] as f64,
            ls[1] as f64,
            ms[1] as f64,
            ns[1] as f64,
            os[1] as f64,
            ps[1] as f64,
        );
        let ae = lerp3(
            z,
            aa,
            ab,
            is[2] as f64,
            js[2] as f64,
            ks[2] as f64,
            ls[2] as f64,
            ms[2] as f64,
            ns[2] as f64,
            os[2] as f64,
            ps[2] as f64,
        );
        let af = lerp2(aa, ab, e - d, g - f, w - h, y - x);
        let ag = lerp2(ab, z, f - d, x - h, g - e, y - w);
        let ah = lerp2(z, aa, h - d, w - e, x - f, y - g);
        let ai = smoothstep_derivative(local_x);
        let aj = smoothstep_derivative(local_y);
        let ak = smoothstep_derivative(local_z);
        let al = ac + ai * af;
        let am = ad + aj * ag;
        let an = ae + ak * ah;
        ds[0] += al;
        ds[1] += am;
        ds[2] += an;
        lerp3(z, aa, ab, d, e, f, g, h, w, x, y)
    }
}

const SHIFT_UP_EPSILON: f64 = 1.0E-7;

fn grad_dot(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    simplex_noise::dot(simplex_noise::GRADIENT[(hash & 15) as usize], x, y, z)
}

fn smoothstep(x: f64) -> f64 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

fn smoothstep_derivative(x: f64) -> f64 {
    30.0 * x * x * (x - 1.0) * (x - 1.0)
}

fn lerp3(
    delta_x: f64,
    delta_y: f64,
    delta_z: f64,
    x0y0z0: f64,
    x1y0z0: f64,
    x0y1z0: f64,
    x1y1z0: f64,
    x0y0z1: f64,
    x1y0z1: f64,
    x0y1z1: f64,
    x1y1z1: f64,
) -> f64 {
    lerp(
        delta_z,
        lerp2(delta_x, delta_y, x0y0z0, x1y0z0, x0y1z0, x1y1z0),
        lerp2(delta_x, delta_y, x0y0z1, x1y0z1, x0y1z1, x1y1z1),
    )
}

fn lerp(delta: f64, a: f64, b: f64) -> f64 {
    a + delta * (b - a)
}

fn lerp2(delta_x: f64, delta_y: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    lerp(delta_y, lerp(delta_x, a, b), lerp(delta_x, c, d))
}
