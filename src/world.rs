use vertexobject::VertexObject;
use framegraph::Framegraph;
use resources::Resources;
use block::Block;
use volume::Volume;

pub struct World<'a> {
  placeholder: &'a VertexObject,
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
  
  pub fn get_framegraph(&self) -> Framegraph
  {
    self.terrain.at(0, 0, 0).get_framegraph()
  }
}
