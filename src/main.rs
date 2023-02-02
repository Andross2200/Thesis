use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

struct MainState {
    pos_x: f32,
    circle: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            100.0,
            2.0,
            Color::BLACK,
        )?;

        Ok(MainState { pos_x: 50.0, circle })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.5, 0.3, 1.0]));

        canvas.draw(&self.circle, Vec2::new(self.pos_x, 180.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}