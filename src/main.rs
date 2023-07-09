use ggez::{ event, GameError };
use ggez::graphics::{ self, Color, Drawable };
use ggez::{ Context, GameResult };
use ggez::glam::*;
use ggez::input::keyboard::KeyCode;

use rand::prelude::*;

const PADDLE_HEIGHT: f32 = 140.0;
const PADDLE_WIDTH: f32 = 25.0;
const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;
const PADDLE_SPEED: f32 = 450.0;
const PADDLE_RECT: graphics::Rect = graphics::Rect::new(
    -PADDLE_WIDTH_HALF, 
    -PADDLE_HEIGHT_HALF,
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
);

const BALL_SIZE: f32 = 16.0;
const BALL_SPEED: f32 = 10.0;
const PADDING: f32 = 10.0;


struct MainState {
    player_1_pos: Vec2,
    player_1_score: i32,
    player_2_pos: Vec2,
    player_2_score: i32,
    ball_pos: Vec2,
    ball_velocity: Vec2,
    loaded_custom_font: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        let mut rng = rand::thread_rng();
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let mut s = MainState { 
            player_1_pos: Vec2::new(20.0 + PADDING, screen_height * 0.5),
            player_2_pos: Vec2::new(screen_width - 20.0 - PADDING, screen_height * 0.5),
            ball_pos: Vec2::new(screen_width * 0.5, screen_height * 0.5),
            ball_velocity: Vec2::new(rng.gen(), rng.gen()),
            // ball_velocity: Vec2::new(-1.0, 0.0),
            player_1_score: 0,
            player_2_score: 0,
            loaded_custom_font: false
        };


        // Loading custom font
        if let Ok(custom_font) = graphics::FontData::from_path(ctx, "/ArcadeClassic.ttf") {
            ctx.gfx.add_font("Arcade Classic", custom_font);
            s.loaded_custom_font = true
        } 

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
    let screen_height = ctx.gfx.drawable_size().1;
    pos.x += vel.x * BALL_SPEED;
    pos.y += vel.y * BALL_SPEED;

    if pos.y <= BALL_SIZE || pos.y >= screen_height - BALL_SIZE {
        vel.y *= -1.0;
    }
}

fn reset_game(game_state: &mut MainState, ctx: &mut Context) {
    let (screen_width, screen_height) = ctx.gfx.drawable_size();
    game_state.ball_pos = Vec2::new(screen_width * 0.5, screen_height * 0.5);
}

fn check_collision(ball: Vec2, paddle: Vec2) -> bool {
    return ball.x < paddle.x + PADDLE_WIDTH
        && ball.x > paddle.x - PADDLE_WIDTH
        && ball.y < paddle.y + PADDLE_HEIGHT_HALF
        && ball.y > paddle.y - PADDLE_HEIGHT_HALF; 
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let screen_width = ctx.gfx.drawable_size().0;
        move_racket(ctx, KeyCode::W, &mut self.player_1_pos, -1.0);
        move_racket(ctx, KeyCode::S, &mut self.player_1_pos, 1.0);
        move_racket(ctx, KeyCode::Up, &mut self.player_2_pos, 1.0);
        move_racket(ctx, KeyCode::Down, &mut self.player_2_pos, -1.0);

        move_ball(ctx, &mut self.ball_pos, &mut self.ball_velocity);

        if self.ball_pos.x < 0.0 {
            self.player_2_score += 1;
            reset_game(self, ctx);
        }
        if self.ball_pos.x > screen_width {
            self.player_1_score += 1;
            reset_game(self, ctx);
        }

        if check_collision(self.ball_pos, self.player_1_pos) 
            || check_collision(self.ball_pos, self.player_2_pos) {
            self.ball_velocity.x *= -1.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
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

        let mut score_text = ggez::graphics::Text::new(format!("{}              {}", self.player_1_score, self.player_2_score));
        if self.loaded_custom_font {
            score_text.set_font("Arcade Classic");
        }
        score_text.set_scale(40.0);

        canvas.draw(
            &score_text,
            Vec2::new(
                screen_width * 0.5 - score_text.dimensions(ctx).unwrap_or_else(|| graphics::Rect::default()).w * 0.5, 
                screen_height * 0.1
            )
        );

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Shubham");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    ctx.gfx.set_window_title("Pong");
    event::run(ctx, event_loop, state)
}
