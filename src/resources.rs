use vertexobject::{Vertex, VertexObject};
use glium;

pub struct Resources {
  pub simple_box: VertexObject,
}

impl Resources {
  pub fn new(display: & glium::Display) -> Resources
  {
    let v1 = Vertex { pos: [-0.5, -0.5, 0.0] };
    let v2 = Vertex { pos: [0.5, -0.5, 0.0] };
    let v3 = Vertex { pos: [0.5, 0.5, 0.0] };
    let v4 = Vertex { pos: [-0.5, 0.5, 0.0] };
    let box_shape = vec![v1, v2, v3, v4];
    
    Resources {
      simple_box: VertexObject::new(display, box_shape)
    }
  }
}
