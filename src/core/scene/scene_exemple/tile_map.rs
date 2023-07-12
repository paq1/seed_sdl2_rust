use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct Tile {
    pub pos: Vecteur2D<f32>
}

pub struct TileMap {
    pub tiles: Vec<Vec<Tile>>
}

impl TileMap {
    pub fn new(w: u32, h: u32) -> Self {
        let tiles = (0u32 .. h)
            .into_iter()
            .map(|current_line| {
                (0u32 .. w)
                    .into_iter()
                    .map(|current_column| {
                        Tile {
                            pos: Vecteur2D::new(
                                current_column as f32,
                                current_line as f32
                            )
                        }
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect::<Vec<Vec<Tile>>>();

        Self { tiles }
    }
}