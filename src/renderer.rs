

/*
    The purpose of this crate is to wrap OpenGL so we don't need to directly use GL calls,
    and we can avoid a lot of unsafe tags all around our code by putting them all in here

    Ideally we can interact with this wrapper using normal Rust types, for example passing
    slices to functions that take arrays, rather than raw pointers with sizes.

    Functionality: */
    /*TODO
        * Create a window and context using GLFW
        * Load OpenGL using the gl crate
        * Allow management of resources:
            * Shaders / Programs
            * VBOs
            * Buffers
        * Finish this todo list
*/

extern crate gl;
extern crate glm;
extern crate glfw;

use glfw::{Glfw, Window, WindowEvent, Key, Action, Context};
use std::sync::mpsc::Receiver;

pub struct Renderer {
    glfw : Glfw,
    window : Window,
    events : Receiver<(f64, WindowEvent)>,
}



impl Renderer {
    pub fn initialize(width: u32, height: u32, title: &str) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).expect("Failed to create window");

        window.set_key_polling(true);
        window.make_current();
    
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        Self {glfw: glfw, window: window, events : events}
    }

    // This function should deallocate all OpenGL related resources,
    // which can't clean themselves up because they come from FFI
    pub fn shutdown(&mut self) {
    }

    pub fn update(&mut self) -> bool {
        self.window.swap_buffers();

        // TODO: Don't handle input in the renderer update function, it's stupid af
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }

        !self.window.should_close()
    }
}
