use std::ffi::{CString};
use std::fs;
use std::ptr;

/// A builder pattern that compiles and links the shader stuff.
/// 
/// Only the actual non-default shaders are necessary to be linked.
///
/// For instance, only a vertex shader and a fragment shader is ok, as GL uses the default geometry shader:
/// ```no_run
/// use gl_nt::ShaderBuilder; 
///
/// let mut shader = ShaderBuilder::new()
/// 	.vertex_from_file("assets/shader.vert")
/// 	.fragment_from_file("assets/shader.frag").build();
///
/// //...
/// ```
/// 
/// Note: getting the shader from a source string is coming soon but it isn't here yet (I'm busy making documentation)
pub struct ShaderBuilder {
	vertex: Option<String>,
	geom: Option<String>,
	frag: Option<String>,
}

impl ShaderBuilder {
	pub fn new() -> Self {
		Self {
			vertex: None,
			geom: None,
			frag: None,
		}
	}

	/// Gets the vertex shader from a file path. The path is relative to the project directory, not src/ (this tripped me up, but I suppose most have gotten used to it)
	pub fn vertex_from_file(&self, path: &str) -> Self {
		let vertex_source = fs::read_to_string(path)
			.unwrap_or_else(|e| {
				panic!(
					"Failed to read vertex shader '{}': {}",
					path, e
				)
			});

		Self {
			vertex: Some(vertex_source),
			geom: self.geom.clone(),
			frag: self.frag.clone(),
		}
	}

	/// Gets the geometry shader from a file path. The path is relative to the project directory, not src/ (this tripped me up, but I suppose most have gotten used to it)
	pub fn geometry_from_file(&self, path: &str) -> Self {
		let geom_source = fs::read_to_string(path)
			.unwrap_or_else(|e| {
				panic!(
					"Failed to read geometry file '{}': {}",
					path, e
				)
			});

		Self {
			vertex: self.vertex.clone(),
			geom: Some(geom_source),
			frag: self.frag.clone(),
		}
	}

	/// Gets the fragment shader from a file path. The path is relative to the project directory, not src/ (this tripped me up, but I suppose most have gotten used to it)
	pub fn fragment_from_file(&self, path: &str) -> Self {
		let frag_source = fs::read_to_string(path)
			.unwrap_or_else(|e| {
				panic!(
					"Failed to read fragment file '{}': {}",
					path, e
				)
			});

		Self {
			vertex: self.vertex.clone(),
			geom: self.geom.clone(),
			frag: Some(frag_source),
		}
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

	fn complain_about_shader(program: u32) {
		unsafe {
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
	}

	fn create_program(v_shader: Option<u32>, g_shader: Option<u32>, f_shader: Option<u32>) -> u32 { unsafe {
		let program = gl::CreateProgram();

		if let Some(shader) = v_shader {
			gl::AttachShader(program, shader);
		}

		if let Some(shader) = g_shader {
			gl::AttachShader(program, shader);
		}

		if let Some(shader) = f_shader {
			gl::AttachShader(program, shader);
		}

		gl::LinkProgram(program);

		let mut success = 0;
		gl::GetProgramiv(
			program,
			gl::LINK_STATUS,
			&mut success,
		);

		if success == 0 {
			Self::complain_about_shader(program);
		}

		if let Some(shader) = v_shader {
			gl::DeleteShader(shader);
		}

		if let Some(shader) = g_shader {
			gl::DeleteShader(shader);
		}

		if let Some(shader) = f_shader {
			gl::DeleteShader(shader);
		}

		program
	}}

	/// Remember to put this at the end of the building process!! It turns the `ShaderBuilder` into the `Shader` struct.
	pub fn build(&self) -> Shader {
		let mut v_shader = None;
		let mut g_shader = None;
		let mut f_shader = None;

		if let Some(v_src) = &self.vertex {
			v_shader = Some(Self::compile_shader(&v_src, gl::VERTEX_SHADER));
		}

		if let Some(g_src) = &self.geom {
			g_shader = Some(Self::compile_shader(&g_src, gl::GEOMETRY_SHADER));
		}

		if let Some(f_src) = &self.frag {
			f_shader = Some(Self::compile_shader(&f_src, gl::FRAGMENT_SHADER));
		}

		Shader {
			program: Self::create_program(v_shader, g_shader, f_shader),
		}
	}
}

///
pub struct Shader {
	program: u32,
}

impl Shader {
	/// This binds the shader to whatever draw calls are going to happen next.
	///
	/// **The shader stays bound!!**
	///
	/// ```no_run
	/// shader.bind();
	/// mesh1.draw();
	///
	/// mesh2.draw(); //this draws with the shader active still!
	/// ```
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