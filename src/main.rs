use ggez::{event, GameError};
use ggez::graphics::{self, Color};
use ggez::{ Context, GameResult };
use ggez::glam::*;
use ggez::input::keyboard::KeyCode;

const PADDLE_HEIGHT: f32 = 140.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
const PADDING: f32 = 10.0;
const PADDLE_SPEED: f32 = 350.0;

struct MainState {
    player_1_pos: Vec2,
    player_2_pos: Vec2,
}

impl MainState {
    fn new(ctx: &Context) -> GameResult<MainState> {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let s = MainState { 
            player_1_pos: Vec2::new(20.0 + PADDING, screen_height * 0.5), 
            player_2_pos: Vec2::new(screen_width - 20.0 - PADDING, screen_height * 0.5)
        };
        Ok(s)
    }
}

fn draw_racket (ctx: &Context) -> Result<graphics::Mesh, GameError> {
    let rect = graphics::Rect::new(
        -PADDLE_WIDTH_HALF, 
        -PADDLE_HEIGHT_HALF,
        PADDLE_WIDTH,
        PADDLE_HEIGHT,
    );
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        rect,
        Color::WHITE,
    )
}

fn move_racket (ctx: &Context, keycode: KeyCode, pos: &mut Vec2, dir: f32) {
    let dt = ctx.time.delta().as_secs_f32();
    if ctx.keyboard.is_key_pressed(keycode) {
        pos.y += PADDLE_SPEED * dir * dt
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        move_racket(ctx, KeyCode::W, &mut self.player_1_pos, -1.0);
        move_racket(ctx, KeyCode::S, &mut self.player_1_pos, 1.0);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, 
            Color::BLACK
        );

        let player_1_rect = draw_racket(&ctx)?;
        let player_2_rect = draw_racket(&ctx)?;

        canvas.draw(
            &player_1_rect,
            Vec2::new(self.player_1_pos.x, self.player_1_pos.y)
        );

        canvas.draw(
            &player_2_rect, 
            Vec2::new(self.player_2_pos.x, self.player_2_pos.y)
        );

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Shubham");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(&ctx)?;
    ctx.gfx.set_window_title("Pong");
    event::run(ctx, event_loop, state)
}
