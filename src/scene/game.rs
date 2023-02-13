use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;

pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        Game {}
    }
} 

impl Scene for Game {
    fn update(&mut self) -> SceneId {
        SceneId::Game
    }
}