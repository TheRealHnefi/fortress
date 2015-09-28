use vertexobject::VertexObject;
use resources::Resources;
use framegraph::Framegraph;
use cgmath::{Matrix4,Vector3};

#[derive(Clone)]
pub struct Block<'a> {
  shape: &'a VertexObject,
  pub x: usize,
  pub y: usize,
  pub z: usize,
  pub placeholder: u64,
}

impl<'a> Block<'a> {
  pub fn new(resources: &'a Resources, pos_x: usize, pos_y: usize, pos_z: usize) -> Block<'a>
  {
    Block {
      shape: &resources.simple_box,
      x: pos_x,
      y: pos_y,
      z: pos_z,
      placeholder: 0,
    }
  }

  pub fn get_framegraph(&self) -> Framegraph
  {
    let placeholder_pos = Vector3::<f32>::new(self.x as f32, -(self.y as f32), -(self.z as f32));
    
    Framegraph {
      children: vec![],
      transform: Matrix4::<f32>::from_translation(&placeholder_pos),
      vertices: if self.placeholder < 500 {Some(&self.shape)} else {None},
    }
  }
}
