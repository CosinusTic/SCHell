extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::time::{Duration, Instant};

const FONT_PATH: &str = "/usr/share/fonts/truetype/hack/Hack-Regular.ttf";
const FONT_SIZE: u16 = 16;

fn render_line<'a>(
    font: &Font,
    texture_creator: &'a TextureCreator<WindowContext>,
    text: &str,
) -> Option<sdl2::render::Texture<'a>> {
    if text.is_empty() {
        return None;
    }
    let surface = font.render(text).blended(Color::WHITE).ok()?;
    texture_creator.create_texture_from_surface(&surface).ok()
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video()?;
    let mut cursor_visible = true;
    let mut last_cursor_toggle = Instant::now();
    let cursor_blink_interval = Duration::from_millis(500);

    let window = video_subsystem
        .window("SDL2 Terminal", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font(FONT_PATH, FONT_SIZE)?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut input_buffer = String::new();
    let mut lines: Vec<String> = vec![];
    let line_height = font.recommended_line_spacing();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::TextInput { text, .. } => {
                    input_buffer.push_str(&text);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    input_buffer.pop();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    lines.push(input_buffer.clone());
                    // Interpret commands here
                    println!("Buffer: {}", input_buffer);
                    input_buffer.clear();
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Render previous lines
        for (i, line) in lines.iter().enumerate() {
            if let Some(texture) = render_line(&font, &texture_creator, line) {
                let TextureQuery { width, height, .. } = texture.query();
                let _ = canvas.copy(
                    &texture,
                    None,
                    Some(Rect::new(5, (i as i32) * line_height, width, height)),
                );
            }
        }

        if last_cursor_toggle.elapsed() >= cursor_blink_interval {
            cursor_visible = !cursor_visible;
            last_cursor_toggle = Instant::now();
        }

        let mut render_text = input_buffer.clone();
        if cursor_visible {
            render_text.push('_');
        }
        // Render current input buffer
        if let Some(texture) = render_line(&font, &texture_creator, &render_text) {
            let TextureQuery { width, height, .. } = texture.query();
            let _ = canvas.copy(
                &texture,
                None,
                Some(Rect::new(
                    5,
                    (lines.len() as i32) * line_height,
                    width,
                    height,
                )),
            );
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
