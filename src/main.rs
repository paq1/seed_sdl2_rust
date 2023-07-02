extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

use crate::app::graphics::CanvasServiceImpl;
use crate::app::input::InputServiceImpl;
use crate::core::graphics::CanvasService;
use crate::core::input::InputService;
use crate::core::scene::SceneManager;
use crate::core::scene::scene_menu::SceneMenu;

pub mod utils;
pub mod core;
pub mod app;

pub fn main() -> Result<(), String> {
    let canvas_service = Rc::new(
        RefCell::new(
            Box::new(
                CanvasServiceImpl::new()?
            ) as Box<dyn CanvasService<Sdl, WindowCanvas>>
        )
    );

    let mut event_pump = canvas_service.borrow_mut().get_ctx().event_pump()?;

    let input_service = Rc::new(
        RefCell::new(
            Box::new(InputServiceImpl::new()) as Box<dyn InputService>
        )
    );

    let scene_menu = SceneMenu {
        key_manager: Rc::clone(&input_service),
        canvas_service: Rc::clone(&canvas_service)
    };
    let mut scene_manager = SceneManager { current: Box::new(scene_menu) };

    'running: loop {
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
