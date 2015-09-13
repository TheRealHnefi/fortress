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
    let mtx1 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0 , 0.0, 0.0, 1.0f32],
      ];
      
    let mtx2 = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.5 , 0.5, 0.0, 1.0f32],
      ];
  
    let child = Framegraph {
      children: vec![],
      transform: mtx1,
      vertices: Some(&self.placeholder.vertices),
      indices: Some(&self.placeholder.indices),
    };
    
    Framegraph {
      children: vec![child],
      transform: mtx2,
      vertices: Some(&self.placeholder.vertices),
      indices: Some(&self.placeholder.indices),
    }
  }
}