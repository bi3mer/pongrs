use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;
use macroquad::math::f32::Vec2;
use macroquad::window::{screen_width, screen_height};
use macroquad::shapes::{draw_rectangle, draw_circle};
use macroquad::color::WHITE;
use macroquad::text::draw_text;
use macroquad::input::{is_key_down, KeyCode};

pub struct Game {
    player_score: u8,
    ai_score: u8,
    player_paddle: Vec2,
    ai_paddle: Vec2,
    ball_pos: Vec2,
    ball_velocity: Vec2,
    paddle_dimensions: Vec2,
    ball_dimensions: Vec2,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player_score: 0,
            ai_score: 0,
            player_paddle: Vec2::new(0.05, 0.4),
            ai_paddle: Vec2::new(0.95, 0.4),
            ball_pos: Vec2::new(0.5, 0.5),
            ball_velocity: Vec2::new(0., 0.),
            paddle_dimensions: Vec2::new(0.008, 0.2),
            ball_dimensions: Vec2::new(0.02, 0.02)
        }
    }
}

impl Scene for Game {
    fn on_enter(&mut self) {
        // Nothing to do   
    }

    fn update(&mut self) -> SceneId {
        // player update
        if is_key_down(KeyCode::S) {
            self.player_paddle.y = f32::min(1. - self.paddle_dimensions.y, self.player_paddle.y + 0.01);
        }
        if is_key_down(KeyCode::W) {
            self.player_paddle.y = f32::max(0., self.player_paddle.y - 0.01);
        }

        // ball update

        // AI update
        if self.ball_pos.y > self.ai_paddle.y + self.paddle_dimensions.y/2. {
            self.ai_paddle.y = f32::min(1. - self.paddle_dimensions.y, self.ai_paddle.y + 0.01);
        } else if self.ball_pos.y < self.ai_paddle.y + self.paddle_dimensions.y/2. {
            self.ai_paddle.y = f32::max(0., self.ai_paddle.y - 0.01);
        }

        SceneId::Game
    }

    fn render(&mut self) {
        // draw the dotted line through the middle
        let h = screen_height();
        let w = screen_width();

        let rectangle_height = h/25.;
        for i in (1..25).step_by(2) {
            draw_rectangle(
                w/2.,
                rectangle_height * (i as f32),
                3.,
                rectangle_height,
                WHITE
            );
        }

        // render ball
        draw_circle(
            self.ball_pos.x * w, 
            self.ball_pos.y * h, 
            f32::max(self.ball_dimensions.x * w, self.ball_dimensions.y * h) / 2.,
            WHITE
        );

        // draw player paddle
        draw_rectangle(
            self.player_paddle.x * w,
            self.player_paddle.y * h,
            self.paddle_dimensions.x * w,
            self.paddle_dimensions.y * h,
            WHITE
        );

        // draw AI paddle
        draw_rectangle(
            self.ai_paddle.x * w,
            self.ai_paddle.y * h,
            self.paddle_dimensions.x * w,
            self.paddle_dimensions.y * h,
            WHITE
        );

        // draw player score
        draw_text(
            &self.player_score.to_string(), 
            0.25*w, 
            0.1*w, 
            0.05*w, 
            WHITE
        );

        // draw AI score
        draw_text(
            &self.ai_score.to_string(), 
            0.75*w, 
            0.1*w, 
            0.05*w, 
            WHITE
        );
    }

    fn on_exit(&mut self) {
        self.player_score = 0;
        self.ai_score = 0;
        self.player_paddle = Vec2::new(0.05, 0.4);
        self.ai_paddle = Vec2::new(0.95, 0.4);
        self.ball_pos = Vec2::new(0.5, 0.5);
        self.ball_velocity = Vec2::new(0., 0.);
        self.paddle_dimensions = Vec2::new(0.008, 0.2);
        self.ball_dimensions = Vec2::new(0.02, 0.02);
    }
}