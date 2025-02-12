use crate::config;
use crate::dna::DNA;
use config::{LIFESPAN, START_POS, TARGET_POS};

const MAX_VELOCITY: f64 = 60.0;

pub struct Rocket {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    acceleration: (f64, f64),
    pub dna: DNA,
    pub fitness: f64,
    pub hit_target: bool,
    pub crashed: bool,
    pub step: usize,
}

impl Rocket {
    pub fn new(dna: DNA) -> Self {
        Self {
            position: START_POS,
            velocity: (0.0, 0.0),
            acceleration: (0.0, -0.1),
            dna,
            fitness: 0.0,
            hit_target: false,
            crashed: false,
            step: 0,
        }
    }
    fn apply_Force(&mut self, force: (f64, f64)) {
        self.acceleration.0 += force.0 * 2.0;
        self.acceleration.1 += force.1 * 2.0;
    }
    pub fn calculate_fitness(&mut self) -> f64 {
        let dx = TARGET_POS.0 - self.position.0;
        let dy = TARGET_POS.1 - self.position.1;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        if self.hit_target {
            // Jeśli rakieta trafiła w cel, dostaje bardzo wysoki wynik
            self.fitness = 1_000.0 / (self.step as f64 + 1.0); // Więcej punktów za szybsze dotarcie
        } else if self.crashed {
            // Jeśli rakieta się rozbiła, dostaje bardzo niski wynik
            self.fitness = 0.1;
        } else {
            // Dla rakiet, które nie trafiły w cel – odwrotność odległości
            self.fitness = 1.0 / (distance + 1.0);
        }

        self.fitness
    }

    pub fn update(&mut self) {
        //checking if rocket hits target
        let dx = self.position.0 - config::TARGET_POS.0;
        let dy = self.position.1 - config::TARGET_POS.1;
        let distance = dx.powi(2) + dy.powi(2);
        if distance < config::TARGET_RADIUS.powi(2) {
            self.hit_target = true;
            self.position = config::TARGET_POS;
        }

        //check screen bounds
        if self.position.0 < 0.0
            || self.position.1 < 0.0
            || self.position.0 > config::SCREEN_WIDTH as f64
            || self.position.1 > config::SCREEN_HEIGHT as f64
        {
            self.crashed = true;
        }

        //Applying DNA
        if self.step < LIFESPAN {
            let force = self.dna.genes[self.step];
            self.apply_Force(force);
            self.step += 1;
        }
        // Applying velocity
        if !self.hit_target && !self.crashed {
            self.velocity.0 += self.acceleration.0;
            self.velocity.1 += self.acceleration.1;
            //limit velocity
            let speed = self.velocity.0.powi(2) + self.velocity.1.powi(2);
            if speed > MAX_VELOCITY.powi(2) {
                let ratio = speed / MAX_VELOCITY;
                self.velocity.0 *= ratio;
                self.velocity.1 *= ratio;
            }
            //update position
            self.position.0 += self.velocity.0;
            self.position.1 += self.velocity.1;

            //resetting acceleration
            self.acceleration = (0.0, 0.0);
        }
    }
}
