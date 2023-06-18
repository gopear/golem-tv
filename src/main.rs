use std::{net::UdpSocket, time::SystemTime};
use captrs::*;
fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("localhost:8000").expect("Could not bind to socket");

        let mut capturer = Capturer::new(0).unwrap();
        let (w,h) = capturer.geometry();
        println!("w: {}, h: {}", w, h);
    
        let client_nr = 1;
        capturer.capture_store_frame().unwrap();
        loop {
            let start = SystemTime::now();
            capturer.capture_store_frame().unwrap();
            let (mut tr, mut tg, mut tb) : (u8, u8, u8) = (0, 0, 0);
            for Bgr8 {r, g, b, .. } in capturer.get_stored_frame().unwrap() {
                tr = *r;
                tg = *g;
                tb = *b;
            }
            println!("it took {} miliseconds", SystemTime::now().duration_since(start).unwrap().as_millis());
            break;

            // println!("ps: {}", ps.);

            // for client in 1..client_nr+1 {
            //     match socket.send_to(&[1], format!("localhost:800{}", client)) {
            //         Ok(_) => println!("Sent"),
            //         Err(e) => println!("Error: {}", e),
            //     }
            // }
        }
    }
    Ok(())
}
