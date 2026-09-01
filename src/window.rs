use std::time::Instant;
use glfw::{Action, Context, Key, MouseButton, WindowEvent, GlfwReceiver};
use std::collections::HashSet;

pub struct Input {
    keys_held: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,

    mouse_held: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    pub mouse_position: (f64, f64),
}

impl Input {
    fn new() -> Self {
        Self {
            keys_held: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),

            mouse_held: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),

            mouse_position: (0.0, 0.0),
        }
    }

    fn begin_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();

        self.mouse_pressed.clear();
        self.mouse_released.clear();
    }

    pub fn is_key_held(&self, key: Key) -> bool {
        self.keys_held.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_held(&self, button: MouseButton) -> bool {
        self.mouse_held.contains(&button)
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }
}

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: glfw::PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,

    pub width: i32,
    pub height: i32,

    pub input: Input,

    start_time: Instant,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors)
            .expect("Failed to initialize GLFW");

        let (mut window, events) = glfw
            .create_window(
                width,
                height,
                title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window");

        window.make_current();

        // Input events
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_close_polling(true);

        // Load OpenGL
        gl::load_with(|symbol| {
            window.get_proc_address(symbol)
        });

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        Self {
            glfw,
            window,
            events,

            width: width as i32,
            height: height as i32,

            input: Input::new(),

            start_time: Instant::now(),
        }
    }

    pub fn running(&self) -> bool {
        !self.window.should_close()
    }

    pub fn poll_events(&mut self) {
        self.input.begin_frame();

        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    match action {
                        Action::Press => {
                            self.input.keys_held.insert(key);
                            self.input.keys_pressed.insert(key);
                        }

                        Action::Repeat => {
                            self.input.keys_held.insert(key);
                        }

                        Action::Release => {
                            self.input.keys_held.remove(&key);
                            self.input.keys_released.insert(key);
                        }
                    }

                    // Escape closes the window
                    if key == Key::Escape && action == Action::Press {
                        self.window.set_should_close(true);
                    }
                }

                WindowEvent::MouseButton(button, action, _) => {
                    match action {
                        Action::Press => {
                            self.input.mouse_held.insert(button);
                            self.input.mouse_pressed.insert(button);
                        }

                        Action::Release => {
                            self.input.mouse_held.remove(&button);
                            self.input.mouse_released.insert(button);
                        }

                        Action::Repeat => {}
                    }
                }

                WindowEvent::CursorPos(x, y) => {
                    self.input.mouse_position = (x, y);
                }

                WindowEvent::FramebufferSize(width, height) => {
                    self.width = width;
                    self.height = height;

                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }

                WindowEvent::Close => {
                    self.window.set_should_close(true);
                }

                _ => {}
            }
        }
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn clear_with_color(&self, r: f32, g: f32, b: f32) {
        unsafe {
            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}
