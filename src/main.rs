extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  let (mut window, events) =
    glfw.create_window(WIDTH, HEIGHT, "Hello, this is dawg", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

  window.make_current();
  window.set_key_polling(true);

  while !window.should_close() {
    window.swap_buffers();

    glfw.poll_events();

    for (_, event) in glfw::flush_messages(&events) {
      println!("{:?}", event);

      match event {
        glfw::WindowEvent::Key(key, _, Action::Press, _) if key == Key::Escape || key == Key::Space => {
          window.set_should_close(true)
        },
        _ => {}
      }
    }
  }
}
