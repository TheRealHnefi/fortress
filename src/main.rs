#[macro_use]
extern crate glium;
extern crate cgmath;

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
    
  let window = glium::glutin::WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title(format!("Fortress"))
    .build_glium()
    .unwrap();
    
  let resources = Resources::new(&window);
  let session = Session::new(&resources);
  let renderer = Renderer::new(&window);

  let mut running = true;
  while running {
    renderer.render(window.draw(), session.get_framegraph());
    
    for event in window.poll_events() {
      match event {
        glium::glutin::Event::Closed => running = false,
        _ => ()
      }
    }
  }
}
