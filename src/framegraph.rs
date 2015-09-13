use glium;
use vertexobject;


pub struct Framegraph<'a> {
  pub vertices: Option<&'a glium::VertexBuffer<vertexobject::Vertex>>,
  pub indices: Option<&'a glium::index::NoIndices>,
}