use ggez::glam::Vec2;
use ggez::{event, graphics, Context, GameResult};
use line_drawing::bresenham;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
mod config;
mod dna;
mod population;
mod rocket;

use config::{SCREEN_HEIGHT, SCREEN_WIDTH, TARGET_POS};
use population::Population;

struct GameState {
    population: Population,
    rocket_image: graphics::Image,
    target_image: graphics::Image,
    drawing_wall: bool,
    last_mouse_pos: Option<Vec2>,
    obstacles: HashSet<(i32, i32)>,
    obstacle_mesh: graphics::Mesh,
    need_mesh_update: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let rocket_image = graphics::Image::from_path(ctx, "/rocket.png")?;
        let target_image = graphics::Image::from_path(ctx, "/target.png")?;

        // Inicjalizacja pustego meshu dla przeszkód
        let obstacle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 5.0, 5.0),
            graphics::Color::new(0.5, 0.5, 0.5, 1.0),
        )?;

        Ok(Self {
            population: Population::new(),
            rocket_image,
            target_image,
            drawing_wall: false,
            last_mouse_pos: None,
            obstacles: HashSet::new(),
            obstacle_mesh,
            need_mesh_update: false,
        })
    }

    fn add_wall_segment(&mut self, x: f32, y: f32) {
        // Konwersja do współrzędnych siatki 5x5
        let grid_x = (x as i32 / 5) * 5;
        let grid_y = (y as i32 / 5) * 5;

        // Dodajemy całą komórkę 5x5
        if self.obstacles.insert((grid_x, grid_y)) {
            self.need_mesh_update = true;
        }
    }

    fn update_obstacle_mesh(&mut self, ctx: &mut Context) -> GameResult {
        let mut mesh_builder = graphics::MeshBuilder::new();

        for &(x, y) in &self.obstacles {
            mesh_builder.rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(x as f32, y as f32, 5.0, 5.0),
                graphics::Color::new(0.5, 0.5, 0.5, 1.0),
            )?;
        }

        // Metoda build() nie przyjmuje kontekstu i zwraca MeshData.
        let mesh_data = mesh_builder.build();
        // Konwertujemy MeshData na Mesh, przekazując kontekst.
        self.obstacle_mesh = graphics::Mesh::from_data(ctx, mesh_data);
        self.need_mesh_update = false;
        Ok(())
    }

    fn save_generation_data(&self) -> GameResult {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data.csv")?;

        let success_rate = self.population.calculate_success();
        writeln!(file, "{},{}", self.population.generation, success_rate)?;

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.population.all_done() {
            self.save_generation_data()?;
            self.population.evaluate();
            self.population.evolve();
        } else {
            self.population.update(&self.obstacles);
        }
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        if button == event::MouseButton::Left {
            self.drawing_wall = true;
            self.last_mouse_pos = Some(Vec2::new(x, y));
            self.add_wall_segment(x, y);
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), ggez::GameError> {
        if button == event::MouseButton::Left {
            self.drawing_wall = false;
            self.last_mouse_pos = None;
        }
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), ggez::GameError> {
        if self.drawing_wall {
            if let Some(last_pos) = self.last_mouse_pos {
                for (px, py) in bresenham(
                    (last_pos.x as isize, last_pos.y as isize),
                    (x as isize, y as isize),
                ) {
                    self.add_wall_segment(px as f32, py as f32);
                }
            }
            self.last_mouse_pos = Some(Vec2::new(x, y));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        if self.need_mesh_update {
            self.update_obstacle_mesh(ctx)?;
        }

        // Rysowanie rakiet
        for rocket in &self.population.rockets {
            let dest = Vec2::new(rocket.position.0 as f32, rocket.position.1 as f32);
            let angle = rocket.velocity.1.atan2(rocket.velocity.0) + std::f64::consts::FRAC_PI_2;

            canvas.draw(
                &self.rocket_image,
                graphics::DrawParam::default()
                    .dest(dest)
                    .rotation(angle as f32)
                    .offset(Vec2::new(0.5, 0.5))
                    .scale(Vec2::new(0.1, 0.1)),
            );
        }

        // Rysowanie celu
        canvas.draw(
            &self.target_image,
            graphics::DrawParam::default()
                .dest(Vec2::new(TARGET_POS.0 as f32, TARGET_POS.1 as f32))
                .offset(Vec2::new(0.5, 0.5))
                .scale(Vec2::new(0.1, 0.1)),
        );

        canvas.draw(&self.obstacle_mesh, graphics::DrawParam::default());
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
        .build()?;

    let game = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
