#[allow(dead_code)]
pub struct Volume<T> {
  data: Vec<T>,
  size_x: usize,
  size_y: usize,
  size_z: usize
}

impl<T: Clone> Volume<T> {
  pub fn new(value: T, x: usize, y: usize, z: usize) -> Volume<T> {
    let mut vector = Vec::with_capacity(x * y * z);
    for _ in 0..x*y*z {
      vector.push(value.clone());
    }
    Volume { data: vector, size_x: x, size_y: y, size_z: z }
  }

  pub fn at(&self, x: usize, y: usize, z: usize) -> &T {
    unsafe {
      let retval =
        self.data.get_unchecked((x + self.size_x * y + self.size_x * self.size_y * z) as usize);
      retval
    }
  }
  
  pub fn at_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
    unsafe {
      let retval =
        self.data.get_unchecked_mut((x + self.size_x * y + self.size_x * self.size_y * z) as usize);
      retval
    }
  }
}
