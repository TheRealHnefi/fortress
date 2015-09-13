use vertexobject;
use framegraph;
use glium;

pub struct World {
  placeholder: glium::VertexBuffer<vertexobject::Vertex>,
}

impl World {
  pub fn new(display: & glium::Display) -> World
  {
    let v1 = vertexobject::Vertex { pos: [-0.5, -0.5, 0.0] };
    let v2 = vertexobject::Vertex { pos: [0.0, 0.5, 0.0] };
    let v3 = vertexobject::Vertex { pos: [0.5, -0.25, 0.0] };
    let shape = vec![v1, v2, v3];
    let vb = glium::VertexBuffer::new(display, &shape).unwrap();
    World {placeholder: vb}
  }
  
  pub fn get_framegraph(&self) -> framegraph::Framegraph
  {
    framegraph::Framegraph {
      placeholder: 0,
      vertices: Some(&self.placeholder),
    }
  }
}