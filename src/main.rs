use ggez::glam::Vec2;
use ggez::{event, graphics, Context, GameResult};

mod config;

use config::{POP_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH, TARGET_POS}; // Importowanie stałych

mod dna;
mod population;
mod rocket;

use population::Population;

struct GameState {
    population: Population,
    rocket_image: graphics::Image,
    target_image: graphics::Image,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let rocket_image = graphics::Image::from_path(ctx, "/rocket.png")?;
        let target_image = graphics::Image::from_path(ctx, "/target.png")?;
        Ok(Self {
            population: Population::new(),
            rocket_image,
            target_image,
        })
    }
}

// ✅ Poprawna implementacja EventHandler dla GameState (MUSI być poza `impl GameState`)
impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Aktualizacja populacji rakiet
        if self.population.all_done() {
            self.population.evaluate();
            self.population.evolve();
        } else {
            for rocket in &mut self.population.rockets {
                rocket.update();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // Rysowanie rakiet
        for rocket in &self.population.rockets {
            let dest = Vec2::new(rocket.position.0 as f32, rocket.position.1 as f32);

            // Oblicz kąt na podstawie wektora prędkości
            let angle = if rocket.velocity.0 == 0.0 && rocket.velocity.1 == 0.0 {
                0.0
            } else {
                rocket.velocity.1.atan2(rocket.velocity.0) + std::f64::consts::FRAC_PI_2
            } as f32;

            let draw_param = graphics::DrawParam::default()
                .dest(dest)
                .rotation(angle)
                .offset(Vec2::new(0.5, 0.5))
                .scale(Vec2::new(0.1, 0.1));

            canvas.draw(&self.rocket_image, draw_param);
        }

        // Rysowanie celu
        let target_pos = Vec2::new(TARGET_POS.0 as f32, TARGET_POS.1 as f32);
        let target_param = graphics::DrawParam::default()
            .dest(target_pos)
            .offset(Vec2::new(0.5, 0.5)) // ✅ Przesunięcie środka sprite'a na TARGET_POS
            .scale(Vec2::new(0.1, 0.1)); // Zmniejszenie obrazka celu

        canvas.draw(&self.target_image, target_param);

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("smart_rockets", "author")
        .add_resource_path("resources")
        .window_mode(ggez::conf::WindowMode {
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .build()
        .expect("Failed to create ggez context");

    let game = GameState::new(&mut ctx)?;

    event::run(ctx, event_loop, game)
}
