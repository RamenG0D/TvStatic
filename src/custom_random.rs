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

    fn next(&mut self, bits: u32) -> u32 {
        let nextseed = (self.seed.wrapping_mul(self.multiplier) + self.addend) & self.mask;
        self.seed = nextseed;
        (nextseed >> (48 - bits)) as u32
    }

    pub fn prandom(&mut self) -> i32 {
        self.next(32) as i32
    }

    pub fn prandom_bound(&mut self, bound: i32) -> i32 {
        if bound <= 0 { panic!("bound must be positive"); }
        let r = self.next(31);
        let m = bound - 1;
        if (bound & m) == 0 {
            ((bound as u64 * r as u64) >> 31) as i32
        } else {
            let mut bits;
            let mut val;
            bits = self.next(31);
            val = bits as i32 % bound;
            while bits as i32 - val + m < 0 {
                bits = self.next(31);
                val = bits as i32 % bound;
            }
            val
        }
    }

    pub fn prandom_range(&mut self, min: i32, max: i32) -> i32 {
        if min >= max { panic!("min must be less than max"); }
        self.prandom_bound(max - min + 1) + min
    }

    pub fn random_color(&mut self) -> Color {
        Color::new(
            self.prandom_range(0, 255) as u8,
            self.prandom_range(0, 255) as u8,
            self.prandom_range(0, 255) as u8,
            self.prandom_range(0, 255) as u8,
        )
    }
}