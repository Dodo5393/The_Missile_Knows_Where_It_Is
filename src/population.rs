use crate::config::{LIFESPAN, MUTATION_RATE, POP_SIZE};
use crate::dna::DNA;
use crate::rocket::Rocket;
use rand::prelude::*;

pub struct Population {
    pub rockets: Vec<Rocket>,
    pub generation: u32,
    mating_pool: Vec<usize>,
}

impl Population {
    pub fn new() -> Self {
        let rockets = (0..POP_SIZE).map(|_| Rocket::new(DNA::new())).collect();
        Self {
            rockets,
            generation: 1,
            mating_pool: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        for rocket in &mut self.rockets {
            rocket.update();
        }
    }

    pub fn evaluate(&mut self) {
        // Oblicz maksymalne fitness
        let max_fitness = self
            .rockets
            .iter_mut()
            .map(|r| r.calculate_fitness())
            .fold(0.0_f64, |a, b| a.max(b));

        // Tworzenie puli rodzicielskiej z uwzględnieniem fitness
        self.mating_pool.clear();
        for (i, rocket) in self.rockets.iter().enumerate() {
            let fitness = if max_fitness > 0.0 {
                rocket.fitness / max_fitness
            } else {
                0.0
            };
            let n = (fitness * 100.0) as usize;
            self.mating_pool.extend(std::iter::repeat(i).take(n));
        }
    }

    pub fn evolve(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_rockets = Vec::with_capacity(POP_SIZE);

        for _ in 0..POP_SIZE {
            // Selekcja rodziców
            let parent_a = self.select_parent(&mut rng);
            let parent_b = self.select_parent(&mut rng);

            // Krzyżowanie
            let mut child_dna = parent_a.dna.crossover(&parent_b.dna);

            // Mutacja
            child_dna.mutate(MUTATION_RATE);

            new_rockets.push(Rocket::new(child_dna));
        }

        self.rockets = new_rockets;
        self.generation += 1;
    }

    fn select_parent<R: Rng>(&self, rng: &mut R) -> &Rocket {
        if self.mating_pool.is_empty() {
            // Losowy rodzic jeśli pula jest pusta
            return self.rockets.choose(rng).unwrap();
        }

        let parent_idx = *self.mating_pool.choose(rng).unwrap();
        &self.rockets[parent_idx]
    }

    pub fn all_done(&self) -> bool {
        self.rockets
            .iter()
            .all(|r| r.hit_target || r.crashed || r.step >= LIFESPAN)
    }
}
