use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("Could not initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Could not access video subsystem in SDL2");

    let _window = video_subsystem.window("Mirror UI",  768, 1024)
        .position_centered()
        .build()
        .expect("Could not create window");

    ::std::thread::sleep(Duration::new(10, 0));
}
