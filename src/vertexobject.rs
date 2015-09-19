use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
  pub pos: [f32; 3],
}

implement_vertex!(Vertex, pos);

pub struct VertexObject {
  pub vertices: glium::VertexBuffer<Vertex>,
  pub indices: glium::index::NoIndices,
}

impl VertexObject {
  pub fn new(display: & glium::Display, shape: Vec<Vertex>) -> VertexObject
  {
    VertexObject {
      vertices: glium::VertexBuffer::new(display, &shape).unwrap(),
      indices: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
    }
  }
}
