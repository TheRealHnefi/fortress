pub struct Session {
  pub tick_counter : i64,
}

impl Session {
  pub fn tick(&mut self) -> ()
  {
    self.tick_counter += 1;
  }
}