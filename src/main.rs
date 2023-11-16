use gl33;
use beryllium::*;
use glfw::{fail_on_errors,Action,Context,Key};
fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut windows, events) = 
        glfw.create_window(1280, 700, "Hello, World!", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW Window");

    windows.make_current();
    windows.set_key_polling(true);

    while !windows.should_close() {
        windows.swap_buffers();

        glfw.poll_events();
        for (_, events) in glfw::flush_messages(&events) {
            println!("{:?}", events);
            match events {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    windows.set_should_close(true);
                },
                _ => {},
            }
        }
    }


}
