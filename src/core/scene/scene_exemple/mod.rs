pub mod scene_exemple_data;
pub mod player;
pub mod tile_map;

use std::cell::RefCell;
use std::rc::Rc;
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::scene::{SceneEnum};
use crate::core::scene::scene_exemple::scene_exemple_data::SceneExempleData;
use crate::core::scene::scene_exemple::tile_map::TileType;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SceneExemple<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub key_manager: Rc<RefCell<InputService>>,
    pub text_service: Rc<RefCell<TextService>>,
    pub sprite_service: Rc<RefCell<SpriteService>>,
    pub music_service: Rc<RefCell<MusicService>>,
    pub data: SceneExempleData
}

impl<SpriteService, TextService, InputService, MusicService> SceneExemple<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub fn on_scene(
        &mut self,
        dt: f32
    ) -> Option<SceneEnum<SpriteService, TextService, InputService, MusicService>> {

        self.init_scene().expect("erreur lors de l'initialisation de la scene");

        self.update_player(dt).expect("erreur lors de l'update du player");
        self.update_camera();
        self.test_play_sound();

        self.draw_tilemap().expect("erreur lors de l'affichage de la map");
        self.draw_player().expect("erreur lors de l'affichage du player");

        let keys_pressed = self.get_keys_pressed();
        self.text_service.borrow_mut().create_text(
            format!("keys = {}", keys_pressed).as_str(),
            10i32,
            0i32,
            14u32,
            Color::rgb(0u8, 200u8, 100u8)
        ).expect("erreur lors de l'affichage");

        None
    }

    fn get_keys_pressed(&self) -> String {
        self
            .key_manager
            .borrow()
            .key_pressed()
            .join("-")
    }

    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
    ) -> Self {
        Self {
            key_manager,
            text_service,
            sprite_service,
            music_service,
            data: SceneExempleData::new()
        }
    }

    fn init_scene(&mut self) -> Result<(), String> {
        if !self.data.is_init {
            self.data.is_init = true;
            self.music_service.borrow().play("digital-love")
        } else {
            Ok(())
        }
    }

    fn update_player(&mut self, dt: f32) -> Result<(), String> {
        let vitesse = self.data.player.vitesse;
        let vitesse_temps = vitesse * dt;
        let old_pos = self.data.player.pos.clone();

        if self.key_manager.borrow().is_key_pressed("Z") {
            self.data.player.pos.y -= vitesse_temps;
        }
        if self.key_manager.borrow().is_key_pressed("D") {
            self.data.player.pos.x += vitesse_temps;
        }
        if self.key_manager.borrow().is_key_pressed("S") {
            self.data.player.pos.y += vitesse_temps;
        }
        if self.key_manager.borrow().is_key_pressed("Q") {
            self.data.player.pos.x -= vitesse_temps;
        }

        let tile = self.data.tilemap.get_tile_from_position(&self.data.player.pos);
        match tile {
            Some(x) if x.r#type == TileType::Mur => self.data.player.pos = old_pos,
            None => self.data.player.pos = old_pos,
            _ => ()
        };

        Ok(())
    }

    fn update_camera(&mut self) {
        let window_width = 800f32; // fixme utiliser un service window afin de recup les infos de la window
        let window_height = 600f32; // fixme utiliser un service window afin de recup les infos de la window
        // let vec_player = self.data.player.pos.clone();
        self.data.camera = Vecteur2D::new(
            self.data.player.pos.x - window_width / 2.0,
            self.data.player.pos.y - window_height / 2.0,
        );
    }

    fn draw_player(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "smiley",
            self.data.player.pos.x as i32 - self.data.camera.x as i32 - 16,
            self.data.player.pos.y as i32 - self.data.camera.y as i32 - 16,
        )
    }

    fn draw_tilemap(&mut self) -> Result<(), String> {
        self.data
            .tilemap
            .tiles
            .iter_mut()
            .for_each(|line| {
                line
                    .iter_mut()
                    .filter(|current| {
                        SceneExemple::<
                            SpriteService,
                            TextService,
                            InputService,
                            MusicService
                        >::is_in_screen(
                            current.pos.x as i32 * 32 - self.data.camera.x as i32,
                            current.pos.y as i32 * 32 - self.data.camera.y as i32
                        )
                    })
                    .for_each(|current| {

                        let sprite_index = if current.r#type == TileType::Herbe {
                            "tile_herbe"
                        } else {
                            "tile_brique"
                        };

                        self.sprite_service.borrow_mut().draw_sprite(
                            sprite_index,
                            current.pos.x as i32 * 32 - self.data.camera.x as i32,
                            current.pos.y as i32 * 32 - self.data.camera.y as i32,
                        ).expect("erreur de lors de la 'affiche de la tuile");
                    });
            });

        Ok(())
    }

    fn is_in_screen(point_x: i32, point_y: i32) -> bool {
        let window_width = 800;
        let window_height = 600;
        let margin = 100;
        // fixme utilise un service window (pas encore dev) afin de recupere ces info
        point_x > 0 - margin && point_x < window_width && point_y > 0 - margin && point_y < window_height
    }

    fn test_play_sound(&self) {
        if self.key_manager.borrow().is_key_pressed("X") {
            self.music_service.borrow().play_sound("arme").expect("erreur lors de la lecture du son arme");
        }
    }
}