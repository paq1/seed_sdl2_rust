extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

use crate::app::graphics::canvas_service_sdl::CanvasServiceImpl;
use crate::app::graphics::text_service_sdl::TextServiceSDL;
use crate::app::graphics::texture_creator_service::TextureCreatorService;
use crate::app::input::InputServiceImpl;
use crate::core::graphics::{CanvasService, TextService};
use crate::core::input::InputService;
use crate::core::scene::scene_menu::SceneMenu;
use crate::core::scene::SceneManager;

pub mod utils;
pub mod core;
pub mod app;

pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("seed sdl2 -- paq1", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize the video subsystem");
    let canvas = window.into_canvas()
        .build()
        .expect("Failed to initialize canvas");
    let texture_creator = canvas.texture_creator();

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
            ) as Box<dyn CanvasService<Sdl2TtfContext, WindowCanvas>>
        )
    );


    let text_service = Rc::new(
        RefCell::new(
            Box::new(
                TextServiceSDL::new(
                    Rc::clone(&canvas_service),
                    Rc::clone(&texture_creator_service)
                )
            ) as Box<dyn TextService<Sdl2TtfContext>>
        )
    );

    let mut event_pump = sdl_context.event_pump()?;

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
        // The rest of the game loop goes here...
        // render(canvas_service.borrow_mut(), input_service.borrow())?;
        scene_manager.update_scene(&ttf_context);

        canvas_service.borrow_mut().get_canvas().present();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
