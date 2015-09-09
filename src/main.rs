#[macro_use]
extern crate glium;

mod session;

fn main() {
  use glium::DisplayBuild;
    
  let window = glium::glutin::WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title(format!("Fortress"))
    .build_glium()
    .unwrap();
    
  let session = session::Session {TickCounter: 0};
      
  let mut running = true;
  while running {
    for event in window.poll_events() {
      match event {
        glium::glutin::Event::Closed => running = false,
        _ => ()
      }
    }
  }
}
