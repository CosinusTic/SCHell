extern crate sdl2;

use anyhow::{self};
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::io::{stdin, stdout, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const FONT_PATH: &str = "/usr/share/fonts/archcraft/nerd-fonts/Hack/HackNerdFont-Bold.ttf";
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

pub fn main() -> Result<(), anyhow::Error> {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut cursor_visible = true;
    let mut last_cursor_toggle = Instant::now();
    let cursor_blink_interval = Duration::from_millis(500);
    let pty_system = native_pty_system();
    let rows: u16 = 24;
    let cols: u16 = 80;

    let p2 = "/home/cosinutt/Documents/Personal/SCHell/target/debug/shell";

    let pair: PtyPair = pty_system.openpty(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let cmd = CommandBuilder::new(p2);
    let mut child = pair.slave.spawn_command(cmd)?;
    // let mut writer = pair.master.take_writer()?;
    // let mut reader = pair.master.try_clone_reader()?;
    let mut writer = Arc::new(Mutex::new(pair.master.take_writer()?));
    let mut reader = Arc::new(Mutex::new(pair.master.try_clone_reader()?));
    let output_lines = Arc::new(Mutex::new(Vec::<String>::new()));

    let window = video_subsystem
        .window("SDL2 Terminal", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font(FONT_PATH, FONT_SIZE).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // let mut input_buffer = String::new();
    let mut input_buffer = String::new();
    let mut lines: Vec<String> = vec![];
    let line_height = font.recommended_line_spacing();

    // FW stdin from master to slave
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
                    let writer_clone = Arc::clone(&writer);
                    let reader_clone = Arc::clone(&reader);
                    let input = input_buffer.clone();
                    lines.push(input_buffer.clone());
                    thread::spawn(move || {
                        let mut writer = writer_clone.lock().unwrap();
                        if writer.write_all(input.as_bytes()).is_err() {
                            println!("Cannot write to slave (shell)")
                        } else {
                            println!("Successfully wrote to slave: {}", input);
                        }
                    });
                    thread::spawn(move || {
                        let mut out = stdout();
                        let mut reader = reader_clone.lock().unwrap();
                        let mut buf = [0u8; 1024];
                        loop {
                            match reader.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => {
                                    if out.write_all(&buf[..n]).is_err() {
                                        break;
                                    } else {
                                        let s: String =
                                            String::from_utf8_lossy(&buf[..n]).to_string();
                                        println!("Recieved from slave: {}", s);
                                        // Append to input_buffer here ?
                                    }
                                    let _ = out.flush();
                                }
                                Err(_) => break,
                            }
                        }
                    });
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

/*
fn main() -> anyhow::Result<()> {




    // FW stdout from slave to master

    child.wait()?;
    Ok(())
}
*/
