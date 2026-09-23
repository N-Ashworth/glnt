## GL_NT

GL_NT (pronounced 'glint') is a small graphics library I made because
most GPU apis (Even OpenGL, widely regarded as the easiest) are incredibly,
unreasonably, unnecessarily verbose. For 99% of use cases, you do **not need
to manipulate the boilerplate order of calls!!!!!!**

GL_NT is for that 99%.
The code to render a triangle:
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