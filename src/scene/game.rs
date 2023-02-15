use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;
use macroquad::math::f32::Vec2;
use macroquad::window::{screen_width, screen_height};

pub struct Game {
    player_paddle: Vec2,
    ai_paddle: Vec2,
    ball_pos: Vec2,
    ball_velocity: Vec2
}

impl Game {
    pub fn new() -> Self {
        let w = screen_width();
        let h = screen_height();

        Game {
            player_paddle: Vec2::new(10., h/2.),
            ai_paddle: Vec2::new(w-10., h/2.),
            ball_pos: Vec2::new(w/2., h/2.),
            ball_velocity: Vec2::new(0., 0.)
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

    }

    fn on_exit(&mut self) {
        // reset
        let w = screen_width();
        let h = screen_height();

        self.player_paddle = Vec2::new(10., h/2.);
        self.ai_paddle = Vec2::new(w-10., h/2.);
        self.ball_pos = Vec2::new(w/2., h/2.);
        self.ball_velocity = Vec2::new(0., 0.);
    }
}