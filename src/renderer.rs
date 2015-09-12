use framegraph;
use glium::{self, Display, Surface};

pub struct Renderer {
  default_shader : glium::Program,
  clear_color : [f32; 4],
}

impl Renderer {
  pub fn new(display: & Display) -> Renderer
  {
    let vertex_src = "
    #version 140

    in vec3 pos;
    
    void main() {
        vec3 position = pos;
        gl_Position = vec4(position, 1.0);
    }
";

    let frag_src = "
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
";
  
    Renderer {
      default_shader: glium::Program::from_source(display, vertex_src, frag_src, None).unwrap(),
      clear_color: [0.5, 0.5, 0.0, 1.0]
    }
  }
  
  pub fn render(&self, mut frame : glium::Frame, graph: framegraph::Framegraph) -> ()
  {
    frame.clear_color(self.clear_color[0],
      self.clear_color[1],
      self.clear_color[2],
      self.clear_color[3]);
      
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    frame.draw(graph.vertices.unwrap(),
      &indices,
      &self.default_shader,
      &glium::uniforms::EmptyUniforms,
      &Default::default()).unwrap();
    frame.finish().unwrap();
  }
}