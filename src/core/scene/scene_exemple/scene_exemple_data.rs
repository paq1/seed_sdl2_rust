use crate::core::scene::scene_exemple::player::Player;

pub struct SceneExempleData {
    pub is_init: bool,
    pub player: Player
}

impl SceneExempleData {
    pub fn new() -> Self {
        Self {
            is_init: false,
            player: Player {
                x: 300f32,
                y: 300f32
            }
        }
    }
}