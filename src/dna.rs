use crate::config;
use config::{FORCE_MAGNITUDE, LIFESPAN};
use rand::Rng;

#[derive(Clone)]
pub struct DNA {
    pub genes: Vec<(f64, f64)>,
}

impl DNA {
    pub fn new() -> Self {
        let mut genes = Vec::with_capacity(LIFESPAN);

        for _ in 0..LIFESPAN {
            genes.push((0.0, 0.0));
        }

        DNA { genes }
    }

    pub fn crossover(&self, partner: &DNA) -> DNA {
        let mut child_genes = Vec::with_capacity(LIFESPAN);
        let crossover_point = rand::thread_rng().gen_range(0..LIFESPAN);

        for i in 0..LIFESPAN {
            if i < crossover_point {
                child_genes.push(self.genes[i]);
            } else {
                child_genes.push(partner.genes[i]);
            }
        }

        DNA { genes: child_genes }
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::thread_rng();

        for gene in &mut self.genes {
            if rng.gen_bool(mutation_rate) {
                let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
                *gene = (angle.cos() * FORCE_MAGNITUDE, angle.sin() * FORCE_MAGNITUDE);
            }
        }
    }
}
