use rand::prelude::ThreadRng;
use rand::RngCore;

pub struct RandomHelper {
    random: ThreadRng,
    probabilities: Vec<u32>,
    sum: u32,
}

impl RandomHelper {
    pub fn new(probabilities: Vec<u32>) -> Self {
        assert_ne!(probabilities.len(), 0, "Probabilities list cannot be empty");
        Self {
            random: rand::thread_rng(),
            sum: probabilities.iter().sum(),
            probabilities,
        }
    }

    pub fn next_index(&mut self) -> usize {
        let random_val =  (self.random.next_u32() % self.sum) as usize;
        let mut random_val_counter: usize = 0;
        let mut i: usize = 0;
        for value in &self.probabilities {
            random_val_counter += *value as usize;
            if random_val_counter > random_val {
                return i;
            }
            i += 1;
        }
        return 0
    }
}