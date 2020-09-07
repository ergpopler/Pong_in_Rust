use ggez;
use ggez::{Context, GameResult, graphics, event};
use ggez::nalgebra as na;
use ggez::input::keyboard::{self, KeyCode};
use rand::{self, thread_rng, Rng};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 25.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 600.0;
const BALL_SPEED: f32 = 300.0;
const PADDING: f32 = 40.0;

fn clamp(value: &mut f32, low: f32, high: f32){
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    } 
}

fn rand_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32){
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5){
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5){
        true => y,
        false => -y,
    };
}


struct MainState {
    p1_pos: na::Point2<f32>,
    p2_pos: na::Point2<f32>,
    b_pos: na::Point2<f32>,
    b_vel: na::Vector2<f32>,
    p1_points: i32,
    p2_points: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {

        
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        let mut b_vel =  na::Vector2::new(0.0, 0.0);
        rand_vec(&mut b_vel, BALL_SPEED, BALL_SPEED);
        MainState {
            p1_pos: na::Point2::new(RACKET_WIDTH_HALF + PADDING, screen_h_half),
            p2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF - PADDING , screen_h_half),
            b_pos: na::Point2::new(screen_w_half, screen_h_half),
            p1_points: 0,
            p2_points: 0,
            b_vel: b_vel,

        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut  Context) -> GameResult {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let s_h = graphics::drawable_size(ctx).1;
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.p1_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::S){
            self.p1_pos.y += PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            self.p2_pos.y -= PLAYER_SPEED * dt;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down){
            self.p2_pos.y += PLAYER_SPEED * dt;
        }

        self.b_pos += self.b_vel * dt;


        clamp(&mut self.p1_pos.y, RACKET_HEIGHT_HALF, s_h - RACKET_HEIGHT_HALF);
        clamp(&mut self.p2_pos.y, RACKET_HEIGHT_HALF, s_h - RACKET_HEIGHT_HALF);

        if self.b_pos.x < 0.0 {
            self.b_pos.x = screen_w * 0.5;
            self.b_pos.y = screen_h * 0.5;
            rand_vec(&mut self.b_vel, BALL_SPEED, BALL_SPEED);
            self.p2_points += 1;
        }
        if self.b_pos.x > screen_w {
            self.b_pos.x = screen_w * 0.5;
            self.b_pos.y = screen_h * 0.5;
            rand_vec(&mut self.b_vel, BALL_SPEED, BALL_SPEED);
            self.p1_points += 1;
        }

        if self.b_pos.y < BALL_SIZE_HALF{
            self.b_pos.y = BALL_SIZE_HALF;
            self.b_vel.y = self.b_vel.y.abs();
        }
        else if self.b_pos.y > screen_h - BALL_SIZE_HALF {
            self.b_pos.y = screen_h - BALL_SIZE_HALF;
            self.b_vel.y = -self.b_vel.y.abs() ;
        }

        let intersects_p1 = self.b_pos.x - BALL_SIZE_HALF
            < self.p1_pos.x + RACKET_WIDTH_HALF
            && self.b_pos.x + BALL_SIZE_HALF > self.p1_pos.x - RACKET_WIDTH_HALF
            && self.b_pos.y - BALL_SIZE_HALF < self.p1_pos.y + RACKET_HEIGHT_HALF
            && self.b_pos.y + BALL_SIZE_HALF > self.p1_pos.y - RACKET_HEIGHT_HALF;

        let intersects_p2 = self.b_pos.x - BALL_SIZE_HALF
            < self.p2_pos.x + RACKET_WIDTH_HALF
            && self.b_pos.x + BALL_SIZE_HALF > self.p2_pos.x - RACKET_WIDTH_HALF
            && self.b_pos.y - BALL_SIZE_HALF < self.p2_pos.y + RACKET_HEIGHT_HALF
            && self.b_pos.y + BALL_SIZE_HALF > self.p2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_p1{
            self.b_vel.x = self.b_vel.x.abs();
        }
        if intersects_p2{
            self.b_vel.x = -self.b_vel.x.abs();
        }


        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let (screen_w, screen_h) = graphics::drawable_size(ctx);

        let racket_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),racket_rect, graphics::WHITE)?;

        let ball_rect = graphics::Rect::new(-RACKET_WIDTH_HALF, -RACKET_HEIGHT_HALF, RACKET_WIDTH, RACKET_HEIGHT);
        let racket_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),ball_rect, graphics::WHITE)?;

        let mut draw_param =graphics::DrawParam::default();
        draw_param.dest = self.p1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.p2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.b_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text = ggez::graphics::Text::new(format!("{}                {}", self.p1_points, self.p2_points));
        let score_pos = na::Point2::new(screen_w * 0.45, screen_h * 0.10);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}
fn main() -> GameResult{
    let cb = ggez::ContextBuilder::new("Pong", "Ergpopler");
    let (ctx, event_loop) = &mut cb.build()?;

    graphics::set_window_title(ctx, "Pong");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state);

    Ok(())
}
