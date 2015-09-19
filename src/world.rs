use vertexobject::VertexObject;
use framegraph::Framegraph;
use resources::Resources;
use block::Block;

struct Volume<T> {
  data: Vec<T>,
  size_x: usize,
  size_y: usize,
  size_z: usize
}

impl<T: Clone> Volume<T> {
  pub fn new(value: T, x: usize, y: usize, z: usize) -> Volume<T> {
    let mut vector = Vec::with_capacity(x * y * z);
    for _ in 0..x*y*z {
      vector.push(value.clone());
    }
    Volume { data: vector, size_x: x, size_y: y, size_z: z }
  }

  pub fn at(&self, x: usize, y: usize, z: usize) -> &T {
    unsafe {
      let retval = self.data.get_unchecked((x + self.size_x * y + self.size_x * self.size_y * z) as usize);
      retval
    }
  }
  
  pub fn at_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
    unsafe {
      let retval = self.data.get_unchecked_mut((x + self.size_x * y + self.size_x * self.size_y * z) as usize);
      retval
    }
  }
}

pub struct World<'a> {
  placeholder: &'a VertexObject,
  terrain: Volume<Block<'a>>,
}

impl<'a> World<'a> {
  pub fn new(resources: &'a Resources) -> World<'a>
  {
    let template_block = Block::new(resources);
    World {
      placeholder: &resources.simple_box,
      terrain: Volume::<Block>::new(template_block, 10, 10, 10)
    }
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
      transform: mtx2,
      vertices: Some(&self.placeholder),
    };
    
    Framegraph {
      children: vec![child],
      transform: mtx1,
      vertices: Some(&self.placeholder),
    }
  }
}
