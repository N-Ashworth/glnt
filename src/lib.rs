//! ## gl_nt
//! or gl, no tricks, is a graphics library intended
//! to reduce the boilerplate of the glfw and gl
//! windowing and graphics libraries, respectively.

#![allow(dead_code)]
mod window;
pub use crate::window::{Input, Window};

mod mesh;
pub use crate::mesh::Mesh;

mod shader;
pub use crate::shader::{ShaderBuilder, Shader};

mod texture;
pub use crate::texture::{TextureWrap, TextureFilter, Framebuffer, TextureBuilder, Texture};