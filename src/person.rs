use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

#[derive(PartialEq)]
pub enum Status {
    Sick,
    Healthy,
}

pub struct Person {
    pos_x: f32,
    pos_y: f32,
    status: Status,
}

impl Person {
    pub fn new(status: Status) -> GameResult<Person> {
        let s = Person {
            pos_x: 0.0,
            pos_y: 0.0,
            status: Status::Sick,
        };
        Ok(s)
    }
}

impl event::EventHandler for Person {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        self.pos_y = self.pos_y % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let color = if Status::Sick == self.status {
            graphics::BLACK
        } else {
            graphics::WHITE
        };
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            20.0,
            2.0,
            color,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(self.pos_x, self.pos_y),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}
