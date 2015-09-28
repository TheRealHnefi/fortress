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
    
  let resources = Resources::new(&window);
  let mut session = Session::new(&resources);
  let renderer = Renderer::new(&window);

  let mut running = true;
  while running {

    let new_time = clock_ticks::precise_time_ms();
    session.tick(new_time - time);
    renderer.render(window.draw(), session.get_framegraph());
    
    for event in window.poll_events() {
      match event {
        glium::glutin::Event::Closed => running = false,
        _ => ()
      }
    }
    
    time = new_time;
  }
}
