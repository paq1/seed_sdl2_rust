pub mod scene_exemple_data;
pub mod player;

use std::cell::RefCell;
use std::rc::Rc;
use crate::core::elements::tilemap::tile::TileType;
use crate::core::graphics::models::color::Color;

use crate::core::graphics::{CanDrawSprite, CanDrawText};
use crate::core::input::CanManageInput;
use crate::core::musics::CanPlayMusic;
use crate::core::physics::collide_body::CanCollideWithTileMap;
use crate::core::scene::{SceneEnum};
use crate::core::scene::scene_exemple::scene_exemple_data::SceneExempleData;
use crate::core::sdd::vecteur2d::Vecteur2D;

pub struct SceneExemple<SpriteService, TextService, InputService, MusicService>
    where
        SpriteService: CanDrawSprite,
        TextService: CanDrawText,
        InputService: CanManageInput,
        MusicService: CanPlayMusic
{
    pub input_service: Rc<RefCell<InputService>>,
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
        self.update_curseur();
        self.update_camera();
        self.test_play_sound();

        self.draw_tilemap().expect("erreur lors de l'affichage de la map");
        self.draw_player().expect("erreur lors de l'affichage du player");
        self.draw_cursor().expect("erreur lors de l'affichage du curseur");

        let keys_pressed = self.get_keys_pressed();
        let mouse_key_pressed = self.get_mouse_keys_pressed();

        let font_size = 14u32;
        let pos = self.input_service.borrow_mut().get_mouse_position();


        vec![
            "-------- debug --------".to_string(),
            format!("keys = {}", keys_pressed),
            format!("mouse = ({}, {})", (pos.x + self.data.camera.x) as i32, (pos.y + self.data.camera.y) as i32),
            format!("mouse keys = {}", mouse_key_pressed)
        ]
            .iter()
            .enumerate()
            .for_each(|(index, debug_str)| {
                self.text_service.borrow_mut().create_text(
                    debug_str.as_str(),
                    10i32,
                    font_size as i32 * index as i32,
                    font_size,
                    Color::rgb(255u8, 255u8, 255u8)
                ).expect("erreur lors de l'affichage");
            });

        None
    }

    fn get_keys_pressed(&self) -> String {
        self
            .input_service
            .borrow()
            .key_pressed()
            .join("-")
    }

    fn get_mouse_keys_pressed(&self) -> String {
        self
            .input_service
            .borrow()
            .mouse_key_pressed()
            .join("-")
    }

    pub fn new(
        key_manager: Rc<RefCell<InputService>>,
        text_service: Rc<RefCell<TextService>>,
        sprite_service: Rc<RefCell<SpriteService>>,
        music_service: Rc<RefCell<MusicService>>,
    ) -> Self {
        Self {
            input_service: key_manager,
            text_service,
            sprite_service,
            music_service,
            data: SceneExempleData::new()
        }
    }

    fn init_scene(&mut self) -> Result<(), String> {
        if !self.data.is_init {
            self.data.is_init = true;
            self.music_service.borrow().play("hold-the-line", 10)
        } else {
            Ok(())
        }
    }

    fn update_player(&mut self, dt: f32) -> Result<(), String> {
        let vitesse = self.data.player.vitesse;
        let vitesse_temps = vitesse * dt;

        if self.input_service.borrow().is_key_pressed("Z") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.y -= vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.y -= vitesse_temps;
                self.data.player.collide_body.position.y -= vitesse_temps;
            }
        }
        if self.input_service.borrow().is_key_pressed("D") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.x += vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.x += vitesse_temps;
                self.data.player.collide_body.position.x += vitesse_temps;
            }
        }
        if self.input_service.borrow().is_key_pressed("S") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.y += vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.y += vitesse_temps;
                self.data.player.collide_body.position.y += vitesse_temps;
            }
        }
        if self.input_service.borrow().is_key_pressed("Q") {

            let mut col_body = self.data.player.collide_body.clone();
            col_body.position.x -= vitesse_temps;

            if !col_body.is_collide(&self.data.tilemap, vec![TileType::Mur]) {
                self.data.player.pos.x -= vitesse_temps;
                self.data.player.collide_body.position.x -= vitesse_temps;
            }
        }

        Ok(())
    }

    fn update_curseur(&mut self) {
        // on recup la pos du joueur et de la souris
        let pos_joueur = self.data.player.pos.clone();
        let pos_souris = self.input_service.borrow().get_mouse_position() + self.data.camera.clone();

        // on recupere le vecteur entre ces 2 points et on prend sa valeur unitaire
        let vec_joueur_curseur = Vecteur2D::<f32>::from_points(&pos_joueur, &pos_souris);
        let vec_joueur_curseur_unitaire = vec_joueur_curseur.unitaire();

        let distance_souris_joureur = vec_joueur_curseur.norme();

        // on met a jour la position du curseur uniquement si le calcul unitaire est possible
        match vec_joueur_curseur_unitaire {
            Some(unitaire) => {
                let distance_min = 32.0;
                let distance_max = distance_min * 2.0;

                let distance_viseur = if distance_souris_joureur > distance_max {
                    distance_max
                } else if distance_souris_joureur < distance_min {
                    distance_min
                } else {
                    distance_souris_joureur
                };

                self.data.pos_curseur = pos_joueur.clone() + Vecteur2D::new(
                    unitaire.x * distance_viseur,
                    unitaire.y * distance_viseur
                )
            }
            _ => ()
        }
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
            Vecteur2D::new(
                (self.data.player.pos.x - self.data.camera.x - 16f32) as i32,
                (self.data.player.pos.y - self.data.camera.y - 16f32) as i32
            )
            ,None, None
        )
    }

    fn draw_cursor(&mut self) -> Result<(), String> {
        self.sprite_service.borrow_mut().draw_sprite(
            "viseur",
            Vecteur2D::new(
                (self.data.pos_curseur.x - self.data.camera.x - 16f32) as i32,
                (self.data.pos_curseur.y - self.data.camera.y - 16f32) as i32
            )
            , Some(Vecteur2D::new(512, 512)), None
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
                            Vecteur2D::new(
                                current.pos.x as i32 * 32 - self.data.camera.x as i32,
                                current.pos.y as i32 * 32 - self.data.camera.y as i32
                            )
                            , None, None
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
        if self.input_service.borrow().is_key_pressed("X") {
            self.music_service.borrow().play_sound("arme", 1).expect("erreur lors de la lecture du son arme");
        }
    }
}