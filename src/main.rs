
use macroquad::prelude::*;

mod ui;
mod scene;
use scene::{menu::Menu, game::Game, scene_id::SceneId, scene_trait::Scene, game_over::GameOver};

#[macroquad::main("Pong")]
async fn main() {
    let mut scene = SceneId::Menu;
    
    let mut menu_scene = Menu::new();
    let mut game_scene = Game::new();
    let mut game_over_scene = GameOver::new();
    
    let mut current_scene: &mut dyn Scene = &mut menu_scene;
    current_scene.on_enter();

    loop {
        clear_background(BLACK);
        
        let new_scene = current_scene.update(get_frame_time());
        current_scene.render();

        if new_scene != scene {
            current_scene.on_exit();
            scene = new_scene;
            
            match scene {
                SceneId::Menu => {
                    current_scene = &mut menu_scene;
                },
                SceneId::Game => {
                    current_scene = &mut game_scene;
                },
                SceneId::GameOver => {
                game_over_scene.set_winner(game_scene.get_winner());
                    current_scene = &mut game_over_scene;
                },
            }
            current_scene.on_enter();
        }

        next_frame().await
    }
}