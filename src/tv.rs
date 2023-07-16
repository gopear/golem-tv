pub const WIDTH: u32 = 336;
pub const HEIGHT: u32 = 240;
pub struct TV {
    pub id: u32,
    pub ip: String,
    pub sides: u32,
    pub top: u32,
    pub bottom: u32,
    // pub mask: SliceInfo<[ndarray::SliceInfoElem; 2], ndarray::Dim<[usize; 2]>, ndarray::Dim<[usize; 2]>>
}

impl TV {

  // pub fn new(id: u32, ip: String) -> Self {
  //   TV {
  //     id,
  //     ip,
  //     mask: s![.., ..]
  //   }
  // }

  pub fn new(id: u32, sides: f32, top: f32, bottom: f32, diagonal_cm: f32,  ip: String) -> Self {

    let ppcm = ((WIDTH.pow(2) + HEIGHT.pow(2)) as f32).sqrt() / diagonal_cm;

    let side_p = f32::ceil(sides*ppcm) as u32;
    let top_p = f32::ceil(top*ppcm) as u32;
    let bottom_p = f32::ceil(bottom*ppcm) as u32;

    // let mask = s![top_p..bottom_p, side_p..-side_p];
    
    TV {
      id,
      ip,
      sides: side_p,
      top: top_p,
      bottom: bottom_p,
    }
  }

}

impl Clone for TV {
  fn clone(&self) -> Self {
    TV {
      id: self.id.clone(),
      ip: self.ip.clone(),
      // mask: self.mask.clone()
      sides: self.sides.clone(),
      top: self.top.clone(),
      bottom: self.bottom.clone(),
    }
  }
}
