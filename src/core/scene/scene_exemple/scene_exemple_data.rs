use crate::core::scene::scene_exemple::player::Player;
use crate::core::scene::scene_exemple::tile_map::TileMap;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SceneExempleData {
    pub is_init: bool,
    pub player: Player,
    pub camera: Vecteur2D<f32>,
    pub tilemap: TileMap // systeme de map basique (si trop grande joue sur les perfs)
}

impl SceneExempleData {
    pub fn new() -> Self {
        Self {
            is_init: false,
            player: Player {
                pos: Vecteur2D::new(
                    300f32,
                    300f32
                )

            },
            camera: Vecteur2D::new(300f32, 300f32),
            tilemap: TileMap::new(200, 200)
        }
    }
}