use world::World;
use framegraph::Framegraph;
use resources::Resources;
use cgmath::{Matrix4,Vector3};

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
    Framegraph {
      children: vec![self.world.get_framegraph(0,0,0,10,10,3)],
      transform: Matrix4::<f32>::from_translation(&Vector3::<f32>::new(-5.0, 5.0, -10.0)),
      vertices: None,
    }
  }

  pub fn tick(&mut self, ms: u64) -> ()
  {
    self.world.tick(ms);
  }
}
