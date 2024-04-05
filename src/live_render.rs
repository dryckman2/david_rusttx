use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

use crate::winsdl::Winsdl;

pub fn show_screen(
    width: usize,
    height: usize,
    rx: Receiver<(i64, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut win_sdl = Winsdl::new(width, height)?;2
    let mut canvas = win_sdl
        .window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut surface = Surface::new(width as u32, height as u32, PixelFormatEnum::RGB888).unwrap();
    let mut texture = surface.as_texture(&texture_creator).unwrap();
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
                change_surface(
                    msg,
                    needed_index,
                    width,
                    &mut surface,
                );
                texture = surface.as_texture(&texture_creator).unwrap();
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

fn change_surface(
    msg: String,
    y: i64,
    width: usize,
    surface: &mut Surface,
) {
    let mut data = msg.trim().split("\n");
    let offset = y as usize * width * 3;
    for x in 0..width {
        let mut line = data.next().unwrap().split(" ");
        let r = line.next().unwrap().trim().parse::<u8>().unwrap();
        let g = line.next().unwrap().trim().parse::<u8>().unwrap();
        let b = line.next().unwrap().trim().parse::<u8>().unwrap();
        surface
            .with_lock_mut(|buffer: &mut [u8]| {
                buffer[x + offset] = r;
                buffer[x + offset + 1] = g;
                buffer[x + offset + 2] = b;
            });
    }
}
