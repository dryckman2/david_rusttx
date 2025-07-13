use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use crate::math_structures::color::Color;
use crate::winsdl::Winsdl;

pub fn show_screen(
    width: usize,
    height: usize,
    rx: Receiver<(i64, i64, Color)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut win_sdl = Winsdl::new(width, height)?;
    let mut canvas = win_sdl
        .window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut surface = Surface::new(width as u32, height as u32, PixelFormatEnum::RGB24).unwrap();
    let mut texture;

    let mut last_applied = Instant::now();
    let max_wait = Duration::from_millis(16);
    let pitch = surface.pitch() as usize;
    let mut pending_pixels = vec![];
    'running: loop {
        while let Some(event) = win_sdl.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {
                    break;
                }
            }
        }
        let value = rx.recv();
        if value.is_ok() {
            pending_pixels.push(value.unwrap());
        }

        if max_wait < last_applied.elapsed() {
            change_surface(&pending_pixels, pitch, &mut surface);
            pending_pixels.clear();
            texture = surface.as_texture(&texture_creator).unwrap();
            // Clear the canvas
            canvas.clear();
            // Render the texture
            canvas
                .copy(
                    &texture,
                    None,
                    Some(Rect::new(0, 0, width as u32, height as u32)),
                )
                .unwrap();
            // Present the updated canvas
            canvas.present();
            last_applied = Instant::now();
        }
    }

    Ok(())
}

fn change_surface(pending_pixels: &Vec<(i64, i64, Color)>, pitch: usize, surface: &mut Surface) {
    for msg in pending_pixels {
        let offset = msg.1 as usize * pitch;
        let x = 3 * msg.0 as usize;
        let c = msg.2;
        let r = c.x() as u8;
        let g = c.y() as u8;
        let b = c.z() as u8;
        surface.with_lock_mut(|buffer: &mut [u8]| {
            buffer[x + offset] = r;
            buffer[x + offset + 1] = g;
            buffer[x + offset + 2] = b;
        });
    }
}
