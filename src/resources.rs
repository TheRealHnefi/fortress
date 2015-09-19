use vertexobject::{Vertex, VertexObject};
use glium;

pub struct Resources {
  pub simple_box: VertexObject,
}

impl Resources {
  pub fn new(display: & glium::Display) -> Resources
  {
    let box_shape = vec![
      Vertex { pos: [-0.5, -0.5, -0.5] },
      Vertex { pos: [-0.5, 0.5, -0.5] },
      Vertex { pos: [0.5, 0.5, -0.5] },

      Vertex { pos: [0.5, -0.5, -0.5] },
      Vertex { pos: [-0.5, -0.5, -0.5] },                  
      Vertex { pos: [0.5, 0.5, -0.5] },

      Vertex { pos: [0.5, -0.5, 0.5] },
      Vertex { pos: [-0.5, -0.5, 0.5] },
      Vertex { pos: [0.5, 0.5, 0.5] },

      Vertex { pos: [-0.5, -0.5, 0.5] },
      Vertex { pos: [-0.5, 0.5, 0.5] },
      Vertex { pos: [0.5, 0.5, 0.5] },
      
      Vertex { pos: [-0.5, -0.5, -0.5] },
      Vertex { pos: [-0.5, 0.5, -0.5] },
      Vertex { pos: [-0.5, 0.5, 0.5] },

      Vertex { pos: [-0.5, -0.5, -0.5] },
      Vertex { pos: [-0.5, 0.5, 0.5] },
      Vertex { pos: [-0.5, -0.5, 0.5] },

      Vertex { pos: [0.5, -0.5, -0.5] },
      Vertex { pos: [0.5, 0.5, 0.5] },
      Vertex { pos: [0.5, 0.5, -0.5] },

      Vertex { pos: [0.5, -0.5, -0.5] },
      Vertex { pos: [0.5, -0.5, 0.5] },
      Vertex { pos: [0.5, 0.5, 0.5] },

      Vertex { pos: [-0.5, -0.5, -0.5] },
      Vertex { pos: [-0.5, -0.5, 0.5] },
      Vertex { pos: [0.5, -0.5, -0.5] },

      Vertex { pos: [0.5, -0.5, -0.5] },
      Vertex { pos: [-0.5, -0.5, 0.5] },
      Vertex { pos: [0.5, -0.5, 0.5] },
      
      Vertex { pos: [0.5, 0.5, -0.5] },
      Vertex { pos: [-0.5, 0.5, 0.5] },
      Vertex { pos: [-0.5, 0.5, -0.5] },

      Vertex { pos: [0.5, 0.5, -0.5] },
      Vertex { pos: [0.5, 0.5, 0.5] },
      Vertex { pos: [-0.5, 0.5, 0.5] },
      ];
    
    Resources {
      simple_box: VertexObject::new(display, box_shape)
    }
  }
}
