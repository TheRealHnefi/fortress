use framegraph::Framegraph;
use resources::Resources;
use block::Block;
use volume::Volume;

pub struct World<'a> {
  terrain: Volume<Block<'a>>,
}

impl<'a> World<'a> {
  pub fn new(resources: &'a Resources) -> World<'a>
  {
    let template_block = Block::new(resources);
    World {
      terrain: Volume::<Block>::new(template_block, 10, 10, 10)
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
    
    self.terrain.at(0, 0, 0).get_framegraph()
  }
}
