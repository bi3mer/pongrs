use crate::scene::scene_id::SceneId;

pub trait Scene {
    fn update(&mut self) -> SceneId;
}