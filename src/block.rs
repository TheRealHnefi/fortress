use vertexobject::VertexObject;
use resources::Resources;
use framegraph::Framegraph;
use cgmath::{Matrix4,Vector3};

#[derive(Clone)]
pub struct Block<'a> {
  shape: &'a VertexObject,
}

impl<'a> Block<'a> {
  pub fn new(resources: &'a Resources) -> Block<'a>
  {
    Block { shape: &resources.simple_box }
  }

  pub fn get_framegraph(&self) -> Framegraph
  {
    let placeholder_pos = Vector3::<f32>::new(0.0, 0.0, -0.6);
    
    Framegraph {
      children: vec![],
      transform: Matrix4::<f32>::from_translation(&placeholder_pos),
      vertices: Some(&self.shape),
    }
  }
}
