use world::*;
use framegraph::*;
use glium::Display;

pub struct Session {
  world: World,
}

impl Session {
  pub fn new(display: & Display) -> Session
  {
    Session {
      world: World::new(display)
    }
  }
  
  pub fn get_framegraph(&self) -> Framegraph
  {
    self.world.get_framegraph()
  }
}