use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;
use macroquad::math::f32::Vec2;
use macroquad::window::{screen_width, screen_height};
use macroquad::shapes::draw_rectangle;
use macroquad::color::WHITE;
use macroquad::text::draw_text;

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
        let w = screen_width();
        let h = screen_height();

        Game {
            player_score: 0,
            ai_score: 0,
            player_paddle: Vec2::new(30., h/2.),
            ai_paddle: Vec2::new(w-30., h/2.),
            ball_pos: Vec2::new(w/2., h/2.),
            ball_velocity: Vec2::new(0., 0.),
            paddle_dimensions: Vec2::new(10., 100.),
            ball_dimensions: Vec2::new(10., 10.)
        }
    }
}

impl Scene for Game {
    fn on_enter(&mut self) {
        // Nothing to do   
    }

    fn update(&mut self) -> SceneId {
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
        draw_rectangle(
            self.ball_pos.x, 
            self.ball_pos.y, 
            self.ball_dimensions.x,
            self.ball_dimensions.y,
            WHITE
    );

        // draw player paddle
        draw_rectangle(
            self.player_paddle.x,
            self.player_paddle.y,
            self.paddle_dimensions.x,
            self.paddle_dimensions.y,
            WHITE
        );

        // draw AI paddle
        draw_rectangle(
            self.ai_paddle.x,
            self.ai_paddle.y,
            self.paddle_dimensions.x,
            self.paddle_dimensions.y,
            WHITE
        );

        // draw player score
        let mid_point = w / 2.;
        draw_text(
            &self.player_score.to_string(), 
            mid_point - mid_point/2., 
            100., 
            60., 
            WHITE
        );

        // draw AI score
        draw_text(
            &self.ai_score.to_string(), 
            mid_point + mid_point/2., 
            100., 
            60., 
            WHITE
        );
    }

    fn on_exit(&mut self) {
        // reset
        let w = screen_width();
        let h = screen_height();

        self.player_score = 0;
        self.ai_score = 0;
        self.player_paddle = Vec2::new(10., h/2.);
        self.ai_paddle = Vec2::new(w-10., h/2.);
        self.ball_pos = Vec2::new(w/2., h/2.);
        self.ball_velocity = Vec2::new(0., 0.);
        self.paddle_dimensions = Vec2::new(10., 30.)
    }
}