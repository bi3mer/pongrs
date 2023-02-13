use macroquad::{prelude::{WHITE, GRAY}, window::{screen_width, screen_height}, text::{draw_text, get_text_center}};
use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;
use crate::ui::Button;

pub struct Menu {

}

impl Menu {
    pub fn new() -> Self {
        Menu {}
    }
} 

impl Scene for Menu {
    fn update(&mut self) -> SceneId {
        let w = screen_width();
        let h = screen_height();

        // draw title
        let title = "Pong";
        let mut center = get_text_center(title, None, 60, 1., 0.);
        draw_text(
            title, 
            screen_width()/2. - center.x, 
            screen_height()/3., 
            60., 
            WHITE
        );

        // draw play button and check if pressed by the player
        let mut target_scene = SceneId::Menu;
        let play_text = " Play";
        center = get_text_center(play_text, None, 32, 1., 0.);

        if Button::new()
            .dimensions(80., 50.)
            .color(GRAY)
            .hover_color(WHITE)
            .text(play_text.to_string())
            .font_size(32.)
            .font_color(WHITE)
            .is_active(true)
            .pos(w/2. - center.x, h/2. - center.y)
            .draw() 
        {
            target_scene = SceneId::Game;
        }


        target_scene
    }
}