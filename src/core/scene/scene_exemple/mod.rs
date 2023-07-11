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

        self.draw_tilemap().expect("erreur lors de l'affichage de la map");
        self.draw_player().expect("erreur lors de l'affichage du player");

        let keys_pressed = self.get_keys_pressed();
        self.text_service.borrow_mut().create_text(
            format!("keys = {}", keys_pressed).as_str(),
            10i32,
            0i32,
            32u32,
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
        let vitesse = 100f32;
        let vitesse_temps = vitesse * dt;

        if self.key_manager.borrow().is_key_pressed("Z") {
            self.data.player.y -= vitesse_temps
        }
        if self.key_manager.borrow().is_key_pressed("D") {
            self.data.player.x += vitesse_temps
        }
        if self.key_manager.borrow().is_key_pressed("S") {
            self.data.player.y += vitesse_temps
        }
        if self.key_manager.borrow().is_key_pressed("Q") {
            self.data.player.x -= vitesse_temps
        }
        Ok(())
    }

    fn draw_player(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "smiley",
            self.data.player.x as i32,
            self.data.player.y as i32,
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
                    .filter(|current| SceneExemple::<SpriteService, TextService, InputService, MusicService>::is_in_screen(current.x as i32 * 32, current.y as i32 * 32))
                    .for_each(|current| {
                        self.sprite_service.borrow_mut().draw_sprite(
                            "tile_herbe",
                            current.x as i32 * 32,
                            current.y as i32 * 32,
                        ).expect("erreur de lors de la 'affiche de la tuile");
                    });
            });

        Ok(())
        //
        // self.sprite_service.borrow_mut().draw_sprite(
        //     "smiley",
        //     self.data.player.x as i32,
        //     self.data.player.y as i32,
        // )
    }

    fn is_in_screen(point_x: i32, point_y: i32) -> bool {
        point_x > 0 && point_x < 600 && point_y > 0 && point_y < 600
    }
}