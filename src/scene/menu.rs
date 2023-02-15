use macroquad::{prelude::{WHITE, GRAY}, window::{screen_width, screen_height}, text::{draw_text, get_text_center}};
use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;
use crate::ui::Button;

pub struct Menu {
    play_button: Button
}

impl Menu {
    pub fn new() -> Self {
        let mut play_button = Button::new();
        play_button
            .dimensions(80., 50.)
            .color(GRAY)
            .hover_color(WHITE)
            .text(" Play".to_string())
            .font_size(32.)
            .font_color(WHITE)
            .is_active(true);

        Menu {
            play_button: play_button
        }
    }
} 

impl Scene for Menu {
    fn on_enter(&mut self) {
        // nothing to do
    }
    
    fn update(&mut self) -> SceneId {
        let center = get_text_center(" Play", None, 32, 1., 0.);
        self.play_button.pos(screen_width()/2. - center.x, screen_height()/2. - center.y);
        self.play_button.update();

        if self.play_button.was_clicked() { SceneId::Game } else { SceneId::Menu }
    }

    fn render(&mut self) {
        // draw title
        let title = "Pong";
        let center = get_text_center(title, None, 60, 1., 0.);
        draw_text(
            title, 
            screen_width()/2. - center.x, 
            screen_height()/3., 
            60., 
            WHITE
        );

        // draw play button
        self.play_button.render();
    }


    fn on_exit(&mut self) {
        // nothing to do
    }
}