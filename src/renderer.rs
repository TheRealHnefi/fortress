use framegraph;
use glium::{self, Display, Surface};
use cgmath::{self, Matrix4};

pub struct Renderer {
  default_shader: glium::Program,
  clear_color: [f32; 4],
  projection: Matrix4<f32>,
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

    let (view_x, view_y) = match display.get_window() {
      Some(w) => w.get_inner_size_pixels().unwrap_or((1,1)),
      None => (1, 1)
    };

    let ratio: f32 = 0.5 * view_y as f32 / view_x as f32;

    Renderer {
      default_shader: glium::Program::from_source(display, vertex_src, frag_src, None).unwrap(),
      clear_color: [0.0, 0.0, 0.0, 1.0],
      projection: cgmath::frustum::<f32>(-0.5, 0.5, -ratio, ratio, 0.5, 100.0)
    }
  }
  
  pub fn render(&self, mut frame: glium::Frame, graph: framegraph::Framegraph) -> ()
  {
    frame.clear_color_and_depth((self.clear_color[0],
                                 self.clear_color[1],
                                 self.clear_color[2],
                                 self.clear_color[3]),
                                1.0);
      
    self.do_render(&mut frame, graph, Matrix4::identity());
    
    frame.finish().unwrap();
  }
  
  fn do_render(&self,
               mut frame: &mut glium::Frame,
               graph: framegraph::Framegraph,
               base_transform: Matrix4<f32>) -> ()
  {
    let new_transform = base_transform * graph.transform;
      
    let uniforms = uniform! {
      matrix: self.projection * new_transform
    };

    let params = glium::DrawParameters {
      depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        .. Default::default()
      },
      blend: glium::Blend::alpha_blending(),
      .. Default::default()
    };
    
    match graph.vertices {
      Some(v) =>
        frame.draw(&v.vertices,
          &v.indices,
          &self.default_shader,
          &uniforms,
          &params).unwrap(),
      None => ()
    };
  
    for child in graph.children {
      self.do_render(&mut frame, child, new_transform);
    }
  }
}
