use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
  pub pos: [f32; 3],
}

implement_vertex!(Vertex, pos);

#[derive(Default)]
pub struct Framegraph<'a> {
  pub placeholder : i32,
  pub vertices : Option<&'a glium::VertexBuffer<Vertex>>,
}
