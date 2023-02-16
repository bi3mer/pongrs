use macroquad::{prelude::{WHITE, GRAY}, window::{screen_width, screen_height}, text::{draw_text, get_text_center}};
use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;
use crate::ui::Button;

pub struct GameOver {
    replay: Button,
    quit: Button,
    winner: String
}

impl GameOver {
    pub fn new() -> Self {
        let mut replay = Button::new();
        replay
            .dimensions(120., 50.)
            .color(GRAY)
            .hover_color(WHITE)
            .text(" Replay".to_string())
            .font_size(32.)
            .font_color(WHITE)
            .is_active(true);

        let mut quit = Button::new();
        quit
            .dimensions(80., 50.)
            .color(GRAY)
            .hover_color(WHITE)
            .text(" Quit".to_string())
            .font_size(32.)
            .font_color(WHITE)
            .is_active(true);

        GameOver {
            replay: replay,
            quit: quit,
            winner: "".to_string()
        }
    }

    pub fn set_winner(&mut self, winner: &str) {
        self.winner = winner.to_string();
    }
} 

impl Scene for GameOver {
    fn on_enter(&mut self) {
        // nothing to do
    }
    
    fn update(&mut self, _dt: f32) -> SceneId {
        let mut center = get_text_center(" Replay", None, 32, 1., 0.);
        self.replay.pos(screen_width()*0.4 - center.x, screen_height()*0.5 - center.y);
        self.replay.update();

        center = get_text_center(" Quit", None, 32, 1., 0.);
        self.quit.pos(screen_width()*0.6 - center.x, screen_height()*0.5 - center.y);
        self.quit.update();

        if self.replay.was_clicked() { SceneId::Game }
        else if self.quit.was_clicked() { SceneId::Menu }
        else { SceneId::GameOver }
    }

    fn render(&mut self) {
        // draw title
        let title = format!("{} Won!", self.winner);
        let center = get_text_center(&title, None, 60, 1., 0.);
        draw_text(
            &title, 
            screen_width()/2. - center.x, 
            screen_height()/3., 
            60., 
            WHITE
        );

        // draw play button
        self.replay.render();
        self.quit.render();
    }

    fn on_exit(&mut self) {
        // nothing to do
    }
}