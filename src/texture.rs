use std::ffi::c_void;
use gl;
use std::path::Path;

pub enum TextureWrap {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
	ClampToBorder,
}

impl TextureWrap {
	fn to_gl(&self) -> gl::types::GLenum {
		match self {
			TextureWrap::Repeat => gl::REPEAT,
			TextureWrap::MirroredRepeat => gl::MIRRORED_REPEAT,
			TextureWrap::ClampToEdge => gl::CLAMP_TO_EDGE,
			TextureWrap::ClampToBorder => gl::CLAMP_TO_BORDER,
		}
	}
}

pub enum TextureFilter {
	Nearest,
	Linear,
	LinearMipmapLinear,
	LinearMipmapNearest,
}

impl TextureFilter {
	fn to_gl(&self) -> gl::types::GLenum {
		match self {
			TextureFilter::Nearest => gl::NEAREST,
			TextureFilter::Linear => gl::LINEAR,
			TextureFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
			TextureFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
		}
	}
}

/// This is probably the most powerful graphics rendering feature.
/// It allows post processing, caching... All sorts of things.
/// What it does is it allows you to render to a texture, instead of the screen buffer.
///
/// Example:
/// ```no_run
/// let world_fb = Framebuffer::new(app.width, app.height);
/// 
/// world_fb.bind();
/// //... do rendering...
/// world_fb.unbind();
/// world_fb.texture().bind(0); //this allows the shader to access the texture in slot 0
///
/// post_processing.draw(); //draw the postprocessing to the screen.
/// ``` 
pub struct Framebuffer {
	id: u32,
	color: Texture,
}

impl Framebuffer {
	pub fn new(width: i32, height: i32) -> Self {
		let color = TextureBuilder::new()
			.empty(width, height);

		let mut id = 0;

		unsafe {
			gl::GenFramebuffers(1, &mut id);
			gl::BindFramebuffer(gl::FRAMEBUFFER, id);

			gl::FramebufferTexture2D(
				gl::FRAMEBUFFER,
				gl::COLOR_ATTACHMENT0,
				gl::TEXTURE_2D,
				color.id,
				0,
			);

			if gl::CheckFramebufferStatus(gl::FRAMEBUFFER)
				!= gl::FRAMEBUFFER_COMPLETE
			{
				panic!("Framebuffer is incomplete");
			}

			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}

		Self { id, color }
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}
	}

	pub fn texture(&self) -> &Texture {
		&self.color
	}

	pub fn resize(&mut self, width: i32, height: i32) {
		if width <= 0 || height <= 0 { return; }

		self.color.width = width;
		self.color.height = height;

		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.color.id);

			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA8 as i32,
				width,
				height,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				std::ptr::null(),
			);

			gl::BindTexture(gl::TEXTURE_2D, 0);

			gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
			if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
				panic!("Framebuffer became incomplete after resize");
			}
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		}
	}


}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteFramebuffers(1, &self.id);
		}
	}
}

pub struct TextureBuilder {
	wrap_s: TextureWrap,
	wrap_t: TextureWrap,
	min_filter: TextureFilter,
	mag_filter: TextureFilter,
	generate_mipmaps: bool,
}

impl TextureBuilder {
	pub fn new() -> Self {
		Self {
			wrap_s: TextureWrap::Repeat,
			wrap_t: TextureWrap::Repeat,
			min_filter: TextureFilter::Nearest,
			mag_filter: TextureFilter::Nearest,
			generate_mipmaps: true,
		}
	}

	pub fn with_wrapping(mut self, s: TextureWrap, t: TextureWrap) -> Self {
		self.wrap_s = s;
		self.wrap_t = t;
		self
	}

	pub fn with_filtering(mut self, min: TextureFilter, mag: TextureFilter) -> Self {
		self.min_filter = min;
		self.mag_filter = mag;
		self
	}

	pub fn generate_mipmaps(mut self, generate: bool) -> Self {
		self.generate_mipmaps = generate;
		self
	}

	pub fn empty(&self, width: i32, height: i32) -> Texture {
		let mut id = 0;

		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);

			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_WRAP_S,
				self.wrap_s.to_gl() as i32,
			);

			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_WRAP_T,
				self.wrap_t.to_gl() as i32,
			);

			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_MIN_FILTER,
				self.min_filter.to_gl() as i32,
			);

			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_MAG_FILTER,
				self.mag_filter.to_gl() as i32,
			);

			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA8 as i32,
				width,
				height,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				std::ptr::null(),
			);

			if self.generate_mipmaps {
				gl::GenerateMipmap(gl::TEXTURE_2D);
			}

			gl::BindTexture(gl::TEXTURE_2D, 0);
		}

		Texture { id, width, height }
	}

	pub fn from_file<P: AsRef<Path>>(&self, path: P) -> Texture {
		let img = image::open(&path)
			.unwrap_or_else(|e| panic!("Failed to load texture at {:?}: {}", path.as_ref(), e));
		
		let img = img.flipv(); 
		
		let width = img.width() as i32;
		let height = img.height() as i32;

		let (internal_format, format, data) = match img.color() {
			image::ColorType::Rgb8 => {
				(
					gl::RGB as i32,
					gl::RGB,
					img.to_rgb8().into_raw(),
				)
			}

			image::ColorType::Rgba8 => {
				(
					gl::RGBA as i32,
					gl::RGBA,
					img.to_rgba8().into_raw(),
				)
			}

			_ => {
				(
					gl::RGBA as i32,
					gl::RGBA,
					img.to_rgba8().into_raw(),
				)
			}
		};

		let mut id = 0;
		unsafe {
			gl::GenTextures(1, &mut id);
			gl::BindTexture(gl::TEXTURE_2D, id);

			// Configure wrapping parameters
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s.to_gl() as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t.to_gl() as i32);

			// Configure filtering parameters
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.min_filter.to_gl() as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.mag_filter.to_gl() as i32);

			// Upload the pixel data
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				internal_format,
				width,
				height,
				0,
				format,
				gl::UNSIGNED_BYTE,
				data.as_ptr() as *const c_void,
			);

			if self.generate_mipmaps {
				gl::GenerateMipmap(gl::TEXTURE_2D);
			}
			
			// Unbind to prevent accidental modifications later
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}

		Texture { id, width, height }
	}
}

pub struct Texture {
	id: u32,
	width: i32,
	height: i32,
}

impl Texture {
	pub fn bind(&self, slot: u32) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + slot);
			gl::BindTexture(gl::TEXTURE_2D, self.id);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
	}

	pub fn width(&self) -> i32 { self.width }
	pub fn height(&self) -> i32 { self.height }
}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, &self.id);
		}
	}
}