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
use std::sync::{Arc, Mutex};
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

    let listener = TcpListener::bind("0.0.0.0:12345").await?;

    let counter = Arc::new(Mutex::new([0; 13]));

    let mut sc = main_screen.capture().unwrap();
    let mut img: Arc<DynamicImage> = Arc::new(RgbaImage::from_raw(width, height, sc).unwrap().into());

    let timeout = std::time::Duration::from_millis(1000);
    let mut time = std::time::Instant::now();
    let mut sync_sum = 11;
    loop {

      let (mut socket, _) = listener.accept().await.unwrap();
      let counter = counter.clone();

    //   if counter.lock().unwrap().iter().sum::<i32>() == sync_sum || time.elapsed() > timeout {
    //     counter.lock().unwrap().iter_mut().for_each(|c| *c = 0);
        
        sc = main_screen.capture().unwrap();
        img = Arc::new(RgbaImage::from_raw(width, height, sc).unwrap().into());

    //     if time.elapsed() > timeout {
    //         sync_sum -= 1;

    //     }

    //     time = std::time::Instant::now();
    //   }

      let tvs = tvs.clone();
      let lut = lut.clone();
      let img = img.clone();

      tokio::spawn(async move {

        let mut buffer = [0; 2];
        let bytes_read = socket.read(&mut buffer).await.unwrap();
        let data = String::from_utf8_lossy(&buffer[..bytes_read]);
        let cleaned_data: String = data.chars().filter(|&c| !c.is_whitespace()).collect();
        let id = cleaned_data.parse::<u32>().unwrap();

        let current_tv = tvs.iter().find(|tv| tv.id == id).unwrap();
        process(socket, current_tv, img, lut, counter.clone()).await;

        // if counter.lock().unwrap()[id as usize] == 0 {
        //     let current_tv = tvs.iter().find(|tv| tv.id == id).unwrap();
        //     process(socket, current_tv, img, lut, counter.clone()).await;
        // }
        
      });
    }
}

async fn process(mut socket: TcpStream, tv: &TV, img: Arc<DynamicImage>, lut: Arc<Vec<u8>>, counter: Arc<Mutex<[i32; 13]>>) {
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

    
    println!("Sending to {}", tv.id);
    counter.lock().unwrap()[tv.id as usize] = 1;
    socket.write(&tv_out).await.unwrap();
    
}
