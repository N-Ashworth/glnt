use std::ptr;
use std::mem;

#[derive(Clone)]
pub struct Mesh {
	vao: u32,
	vbo: u32,
	ebo: u32,

	attr_vbos: Vec<u32>,

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
			attr_vbos: vec![],
			index_count: indices.len() as i32,
		}
	}

	pub fn quad(mpos: Vec<f32>, width: f32, height: f32) -> Self {
		let mut vs = vec![];
		let mut pos = mpos.clone();
		
		vs.extend(&pos);

		pos[0] += width;
		vs.extend(&pos);

		pos[1] += height;
		vs.extend(&pos);

		pos[0] -= width;
		vs.extend(&pos);

		let is = vec![0, 1, 3, 3, 1, 2];

		Self::new(vs, is, pos.len() as i32)
	}

	pub fn attr_vec2(&mut self, loc: u32, data: Vec<[f32; 2]>) {
		let mut vbo = 0;

		unsafe {
			gl::BindVertexArray(self.vao);

			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(data.len() * mem::size_of::<[f32; 2]>()) as isize,
				data.as_ptr() as *const _,
				gl::STATIC_DRAW,
			);

			gl::VertexAttribPointer(
				loc,
				2,
				gl::FLOAT,
				gl::FALSE,
				2 * mem::size_of::<f32>() as i32,
				ptr::null(),
			);

			gl::EnableVertexAttribArray(loc);

			gl::BindVertexArray(0);
		}

		self.attr_vbos.push(vbo);
	}

	pub fn attr_float(&mut self, loc: u32, data: Vec<f32>) {
		let mut vbo = 0;

		unsafe {
			gl::BindVertexArray(self.vao);

			gl::GenBuffers(1, &mut vbo);
			gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

			gl::BufferData(
				gl::ARRAY_BUFFER,
				(data.len() * mem::size_of::<f32>()) as isize,
				data.as_ptr() as *const _,
				gl::STATIC_DRAW,
			);

			gl::VertexAttribPointer(
				loc,
				1,
				gl::FLOAT,
				gl::FALSE,
				mem::size_of::<f32>() as i32,
				ptr::null(),
			);

			gl::EnableVertexAttribArray(loc);

			gl::BindVertexArray(0);
		}

		self.attr_vbos.push(vbo);
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

			for vbo in &self.attr_vbos {
				gl::DeleteBuffers(1, vbo);
			}
		}
	}
}