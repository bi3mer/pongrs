use crate::scene::scene_id::SceneId;
use crate::scene::scene_trait::Scene;

pub struct Menu {

}

impl Menu {
    pub fn new() -> Self {
        Menu {}
    }
} 

impl Scene for Menu {
    fn update(&mut self) -> SceneId {
        SceneId::Menu
    }
}