extern crate glium;

use glium::Surface;

pub struct Renderer {
  clear_color : [f32; 4],
}

impl Renderer {
  pub fn new() -> Renderer
  {
    Renderer {
      clear_color: [0.5, 0.5, 0.0, 1.0]
    }
  }
  
  pub fn render(&self, mut frame : glium::Frame) -> ()
  {
    frame.clear_color(self.clear_color[0],
      self.clear_color[1],
      self.clear_color[2],
      self.clear_color[3]);
    frame.finish().unwrap();
  }
}