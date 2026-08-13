use glfw::{Action, Context, Key, MouseButton, WindowEvent, GlfwReceiver};
use std::collections::HashSet;
use std::ffi::{CString};
use std::fs;
use std::mem;
use std::ptr;
use std::time::Instant;

// ============================================================
// INPUT
// ============================================================

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


// ============================================================
// WINDOW
// ============================================================

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


// ============================================================
// MESH
// ============================================================

#[derive(Clone)]
pub struct Mesh {
    vao: u32,
    vbo: u32,
    ebo: u32,

    index_count: i32,
}

impl Mesh {
    pub fn new(
        vertices: Vec<f32>,
        indices: Vec<u32>,
        vertex_size: i32,
    ) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // ---------------- VBO ----------------

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                vertex_size,
                gl::FLOAT,
                gl::FALSE,
                vertex_size * mem::size_of::<f32>() as i32,
                ptr::null(),
            );

            gl::EnableVertexAttribArray(0);

            // ---------------- EBO ----------------

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            ebo,
            index_count: indices.len() as i32,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}


// ============================================================
// SHADER
// ============================================================

pub struct Shader {
    program: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_source = fs::read_to_string(vertex_path)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to read vertex shader '{}': {}",
                    vertex_path, e
                )
            });

        let fragment_source = fs::read_to_string(fragment_path)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to read fragment shader '{}': {}",
                    fragment_path, e
                )
            });

        let vertex = Self::compile_shader(
            &vertex_source,
            gl::VERTEX_SHADER,
        );

        let fragment = Self::compile_shader(
            &fragment_source,
            gl::FRAGMENT_SHADER,
        );

        let program;

        unsafe {
            program = gl::CreateProgram();

            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);

            gl::LinkProgram(program);

            let mut success = 0;
            gl::GetProgramiv(
                program,
                gl::LINK_STATUS,
                &mut success,
            );

            if success == 0 {
                let mut len = 0;

                gl::GetProgramiv(
                    program,
                    gl::INFO_LOG_LENGTH,
                    &mut len,
                );

                let mut buffer = vec![0u8; len as usize];

                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );

                let error = String::from_utf8_lossy(&buffer);

                panic!("Shader program linking failed:\n{}", error);
            }

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }

        Self { program }
    }

    fn compile_shader(
        source: &str,
        shader_type: u32,
    ) -> u32 {
        let c_source = CString::new(source)
            .expect("Shader source contained a null byte");

        let shader;

        unsafe {
            shader = gl::CreateShader(shader_type);

            gl::ShaderSource(
                shader,
                1,
                &c_source.as_ptr(),
                ptr::null(),
            );

            gl::CompileShader(shader);

            let mut success = 0;

            gl::GetShaderiv(
                shader,
                gl::COMPILE_STATUS,
                &mut success,
            );

            if success == 0 {
                let mut len = 0;

                gl::GetShaderiv(
                    shader,
                    gl::INFO_LOG_LENGTH,
                    &mut len,
                );

                let mut buffer = vec![0u8; len as usize];

                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );

                let error = String::from_utf8_lossy(&buffer);

                panic!("Shader compilation failed:\n{}", error);
            }
        }

        shader
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    fn get_uniform_location(&self, name: &str) -> i32 {
        let c_name = CString::new(name)
            .expect("Uniform name contained a null byte");

        unsafe {
            gl::GetUniformLocation(
                self.program,
                c_name.as_ptr(),
            )
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let location = self.get_uniform_location(name);

        unsafe {
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let location = self.get_uniform_location(name);

        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_vec2(&self, name: &str, value: [f32; 2]) {
        let location = self.get_uniform_location(name);

        unsafe {
            gl::Uniform2f(
                location,
                value[0],
                value[1],
            );
        }
    }

    pub fn set_vec3(&self, name: &str, value: [f32; 3]) {
        let location = self.get_uniform_location(name);

        unsafe {
            gl::Uniform3f(
                location,
                value[0],
                value[1],
                value[2],
            );
        }
    }

    pub fn set_vec4(&self, name: &str, value: [f32; 4]) {
        let location = self.get_uniform_location(name);

        unsafe {
            gl::Uniform4f(
                location,
                value[0],
                value[1],
                value[2],
                value[3],
            );
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}