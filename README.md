# GL_NT

GL_NT (pronounced 'glint') is a small graphics library I made because
most GPU apis (Even OpenGL, widely regarded as the easiest) are incredibly,
unreasonably, unnecessarily verbose. For 99% of use cases, you do **not need
to manipulate the boilerplate order of calls!!!!!!**

On the other side of the spectrum, we have the game engines. These are OK, but I like
having *total control* over what I make, including my rendering.

GL_NT is for those that want to control all of their rendering process while working quickly
without manipulating complex APIs.

Basic code to open a window and render a triangle:
```
use gl_nt::{Window, Mesh};

fn main() {
	let tri = Mesh::new(vec![
		0.0, 0.0, 0.0,
		0.0, 1.0, 0.0,
		1.0, 0.0, 0.0,
		], vec![0, 1, 2], 3); //vertex positions, indices, number of dimensions. Works well w things like tobj

	let mut app = Window::new(800, 600, "hello"); //width, height, title

	while app.running() {
		app.poll_events();

		tri.draw();

		app.swap_buffers();
	}
}
```

### Installation
To install, use `cargo add gl_nt` or add `gl_nt = "1.0.0"` under `[dependencies]` in your cargo.toml.

## Warning!
Keep in mind! This library is nice, but it is also important to know how the GPU APIs work. This is just meant
to make it more approachable and easier to learn, and if you already know, act as a massive time-saver.

This is also, currently, **my hobby project** (until people start improving on it). I am not claiming ownership over it. What I mean is that I will only add what **I** need
to, as I make my game. The library is open-source on github, so if you need to make any changes, do it there.