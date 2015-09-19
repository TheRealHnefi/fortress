use vertexobject::VertexObject;
use resources::Resources;
use framegraph::Framegraph;

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
    let unit = [
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0 , 0.0, 0.0, 1.0f32],
      ];
    Framegraph {
      children: vec![],
      transform: unit,
      vertices: Some(&self.shape),
    }
  }
  
}
