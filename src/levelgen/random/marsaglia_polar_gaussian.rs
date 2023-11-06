use super::random_source::RandomCore;

#[derive(Copy, Clone, Default)]
pub struct MarsagliaPolarGaussian {
    next_next_gaussian: f64,
    have_next_next_gaussian: bool,
}

impl MarsagliaPolarGaussian {
    pub fn new() -> Self {
        Self {
            have_next_next_gaussian: false,
            next_next_gaussian: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.have_next_next_gaussian = false;
    }

    pub fn next_gaussian(&mut self, random_source: &mut impl RandomCore) -> f64 {
        if self.have_next_next_gaussian {
            self.have_next_next_gaussian = false;
            self.next_next_gaussian
        } else {
            let mut d;
            let mut e;
            let mut f;
            loop {
                d = 2.0 * random_source.next_double() - 1.0;
                e = 2.0 * random_source.next_double() - 1.0;
                f = d * d + e * e;
                if f < 1.0 && f != 0.0 {
                    break;
                }
            }
            let g = (f * -2.0 * f.ln()).sqrt() / f;
            self.next_next_gaussian = e * g;
            self.have_next_next_gaussian = true;
            d * g
        }
    }
}
