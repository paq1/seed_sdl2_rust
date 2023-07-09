extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use once_cell::sync::Lazy;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Sdl2TtfContext;

use crate::app::factories::FontFactory;
use crate::app::factories::sprite_factory::SpriteFactory;
use crate::app::graphics::sprite_service_sdl2::SpriteServiceSdl2;
use crate::app::graphics::text_service_sdl::TextServiceSDL;
use crate::app::input::InputServiceImpl;
use crate::core::graphics::models::color::Color;
use crate::core::graphics::CanDrawText;
use crate::core::input::CanManageInput;
use crate::core::scene::SceneManager;

pub mod utils;
pub mod core;
pub mod app;

static TTF_CONTEXT: Lazy<Sdl2TtfContext> = Lazy::new(|| {
    sdl2::ttf::init().map_err(|e| e.to_string()).expect("erreur")
});

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
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

    let font_factory = Rc::new(
        RefCell::new(
            FontFactory::new(&TTF_CONTEXT)?
        )
    );

    let text_service: Rc<RefCell<TextServiceSDL>> = Rc::new(
        RefCell::new(
            TextServiceSDL::new(
                Rc::clone(&canvas),
                Rc::clone(&font_factory),
            )
        )
    );
    let input_service: Rc<RefCell<InputServiceImpl>> = Rc::new(
        RefCell::new(
            InputServiceImpl::new()
        )
    );

    let texture_creator = canvas.borrow().texture_creator();
    let sprite_factory: Rc<RefCell<SpriteFactory>> = Rc::new(RefCell::new(SpriteFactory::new(&texture_creator)?));

    let sprite_service: Rc<RefCell<SpriteServiceSdl2>> = Rc::new(RefCell::new(
        SpriteServiceSdl2 {
            canvas: Rc::clone(&canvas),
            sprite_factory: Rc::clone(&sprite_factory),
        }
    ));

    // todo -- initialisation du scene_manager 🤖
    let mut scene_manager = SceneManager::new(
        Rc::clone(&input_service),
        Rc::clone(&text_service),
        Rc::clone(&sprite_service),
    );

    // variables de calcul liée au frame et dt
    let mut last_frame_time = Instant::now();
    let mut time = 0f32;
    let mut frames_per_sec = 0u32;
    let mut frames = 0u32;

    let mut event_pump = sdl_context.event_pump()?;

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

        // todo -- calcul des fps (a bouger ailleur - dans un hud de debug)
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

        // todo -- boucle de gameplay 👾
        scene_manager.update_scene(delta_time);

        canvas.borrow_mut().present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
