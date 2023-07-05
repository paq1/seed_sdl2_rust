extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use crate::app::factories::FontFactory;

use crate::app::graphics::canvas_service_sdl::CanvasServiceImpl;
use crate::app::graphics::text_service_sdl::TextServiceSDL;
use crate::app::graphics::texture_creator_service::TextureCreatorService;
use crate::app::input::InputServiceImpl;
use crate::core::graphics::{CanvasService, TextService};
use crate::core::input::InputService;
use crate::core::scene::scene_menu::SceneMenu;
use crate::core::scene::SceneManager;

use once_cell::sync::{Lazy};
use crate::core::graphics::models::color::Color;

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
    let canvas = window.into_canvas()
        .build()
        .expect("Failed to initialize canvas");
    let texture_creator = canvas.texture_creator();

    let font_factory = Rc::new(
        RefCell::new(
            FontFactory :: new(&TTF_CONTEXT)?
        )
    );
    let texture_creator_service = Rc::new(
        RefCell::new(
            TextureCreatorService {
                texture_creator
            }
        )
    );
    let canvas_service = Rc::new(
        RefCell::new(
            Box::new(
                CanvasServiceImpl::new(canvas, Rc::clone(&texture_creator_service))?
            ) as Box<dyn CanvasService<WindowCanvas>>
        )
    );
    let text_service = Rc::new(
        RefCell::new(
            Box::new(
                TextServiceSDL::new(
                    Rc::clone(&canvas_service),
                    Rc::clone(&texture_creator_service),
                    Rc::clone(&font_factory)
                )
            ) as Box<dyn TextService>
        )
    );
    let input_service = Rc::new(
        RefCell::new(
            Box::new(InputServiceImpl::new()) as Box<dyn InputService>
        )
    );

    let scene_menu = SceneMenu {
        key_manager: Rc::clone(&input_service),
        text_service: Rc::clone(&text_service)
    };
    let mut scene_manager = SceneManager { current: Box::new(scene_menu) };

    let mut event_pump = sdl_context.event_pump()?;

    let mut last_frame_time = Instant::now();

    let mut time = 0f32;

    let mut frames_per_sec = 0u32;
    let mut frames = 0u32;

    'running: loop {
        canvas_service.borrow_mut().get_canvas().clear();
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
            Color::rgb(255u8, 0u8, 0u8)
        )?;

        // The rest of the game loop goes here...
        // render(canvas_service.borrow_mut(), input_service.borrow())?;
        scene_manager.update_scene(delta_time);

        canvas_service.borrow_mut().get_canvas().present();
        // canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
