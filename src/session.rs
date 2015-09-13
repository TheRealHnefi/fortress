use world::World;
use framegraph::Framegraph;
use resources::Resources;

pub struct Session<'a> {
  world: World<'a>,
}

impl<'a> Session<'a> {
  pub fn new(resources: &'a Resources) -> Session<'a>
  {
    Session {
      world: World::new(resources)
    }
  }
  
  pub fn get_framegraph(&self) -> Framegraph
  {
    self.world.get_framegraph()
  }
}