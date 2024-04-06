use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use crate::math_structures::color::Color;
use crate::NUM_OF_ACTIVE_THREADS;

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
    let mut texture = surface.as_texture(&texture_creator).unwrap();

    let max_chunk = width * 1000;
    let pitch = surface.pitch() as usize;
    let mut needed_index = 0;
    'running: loop {
        for event in win_sdl.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        let value = rx.recv();
        if value.is_ok() {
            let mut count = 1;
            let msg = value.unwrap();
            change_surface(msg.2, msg.1, msg.0, pitch, &mut surface);
            needed_index += 1;
            while let Ok(x) = rx.recv() {
                let msg = x;
                change_surface(msg.2, msg.1, msg.0, pitch, &mut surface);
                count += 1;
                if count > max_chunk {
                    break;
                }
            }

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
        }
    }

    Ok(())
}

fn change_surface(c: Color, y: i64, x: i64, pitch: usize, surface: &mut Surface) {
    let offset = y as usize * pitch;
    let x = x as usize;
    let r = (c.x() * 255.0) as u8;
    let g = (c.y() * 255.0) as u8;
    let b = (c.z() * 255.0) as u8;
    surface.with_lock_mut(|buffer: &mut [u8]| {
        buffer[3 * x + offset] = r;
        buffer[3 * x + offset + 1] = g;
        buffer[3 * x + offset + 2] = b;
    });
}
