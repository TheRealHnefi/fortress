use glium;
use vertexobject;


pub struct Framegraph<'a> {
  pub placeholder: i32,
  pub vertices: Option<&'a glium::VertexBuffer<vertexobject::Vertex>>,
}
