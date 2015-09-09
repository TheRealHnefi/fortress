pub struct Session {
  pub TickCounter : i64,
}

impl Session {
  pub fn Tick(&mut self) -> () {
    self.TickCounter += 1;
  }
}