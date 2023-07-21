#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
use win::Display;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::Display;

// mod tv;
// use tv::{HEIGHT, TV, WIDTH};

use image::{RgbaImage, DynamicImage};
// use kiddo::float::{kdtree::KdTree, distance::squared_euclidean};
// use ndarray::{Array2, s};
// use tokio::net::UdpSocket;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};

mod tv;
use tv::{TV, WIDTH, HEIGHT};

#[derive(Serialize, Deserialize)]
struct Config {
    TVs: Vec<TV>,
    display_id: u32,
}

#[tokio::main]
async fn main() -> std::io::Result<()>  {

    let lut = Arc::new(std::fs::read("palette.lut").unwrap());
    println!("LUT loaded");

    let config_str = std::fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(config_str.as_str()).unwrap();

    let tvs = config.TVs.iter().map(|tv| {
        TV::new(tv.id, tv.x, tv.y, tv.width, tv.height, tv.ip.clone())
    }).collect::<Vec<TV>>();
    let tvs = Arc::new(tvs);
    
    let mut main_screen = Display::new(1);
    let (width, height) = (main_screen.width as u32, main_screen.height as u32);
    println!("{}x{}", width, height);

    let mut img = image::DynamicImage::new_rgba8(width, height);

    let listener = TcpListener::bind("0.0.0.0:12345").await?;

    loop {

      let sc = main_screen.capture().unwrap();
      if sc.len() == 0 {
          continue;
      }

      img = RgbaImage::from_raw(width, height, sc).unwrap().into();

      let (socket, _) = listener.accept().await.unwrap();
      // let mut dst = String::new();
      // let _ = socket.read_to_string(&mut dst);
      // let id = dst.trim().parse::<u32>().unwrap();

      // let tvs = tvs.clone();
      // let current_tv = tvs.iter().find(|tv| tv.id == id).unwrap();
      let current_tv = &tvs[1];
      let id = 1;
      process(socket, id, current_tv, img, lut.clone()).await;
    }
}

async fn process(mut socket: TcpStream, id: u32, tv: &TV, img: DynamicImage, lut: Arc<Vec<u8>>) {
  let current_part = img
    .crop_imm(
        tv.x,
        tv.y,
        tv.width,
        tv.height,
        // (col_i as u32) * WIDTH + tv.sides,
        // (row_i as u32) * HEIGHT + tv.top,
        // WIDTH - (tv.sides * 2),
        // HEIGHT - tv.top - tv.bottom,
    )
    .resize_to_fill(WIDTH, HEIGHT, image::imageops::FilterType::Nearest)
    .into_rgba8();
            
    let tv_out = current_part
        .pixels()
        .map(|p| {
            let index = ((p.0[0] as i32) << 16) + ((p.0[1] as i32) << 8) + (p.0[2] as i32);
            lut[index as usize]
        })
        .collect::<Vec<u8>>();

    
    println!("Sending to {}", id);
    socket.write_all(tv_out.as_slice()).await.unwrap();
    
}
