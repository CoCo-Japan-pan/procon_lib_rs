//! From <https://ja.wikipedia.org/wiki/Permuted_congruential_generator>

#[derive(Debug, Clone, Copy)]
pub struct Pcg32 {
    state: u64,
}

impl Pcg32 {
    const MULTIPLIER: u64 = 6364136223846793005;
    const INCREMENT: u64 = 1442695040888963407;

    pub fn init(seed: u64) -> Self {
        Self {
            state: seed.wrapping_add(Self::INCREMENT),
        }
    }

    pub fn init_rand() -> Self {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Self::init(now.as_secs() + now.subsec_nanos() as u64)
    }

    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::INCREMENT);
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
}
