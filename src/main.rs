#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate clock_ticks;

mod session;
mod renderer;
mod world;
mod framegraph;
mod vertexobject;
mod resources;
mod block;
mod volume;

fn main() {
  use glium::DisplayBuild;
  use session::Session;
  use renderer::Renderer;
  use resources::Resources;
  use clock_ticks;
    
  let window = glium::glutin::WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title(format!("Fortress"))
    .with_depth_buffer(24)
    .build_glium()
    .unwrap();

  let mut time = clock_ticks::precise_time_ms();
  let mut ticks = 0 as i32;
    
  let resources = Resources::new(&window);
  let session = Session::new(&resources);
  let renderer = Renderer::new(&window);

  let mut running = true;
  while running {
    ticks += 1;
    let new_time = clock_ticks::precise_time_ms();
    if new_time - time > 1000 {
      time = new_time;
      println!("Tickrate: {}", ticks);
      ticks = 0;
    }
    
    renderer.render(window.draw(), session.get_framegraph());
    
    for event in window.poll_events() {
      match event {
        glium::glutin::Event::Closed => running = false,
        _ => ()
      }
    }
  }
}
