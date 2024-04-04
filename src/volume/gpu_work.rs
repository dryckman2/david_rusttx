use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

pub struct WinSdl {
    pub sdl: Sdl,
    pub window: Window,
    pub event_pump: EventPump,
}

impl WinSdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window("Testing Window", width as u32, height as u32)
            .build()?;
        let event_pump: EventPump = sdl.event_pump()?;

        Ok(WinSdl {
            sdl,
            window,
            event_pump,
        })
    }
}

pub fn testing_gpu() -> Result<(), &'static str> {
    let mut winsdl = WinSdl::new(800, 600)?;
    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
    Ok(())
}
