extern crate sdl2;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::Instant;

use once_cell::sync::Lazy;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;

use crate::app::factories::FontFactory;
use crate::app::factories::sprite_factory::SpriteFactory;
use crate::app::graphics::sprite_service_sdl2::SpriteServiceSdl2;
use crate::app::graphics::text_service_sdl::TextServiceSDL;
use crate::app::graphics::texture_creator_service::TextureCreatorService;
use crate::app::input::InputServiceImpl;
use crate::core::graphics::models::color::Color;
use crate::core::graphics::{SpriteService, TextService};
use crate::core::input::InputService;
use crate::core::scene::scene_menu::SceneMenu;
use crate::core::scene::SceneManager;

pub mod utils;
pub mod core;
pub mod app;

static TTF_CONTEXT: Lazy<Sdl2TtfContext> = Lazy::new(|| {
    sdl2::ttf::init().map_err(|e| e.to_string()).expect("erreur")
});

static SDL_CONTEXT: Sdl = sdl2::init().unwrap();

pub fn main() -> Result<(), String> {
    // let sdl_context = sdl2::init()?;
    let video_subsystem = SDL_CONTEXT.video()?;
    let window = video_subsystem.window("seed sdl2 -- paq1", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize the video subsystem");
    let canvas = Rc::new(
        RefCell::new(window.into_canvas()
            .build()
            .expect("Failed to initialize canvas")
        )
    );
    let texture_creator = canvas.borrow().texture_creator();

    let font_factory = Rc::new(
        RefCell::new(
            FontFactory::new(&TTF_CONTEXT)?
        )
    );
    let texture_creator_service = Rc::new(
        RefCell::new(
            TextureCreatorService {
                texture_creator
            }
        )
    );

    let text_service: Rc<RefCell<Box<dyn TextService>>> = Rc::new(
        RefCell::new(
            Box::new(
                TextServiceSDL::new(
                    Rc::clone(&canvas),
                    Rc::clone(&texture_creator_service),
                    Rc::clone(&font_factory),
                )
            )
        )
    );
    let input_service = Rc::new(
        RefCell::new(
            Box::new(InputServiceImpl::new()) as Box<dyn InputService>
        )
    );

    let tc = Rc::new(canvas.borrow().texture_creator());
    // let mut texture_manager = TextureManager::new(&tc);
    let sprite_factory: Rc<RefCell<SpriteFactory>> = Rc::new(RefCell::new(SpriteFactory::new(&tc)?));

    let sprite_service: Rc<RefCell<Box<dyn SpriteService>>> = Rc::new(RefCell::new( Box::new(
        SpriteServiceSdl2 {
            canvas: Rc::clone(&canvas),
            sprite_factory: Rc::clone(&sprite_factory)
        }
    )));

    let tc2 = Rc::new(canvas.borrow().texture_creator());

    let texture_test = tc2.load_texture("assets/sprites/smiley_sdl_seed.bmp")?;

    let scene_menu = SceneMenu {
        key_manager: Rc::clone(&input_service),
        text_service: Rc::clone(&text_service),
        sprite_service: Rc::clone(&sprite_service)
    };
    let mut scene_manager = SceneManager { current: Box::new(scene_menu) };

    let mut event_pump = sdl_context.event_pump()?;

    let mut last_frame_time = Instant::now();

    let mut time = 0f32;

    let mut frames_per_sec = 0u32;
    let mut frames = 0u32;

    'running: loop {
        canvas.borrow_mut().clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        None => {}
                        Some(key) => {
                            input_service.borrow_mut().key_down(key.to_string());
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        None => {}
                        Some(key) => {
                            input_service.borrow_mut().key_up(key.to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;

        time += delta_time;
        frames += 1;

        if time >= 1f32 {
            time = 0f32;
            frames_per_sec = frames;
            frames = 0;
        }

        text_service.borrow().create_text(
            format!("fps : {}", frames_per_sec).as_str(),
            600i32,
            0i32,
            16u32,
            Color::rgb(255u8, 0u8, 0u8),
        )?;

        // The rest of the game loop goes here...
        // render(canvas_service.borrow_mut(), input_service.borrow())?;
        scene_manager.update_scene(delta_time);

        canvas.borrow_mut().copy_ex(
            &texture_test,
            Rect::new(0,0,32,32),
            Rect::new(0,0,32,32),
            0.0,
            Point::new(16, 16),
            false,
            false
        )?;

        canvas.borrow_mut().present();
        // canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
