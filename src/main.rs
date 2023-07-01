extern crate sdl2;

use std::path::Path;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let sdl_ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("seed sdl2 -- paq1", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize the video subsystem");

    let mut canvas = window.into_canvas()
        .build()
        .expect("Failed to initialize canvas");

    let texture_creator = canvas.texture_creator();

    // load font
    let font_path: &Path = Path::new(&"assets/fonts/dpcomic.ttf");
    let mut font = sdl_ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        render(&mut canvas, &texture_creator, &font)?;

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}

fn render(canvas: &mut WindowCanvas, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();

    let text_hello = "hello world".to_string();
    let surface = font
        .render(&text_hello)
        .blended(Color::RGB(255,0,255))
        .map_err(|err| err.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(surface)
        .map_err(|err| err.to_string())?;

    let target = Rect::new(10i32, 0i32, 600u32, 100u32);

    canvas.copy(&texture, None, Some(target))?;

    canvas.present();
    Ok(())
}
