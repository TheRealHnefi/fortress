use framegraph::Framegraph;
use resources::Resources;
use block::Block;
use volume::Volume;
use cgmath::Matrix4;
use std::thread;
use std::sync::mpsc;

pub struct World<'a> {
  terrain: Volume<Block<'a>>,
  ticks: u32,
  tick_time: u64,
  main_thread: thread::JoinHandle<()>,
  done_sender: mpsc::Sender<bool>,
}

impl<'a> Drop for World<'a> {
  fn drop(&mut self) {
    match self.done_sender.send(true) {
      Ok(_) => (),
      Err(_) => println!("Error closing main thread from World::drop"),
    }
  }
}

impl<'a> World<'a> {
  fn run(rx: mpsc::Receiver<bool>) -> ()
  {
    println!("Starting thread");
    let mut done = false;
    while !done {
      match rx.try_recv() {
        Ok(v) => done = v,
        Err(_) => ()
      }
    };
    println!("Stopping thread");
  }
  
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

    let (tx, rx) = mpsc::channel();
    
    let thread = thread::spawn(move || {World::run(rx)});

    World {
      terrain: proto_terrain,
      ticks: 0,
      tick_time: 0,
      main_thread: thread,
      done_sender: tx
    }
  }

  pub fn tick(&mut self, ms: u64) -> ()
  {
    self.main_thread.thread().unpark();
      
    self.ticks += 1;
    self.tick_time += ms;
    if self.tick_time >= 1000 {
      println!("Tick rate = {}/s", self.ticks);
      self.ticks = 0;
      self.tick_time = 0;
    }
    
    // This is enormously temporary
    for x in 0..self.terrain.size_x {
      for y in 0..self.terrain.size_y {
        for z in 0..self.terrain.size_z {
          self.terrain.at_mut(x, y, z).placeholder += 1;
          if self.terrain.at(x, y, z).placeholder > 1000 {
            self.terrain.at_mut(x, y, z).placeholder = 0;
          }
        }
      }
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
