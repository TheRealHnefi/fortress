use glium;
use vertexobject;


pub struct Framegraph<'a> {
  pub children: Vec<Framegraph<'a>>,

  pub transform: [[f32; 4]; 4],
  pub vertices: Option<&'a glium::VertexBuffer<vertexobject::Vertex>>,
  pub indices: Option<&'a glium::index::NoIndices>,
}