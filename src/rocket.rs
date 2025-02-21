use crate::config;
use crate::dna::DNA;
use config::{LIFESPAN, SCREEN_HEIGHT, SCREEN_WIDTH, START_POS, TARGET_POS};
use std::collections::HashSet;

const MAX_VELOCITY: f64 = 60.0;

pub struct Rocket {
    pub min_distance: f64,
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
            min_distance: std::f64::MAX,
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
        // Obliczamy bazowy fitness na podstawie minimalnej odległości do celu
        let mut fitness = 1.0 / (self.min_distance + 1.0);

        // Dodatkowo możemy uwzględnić, jak szybko rakieta osiągnęła minimalną odległość
        // Przyjmijmy, że wcześniejsze osiągnięcie lepszego min_distance daje wyższy wynik
        // Można na przykład użyć współczynnika malejącego wraz z liczbą kroków:
        let time_factor = 1.0 / ((self.step as f64 / LIFESPAN as f64) + 1.0);
        fitness *= time_factor;

        // W przypadku trafienia celu premiujemy bardzo mocno – im szybciej, tym lepiej
        if self.hit_target {
            fitness = 1_000.0 / (self.step as f64 + 1.0);
        }
        // Jeśli rakieta się rozbiła, nadal chcemy uwzględnić, jak blisko była celu,
        // ale z nałożoną karą
        else if self.crashed {
            fitness *= 0.5;
        }

        self.fitness = fitness;
        self.fitness
    }

    pub fn update(&mut self, obstacles: &HashSet<(i32, i32)>) {
        //checking if rocket hits target
        let dx = self.position.0 - config::TARGET_POS.0;
        let dy = self.position.1 - config::TARGET_POS.1;
        let distance = dx.powi(2) + dy.powi(2);
        if distance < config::TARGET_RADIUS.powi(2) {
            self.hit_target = true;
            self.position = config::TARGET_POS;
        }
        if distance < self.min_distance {
            self.min_distance = distance;
        }

        //check screen bounds
        if self.position.0 < 0.0
            || self.position.1 < 0.0
            || self.position.0 > config::SCREEN_WIDTH as f64
            || self.position.1 > config::SCREEN_HEIGHT as f64
        {
            self.crashed = true;
        }

        // Sprawdź kolizje z optymalizacją przestrzenną
        let grid_x = (self.position.0 as i32 / 5) * 5;
        let grid_y = (self.position.1 as i32 / 5) * 5;

        if obstacles.contains(&(grid_x, grid_y)) {
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
