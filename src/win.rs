use dxgcap::{DXGIManager, CaptureError};

pub struct Display {
    manager: DXGIManager,
    pub width: u32,
    pub height: u32
}

impl Display {
    pub fn new(id: u32) -> Self {

        let mut manager = DXGIManager::new(200).unwrap();
        manager.set_capture_source_index(id as usize);
        let (width, height) = manager.geometry();

        Display {
            width: width as u32,
            height: height as u32,
            manager
        }
    }

    pub fn capture(&mut self) -> Result<Vec<u8>, CaptureError> {
        match self.manager.capture_frame_components() {
            Ok((frame, (_width, _height))) => {
                Ok(frame)
            },
            Err(e) => {
                match e {
                    CaptureError::Timeout => Ok(vec![]),
                    _ => Err(e)
                }
            }
        }
    }
}