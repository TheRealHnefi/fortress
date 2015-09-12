use world;
use framegraph;
use glium::Display;

pub struct Session {
  world : world::World,
}

impl Session {
  pub fn new(display: & Display) -> Session
  {
    Session {
      world: world::World::new(display, 0, 0, 0)
    }
  }
  
  pub fn get_framegraph(&self) -> framegraph::Framegraph
  {
    self.world.get_framegraph()
  }
}