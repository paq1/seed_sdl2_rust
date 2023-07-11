use crate::core::scene::scene_exemple::player::Player;
use crate::core::scene::scene_exemple::tile_map::TileMap;

pub struct SceneExempleData {
    pub is_init: bool,
    pub player: Player,
    pub tilemap: TileMap // systeme de map basique (si trop grande joue sur les perfs)
}

impl SceneExempleData {
    pub fn new() -> Self {
        Self {
            is_init: false,
            player: Player {
                x: 300f32,
                y: 300f32
            },
            tilemap: TileMap::new(200, 200)
        }
    }
}