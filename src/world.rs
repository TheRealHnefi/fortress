use framegraph::Framegraph;
use resources::Resources;
use block::Block;
use volume::Volume;
use cgmath::Matrix4;

pub struct World<'a> {
  terrain: Volume<Block<'a>>,
}

impl<'a> World<'a> {
  pub fn new(resources: &'a Resources) -> World<'a>
  {
    let template_block = Block::new(resources, 0, 0, 0);
    let mut proto_terrain = Volume::<Block>::new(template_block, 10, 10, 10);

    for x in 0..10 {
      for y in 0..10 {
        for z in 0..10 {
          proto_terrain.at_mut(x, y, z).x = x;
          proto_terrain.at_mut(x, y, z).y = y;
          proto_terrain.at_mut(x, y, z).z = z;
        }
      }
    }
    
    World {
      terrain: proto_terrain
    }
  }
  
  pub fn get_framegraph(&self,
                        min_x: usize,
                        min_y: usize,
                        min_z: usize,
                        max_x: usize,
                        max_y: usize,
                        max_z: usize) -> Framegraph
  {
    let _max_x = if max_x >= self.terrain.size_x {
      self.terrain.size_x - 1
    } else {
      max_x
    };

    let _max_y = if max_y >= self.terrain.size_y {
      self.terrain.size_y - 1
    } else {
      max_y
    };

    let _max_z = if max_z >= self.terrain.size_z {
      self.terrain.size_z - 1
    } else {
      max_z
    };

    let mut terrain_graphs =
      Vec::<Framegraph>::with_capacity((_max_x - min_x)
                                       * (_max_y - min_y)
                                       * (_max_z - min_z));

    for x in min_x.._max_x {
      for y in min_y.._max_y {
        for z in min_z.._max_z {
          terrain_graphs.push(self.terrain.at(x, y, z).get_framegraph());
        }
      }
    }
    
    Framegraph {
      children: terrain_graphs,
      transform: Matrix4::<f32>::identity(),
      vertices: None,
    }
  }
}
