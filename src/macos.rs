use core_graphics::display::{CGDisplay,CGMainDisplayID};
use std::fmt;

pub struct Display {
    pub width: u32,
    pub height: u32,
    display: CGDisplay
}

#[derive(Debug, Clone)]
pub struct CaptureError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not get image")
    }
}

impl Display {
    pub fn new(id: u32) -> Self {
        unsafe { CGMainDisplayID() }; //for some reason this is needed to get the correct display

        let display = CGDisplay::new(id);
        let (width, height) = (display.pixels_wide() as u32, display.pixels_high() as u32);

        Display {
            width,
            height,
            display
        }
    }

    pub fn capture(&self) -> Result<Vec<u8>, CaptureError> {
        match self.display.image() {
            Some(image) => {
                
                let size = (self.width * self.height * 4) as usize;
                let mut rgba = vec![0u8; size];

                let u_width = self.width as usize;
                let u_height = self.height as usize;
                let bytes_per_row = image.bytes_per_row();

                let bgra = image.data().bytes().to_vec();

                for r in 0..u_height {
                    for c in 0..u_width {
                        let index = (r * u_width + c) * 4;
                        let i = r * bytes_per_row + c * 4;

                        rgba[index] = bgra[i + 2];
                        rgba[index + 1] = bgra[i + 1];
                        rgba[index + 2] = bgra[i];
                        rgba[index + 3] = 255;
                    }
                }
                Ok(rgba)
            }
            None => Err(CaptureError),
        }
    }
}
