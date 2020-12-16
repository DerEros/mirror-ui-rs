use std::time::Duration;
use sdl2::event::Event;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Could not access video subsystem in SDL2");

    let _window = video_subsystem.window("Mirror UI",  768, 1024)
        .position_centered()
        .build()
        .expect("Could not create window");

    let mut event_pump = sdl_context.event_pump().expect("Could not create event pump");
    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 10));
    }

}
