use std::ffi::{CString};
use std::fs;
use std::ptr;

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