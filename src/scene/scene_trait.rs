use crate::scene::scene_id::SceneId;

pub trait Scene {
    fn on_enter(&mut self);
    fn update(&mut self, dt: f32) -> SceneId;
    fn render(&mut self);
    fn on_exit(&mut self);
}