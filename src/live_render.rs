use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::winsdl::Winsdl;

pub fn show_screen(
    height: usize,
    width: usize,
    rx: Receiver<(i64, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut win_sdl = Winsdl::new(width, height)?;

    let mut canvas = win_sdl
        .window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, width as u32, height as u32)
        .map_err(|e| e.to_string())?;

    let mut pending_map = HashMap::new();

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
            let msg = value.unwrap();
            pending_map.insert(msg.0, msg.1);
        }

        if pending_map.contains_key(&needed_index) {
            while pending_map.contains_key(&needed_index) {
                let msg = pending_map.remove(&needed_index).unwrap().to_owned();
                change_texture(
                    (needed_index, msg),
                    height,
                    width,
                    &mut texture,
                    &mut canvas,
                );
                needed_index += 1;
            }
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

fn change_texture(
    msg: (i64, String),
    height: usize,
    width: usize,
    texture: &mut Texture,
    canvas: &mut WindowCanvas,
) {
    let y = msg.0;
    let mut data = msg.1.trim().split("\n");
    for x in 0..width {
        let mut line = data.next().unwrap().split(" ");
        let r = line.next().unwrap().trim().parse::<i64>().unwrap();
        let g = line.next().unwrap().trim().parse::<i64>().unwrap();
        let b = line.next().unwrap().trim().parse::<i64>().unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let index = y as usize * pitch + x * 3;
                buffer[index] = r as u8;
                buffer[index + 1] = g as u8;
                buffer[index + 2] = b as u8;
            })
            .map_err(|e| format!("Error updating texture: {}", e))
            .unwrap();
    }
}
