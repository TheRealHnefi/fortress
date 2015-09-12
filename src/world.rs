use framegraph;
use glium;

pub struct World {
  pub size_x : i32,
  pub size_y : i32,
  pub size_z : i32,
  placeholder : glium::VertexBuffer<framegraph::Vertex>,
}

impl World {
  pub fn new(display: & glium::Display, x: i32, y: i32, z: i32) -> World
  {
    let v1 = framegraph::Vertex { pos: [-0.5, -0.5, 0.0] };
    let v2 = framegraph::Vertex { pos: [0.0, 0.5, 0.0] };
    let v3 = framegraph::Vertex { pos: [0.5, -0.25, 0.0] };
    let shape = vec![v1, v2, v3];
    let vb = glium::VertexBuffer::new(display, &shape).unwrap();
    World {size_x: 0, size_y: 0, size_z: 0, placeholder: vb}
  }
  
  pub fn get_framegraph(&self) -> framegraph::Framegraph
  {
    framegraph::Framegraph{placeholder: 0, vertices: Some(&self.placeholder)}
  }
}