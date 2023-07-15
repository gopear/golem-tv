use core_graphics::display::{CGDisplay, kCGWindowImageDefault, kCGNullWindowID, kCGWindowListOptionOnScreenOnly, CGMainDisplayID};

pub struct Display {
    id: u32
}

impl Display {
    pub fn new(id: Option<u32>) -> Option<Display> {
        match GDisplay::new(id.unwrap_or(CGMainDisplayID())) {
            Ok(display) => Some(Display { id: id }),
            Err(_) => None
        }
    }

    pub fn capture(&self) -> Option<CGImage> {
        let image = CGDisplay::(
            CGDisplay::main().id(),
            kCGNullWindowID,
            kCGWindowImageDefault,
            kCGWindowListOptionOnScreenOnly
        );

        match image {
            Ok(image) => Some(image),
            Err(_) => None
        }
    }
}