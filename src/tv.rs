use serde::{Serialize, Deserialize};

pub const WIDTH: u32 = 336;
pub const HEIGHT: u32 = 240;

#[derive(Serialize, Deserialize)]
pub struct TV {
    pub id: u32,
    pub ip: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl TV {

  pub fn new(id: u32, x: u32, y: u32, width: u32, height: u32, ip: String) -> Self {
    
    TV {
      id,
      ip,
      x,
      y,
      width,
      height
    }
  }

}

impl Clone for TV {
  fn clone(&self) -> Self {
    TV {
      id: self.id.clone(),
      ip: self.ip.clone(),
      x: self.x.clone(),
      y: self.y.clone(),
      width: self.width.clone(),
      height: self.height.clone()
    }
  }
}
