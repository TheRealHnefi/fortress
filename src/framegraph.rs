use vertexobject::*;
use cgmath::Matrix4;

pub struct Framegraph<'a> {
  pub children: Vec<Framegraph<'a>>,

  pub transform: Matrix4<f32>,
  pub vertices: Option<&'a VertexObject>,
}
