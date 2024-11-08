use rand;
use crate::board::Board;

impl RandomBoardGen<rngs::StdRng> {
    pub fn from_seed(seed: u64) -> Self {
        RandomBoardGen {
            rng: rand::SeedableRng::seed_from_u64(seed),
        }
    }
}

impl Default for RandomBoardGen<rngs::ThreadRng> {
    fn default() -> Self {
        RandomBoardGen {
            rng: rand::thread_rng(),
        }
    }
}