extern crate sdl2;

use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::Sdl;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use crate::app::graphics::CanvasServiceImpl;
use crate::app::input;
use crate::app::input::InputServiceImpl;
use crate::core::graphics::CanvasService;
use crate::core::input::InputService;
use crate::core::scene::scene_menu::SceneMenu;
use crate::core::scene::{Scene, SceneManager};
use crate::core::scene::scene_exemple::SceneExemple;

pub mod utils;
pub mod core;
pub mod app;

type InputServiceAbstrait = Rc<RefCell<Box<dyn InputService>>>;

pub fn main() -> Result<(), String> {
    let mut canvas_service = Rc::new(
        RefCell::new(
            Box::new(
                CanvasServiceImpl::new()?
            ) as Box<dyn CanvasService<Sdl, WindowCanvas>>
        )
    );

    let mut event_pump = canvas_service.borrow_mut().get_ctx().event_pump()?;
    let mut i = 0;

    // let mut key_manager: HashMap<String, bool> = HashMap::new();
    let mut input_service = Rc::new(
        RefCell::new(
            Box::new(InputServiceImpl::new()) as Box<dyn InputService>
        )
    );

    let mut scene_menu = SceneMenu {
        key_manager: Rc::clone(&input_service),
        canvas_service: Rc::clone(&canvas_service)
    };
    let mut scene_manager = SceneManager { current: Box::new(scene_menu) };

    'running: loop {
        i = (i + 1) % 255;
        // canvas_service.borrow_mut().get_canvas().set_draw_color(Color::RGB(i, 64, 255 - i));
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
        if let Some(x) = scene_manager.current.on_scene() {
            scene_manager.current = x;
        }

        canvas_service.borrow_mut().get_canvas().present();
        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
