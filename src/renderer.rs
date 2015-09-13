use framegraph;
use glium::{self, Display, Surface};

pub struct Renderer {
  default_shader: glium::Program,
  clear_color: [f32; 4],
}

impl Renderer {
  pub fn new(display: & Display) -> Renderer
  {
    let vertex_src = "
    #version 140

    in vec3 pos;
    uniform mat4 matrix;
    
    void main() {
        vec3 p = pos;
        gl_Position = matrix * vec4(p, 1.0);
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
  
  pub fn render(&self, mut frame: glium::Frame, graph: framegraph::Framegraph) -> ()
  {
    frame.clear_color(self.clear_color[0],
      self.clear_color[1],
      self.clear_color[2],
      self.clear_color[3]);
      
    self.do_render(&mut frame, graph);
    
    frame.finish().unwrap();
  }
  
  fn do_render(&self,
               mut frame: &mut glium::Frame,
               graph: framegraph::Framegraph) -> ()
  {
    
  
    let uniforms = uniform! {
      matrix: graph.transform
    };
    
    match graph.vertices {
      Some(v) =>
        frame.draw(&v.vertices,
          &v.indices,
          &self.default_shader,
          &uniforms,
          &Default::default()).unwrap(),
      None => ()
    };
  
      
    for child in graph.children {
      self.do_render(&mut frame, child);
    }
  }
}