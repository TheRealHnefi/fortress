use vertexobject::VertexObject;
use framegraph::Framegraph;
use resources::Resources;

pub struct World<'a> {
  placeholder: &'a VertexObject,
}

impl<'a> World<'a> {
  pub fn new(resources: &'a Resources) -> World<'a>
  {
    World {placeholder: &resources.simple_box}
  }
  
  pub fn get_framegraph(&self) -> Framegraph
  {
    Framegraph {
      placeholder: 0,
      vertices: Some(&self.placeholder.vertices),
    }
  }
}