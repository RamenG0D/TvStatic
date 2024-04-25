use std::time::SystemTime;

use raylib::color::Color;
pub struct Random {
    seed: u64,
    multiplier: u64,
    addend: u64,
    mask: u64,
}
impl Random {
    const MULTIPLIER: u64 = 0x5DEECE66D;
    const ADDEND: u64 = 0xB;
    const MASK: u64 = (1 << 48) - 1;

    pub fn new() -> Self {
        let seed = Self::seed_uniquifier() ^ SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64;
        Self {
            seed: Self::initial_scramble(seed),
            multiplier: Self::MULTIPLIER,
            addend: Self::ADDEND,
            mask: Self::MASK,
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.seed = Self::initial_scramble(seed);
    }

    pub fn new_seed(&mut self) {
        self.seed = Self::seed_uniquifier() ^ SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64;
    }

    fn initial_scramble(seed: u64) -> u64 {
        (seed ^ Self::MULTIPLIER) & Self::MASK
    }

    fn seed_uniquifier() -> u64 {
        static mut SEED_UNIQUIFIER: u64 = 8682522807148012;
        let current = unsafe { SEED_UNIQUIFIER };
        let next = current.wrapping_mul(1181783497276652981);
        unsafe {
            SEED_UNIQUIFIER = next;
        }
        next
    }

    fn next(&mut self, bits: usize) -> usize {
        let nextseed = (self.seed.wrapping_mul(self.multiplier) + self.addend) & self.mask;
        self.seed = nextseed;
        (nextseed >> (48 - bits)) as usize
    }

    fn next_float(&mut self) -> f64 {
        self.next(26) as f64 / (1 << 26) as f64
    }

    /// Returns a random integer from -isize::MAX to isize::MAX
    pub fn prandom(&mut self) -> isize {
        self.next(32) as isize
    }

    pub fn prandom_bound(&mut self, bound: isize) -> isize {
        if bound <= 0 { panic!("bound must be positive"); }
        let r = self.next(31);
        let m = bound - 1;
        if (bound & m) == 0 {
            ((bound as u64 * r as u64) >> 31) as isize
        } else {
            let mut bits;
            let mut val;
            bits = self.next(31);
            val = bits as isize % bound;
            while bits as isize - val + m < 0 {
                bits = self.next(31);
                val = bits as isize % bound;
            }
            val
        }
    }

    pub fn prandom_bound_float(&mut self, bound: f64) -> f64 {
        if bound <= 0.0 { panic!("bound must be positive"); }
        let r = self.next_float();
        let m = bound - 1.0;
        if (bound as isize & m as isize) == 0 {
            bound * r
        } else {
            let mut bits;
            let mut val;
            bits = self.next_float();
            val = bits % bound;
            while bits - val + m < 0.0 {
                bits = self.next_float();
                val = bits % bound;
            }
            val
        }
    }

    pub fn random_range_float(&mut self, range: std::ops::Range<f64>) -> f64 {
        if range.start >= range.end { panic!("min must be less than max"); }
        self.prandom_bound_float(range.end - range.start) + range.start
    }

    pub fn random_range(&mut self, range: std::ops::Range<isize>) -> isize {
        if range.start >= range.end { panic!("min must be less than max"); }
        self.prandom_bound(range.end - range.start + 1) + range.start
    }

    pub fn random_color(&mut self) -> Color {
        Color::new(
            self.random_range(0..255) as u8,
            self.random_range(0..255) as u8,
            self.random_range(0..255) as u8,
            self.random_range(0..255) as u8,
        )
    }
}