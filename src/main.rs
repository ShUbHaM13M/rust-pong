use ggez::{event, GameError};
use ggez::graphics::{self, Color};
use ggez::{ Context, GameResult };
use ggez::glam::*;
use ggez::input::keyboard::KeyCode;

use rand::prelude::*;

const PADDLE_HEIGHT: f32 = 140.0;
const PADDLE_WIDTH: f32 = 25.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
const PADDLE_SPEED: f32 = 350.0;
const PADDLE_RECT: graphics::Rect = graphics::Rect::new(
    -PADDLE_WIDTH_HALF, 
    -PADDLE_HEIGHT_HALF,
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
);

const BALL_SIZE: f32 = 20.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 10.0;

const PADDING: f32 = 10.0;


struct MainState {
    player_1_pos: Vec2,
    player_2_pos: Vec2,
    ball_pos: Vec2,
    ball_velocity: Vec2,
}

impl MainState {
    fn new(ctx: &Context) -> GameResult<MainState> {
        let mut rng = rand::thread_rng();
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let s = MainState { 
            player_1_pos: Vec2::new(20.0 + PADDING, screen_height * 0.5),
            player_2_pos: Vec2::new(screen_width - 20.0 - PADDING, screen_height * 0.5),
            ball_pos: Vec2::new(screen_width * 0.5, screen_height * 0.5),
            ball_velocity: Vec2::new(rng.gen(), rng.gen()),
        };
        Ok(s)
    }
}

fn draw_racket (ctx: &Context) -> Result<graphics::Mesh, GameError> {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        PADDLE_RECT,
        Color::WHITE,
    )
}

fn clamp (value: &mut f32, min: f32, max: f32) {
    if *value < min {
        *value = min;
    } else if *value > max {
        *value = max;
    }
}

fn move_racket (ctx: &Context, keycode: KeyCode, pos: &mut Vec2, dir: f32) {
    let screen_height = ctx.gfx.drawable_size().1;
    let dt = ctx.time.delta().as_secs_f32();
    if ctx.keyboard.is_key_pressed(keycode) {
        pos.y += PADDLE_SPEED * dir * dt
    }

    clamp(&mut pos.y, PADDLE_HEIGHT_HALF, screen_height - PADDLE_HEIGHT_HALF);
}

fn move_ball (ctx: &Context, pos: &mut Vec2, vel: &mut Vec2) {
    let (screen_width, screen_height) = ctx.gfx.drawable_size();
    pos.x += vel.x * BALL_SPEED;
    pos.y += vel.y * BALL_SPEED;

    if pos.x <= BALL_SIZE || pos.x >= screen_width - BALL_SIZE {
        vel.x *= -1.0;
    }

    if pos.y <= BALL_SIZE || pos.y >= screen_height - BALL_SIZE {
        vel.y *= -1.0;
    }

}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        move_racket(ctx, KeyCode::W, &mut self.player_1_pos, -1.0);
        move_racket(ctx, KeyCode::S, &mut self.player_1_pos, 1.0);
        move_racket(ctx, KeyCode::Up, &mut self.player_2_pos, 1.0);
        move_racket(ctx, KeyCode::Down, &mut self.player_2_pos, -1.0);

        move_ball(ctx, &mut self.ball_pos, &mut self.ball_velocity);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx, 
            Color::BLACK
        );

        let player_1_rect = draw_racket(&ctx)?;
        let player_2_rect = draw_racket(&ctx)?;

        let ball = graphics::Mesh::new_circle(
            ctx, 
            graphics::DrawMode::fill(), 
            Vec2::new(0.0, 0.0), 
            BALL_SIZE, 
            0.8, 
            Color::WHITE
        )?;

        canvas.draw(&player_1_rect, self.player_1_pos);
        canvas.draw(&player_2_rect, self.player_2_pos);

        canvas.draw(&ball, self.ball_pos);

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
