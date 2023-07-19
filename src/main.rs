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

use image::RgbaImage;
use rayon::prelude::*;
// use kiddo::float::{kdtree::KdTree, distance::squared_euclidean};
// use ndarray::{Array2, s};
use std::{net::UdpSocket, fs::File};
// use tokio::net::UdpSocket;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};


mod tv;
use tv::TV;

#[derive(Serialize, Deserialize)]
struct Config {
    TVs: Vec<TV>,
    display_id: u32,
}

use std::{thread, time};


fn main() {
    // let items: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0],[15.0, 15.0, 15.0],[27.0, 27.0, 27.0],[39.0, 39.0, 39.0],[51.0, 51.0, 51.0],[65.0, 65.0, 65.0],[79.0, 79.0, 79.0],[94.0, 94.0, 94.0],[104.0, 104.0, 104.0],[120.0, 120.0, 120.0],[137.0, 137.0, 137.0],[154.0, 154.0, 154.0],[171.0, 171.0, 171.0],[191.0, 191.0, 191.0],[211.0, 211.0, 211.0],[234.0, 234.0, 234.0],[0.0, 22.0, 0.0],[15.0, 33.0, 0.0],[26.0, 45.0, 0.0],[39.0, 57.0, 0.0],[51.0, 69.0, 0.0],[64.0, 83.0, 0.0],[79.0, 97.0, 0.0],[93.0, 112.0, 0.0],[104.0, 122.0, 0.0],[119.0, 138.0, 23.0],[137.0, 155.0, 41.0],[154.0, 172.0, 59.0],[171.0, 189.0, 76.0],[190.0, 209.0, 96.0],[210.0, 229.0, 116.0],[233.0, 252.0, 139.0],[28.0, 0.0, 0.0],[39.0, 19.0, 0.0],[51.0, 31.0, 0.0],[63.0, 43.0, 0.0],[75.0, 55.0, 0.0],[89.0, 69.0, 0.0],[103.0, 83.0, 0.0],[117.0, 97.0, 0.0],[128.0, 108.0, 18.0],[143.0, 124.0, 34.0],[161.0, 141.0, 52.0],[178.0, 158.0, 69.0],[195.0, 175.0, 86.0],[214.0, 195.0, 106.0],[234.0, 215.0, 126.0],[255.0, 238.0, 150.0],[47.0, 0.0, 0.0],[58.0, 0.0, 0.0],[70.0, 15.0, 0.0],[82.0, 28.0, 0.0],[94.0, 40.0, 0.0],[108.0, 54.0, 0.0],[122.0, 68.0, 22.0],[136.0, 82.0, 36.0],[146.0, 93.0, 47.0],[162.0, 109.0, 63.0],[179.0, 126.0, 80.0],[196.0, 143.0, 98.0],[214.0, 160.0, 115.0],[233.0, 180.0, 135.0],[253.0, 200.0, 155.0],[255.0, 223.0, 178.0],[57.0, 0.0, 0.0],[68.0, 0.0, 0.0],[80.0, 0.0, 10.0],[92.0, 15.0, 23.0],[104.0, 27.0, 35.0],[117.0, 41.0, 49.0],[132.0, 55.0, 63.0],[146.0, 70.0, 78.0],[156.0, 80.0, 88.0],[172.0, 96.0, 104.0],[189.0, 113.0, 121.0],[206.0, 131.0, 138.0],[223.0, 148.0, 156.0],[242.0, 167.0, 175.0],[255.0, 187.0, 195.0],[255.0, 210.0, 218.0],[55.0, 0.0, 32.0],[67.0, 0.0, 44.0],[78.0, 0.0, 55.0],[90.0, 0.0, 68.0],[102.0, 19.0, 80.0],[116.0, 33.0, 93.0],[130.0, 48.0, 108.0],[144.0, 62.0, 122.0],[155.0, 73.0, 132.0],[170.0, 89.0, 148.0],[188.0, 106.0, 165.0],[205.0, 123.0, 182.0],[222.0, 140.0, 199.0],[241.0, 160.0, 219.0],[255.0, 180.0, 239.0],[255.0, 203.0, 255.0],[43.0, 0.0, 71.0],[54.0, 0.0, 82.0],[66.0, 0.0, 94.0],[78.0, 0.0, 106.0],[90.0, 18.0, 118.0],[103.0, 32.0, 131.0],[118.0, 47.0, 146.0],[132.0, 61.0, 160.0],[142.0, 72.0, 170.0],[158.0, 88.0, 186.0],[175.0, 105.0, 203.0],[192.0, 122.0, 220.0],[209.0, 140.0, 237.0],[229.0, 159.0, 255.0],[249.0, 179.0, 255.0],[255.0, 202.0, 255.0],[22.0, 0.0, 95.0],[33.0, 0.0, 106.0],[45.0, 0.0, 118.0],[57.0, 12.0, 130.0],[69.0, 25.0, 141.0],[83.0, 39.0, 155.0],[97.0, 53.0, 169.0],[111.0, 68.0, 183.0],[122.0, 78.0, 194.0],[138.0, 94.0, 209.0],[155.0, 111.0, 226.0],[172.0, 129.0, 243.0],[189.0, 146.0, 255.0],[208.0, 165.0, 255.0],[228.0, 185.0, 255.0],[251.0, 208.0, 255.0],[0.0, 0.0, 99.0],[0.0, 0.0, 111.0],[20.0, 12.0, 122.0],[32.0, 24.0, 134.0],[44.0, 37.0, 146.0],[58.0, 50.0, 159.0],[72.0, 65.0, 174.0],[87.0, 79.0, 188.0],[97.0, 90.0, 198.0],[113.0, 106.0, 214.0],[130.0, 123.0, 231.0],[148.0, 140.0, 248.0],[165.0, 157.0, 255.0],[184.0, 177.0, 255.0],[204.0, 197.0, 255.0],[227.0, 220.0, 255.0],[0.0, 0.0, 84.0],[0.0, 15.0, 95.0],[0.0, 27.0, 106.0],[0.0, 39.0, 118.0],[21.0, 51.0, 130.0],[35.0, 65.0, 144.0],[49.0, 80.0, 158.0],[64.0, 94.0, 172.0],[74.0, 104.0, 182.0],[90.0, 120.0, 198.0],[107.0, 137.0, 215.0],[125.0, 155.0, 232.0],[142.0, 172.0, 249.0],[161.0, 191.0, 255.0],[181.0, 211.0, 255.0],[204.0, 234.0, 255.0],[0.0, 19.0, 50.0],[0.0, 30.0, 62.0],[0.0, 42.0, 73.0],[0.0, 54.0, 85.0],[0.0, 66.0, 97.0],[18.0, 80.0, 111.0],[32.0, 94.0, 125.0],[47.0, 109.0, 139.0],[57.0, 119.0, 150.0],[73.0, 135.0, 166.0],[91.0, 152.0, 183.0],[108.0, 169.0, 200.0],[125.0, 186.0, 217.0],[145.0, 206.0, 236.0],[165.0, 226.0, 255.0],[188.0, 249.0, 255.0],[0.0, 31.0, 0.0],[0.0, 42.0, 18.0],[0.0, 53.0, 30.0],[0.0, 66.0, 42.0],[0.0, 78.0, 54.0],[11.0, 91.0, 68.0],[25.0, 106.0, 83.0],[40.0, 120.0, 97.0],[51.0, 130.0, 107.0],[67.0, 146.0, 123.0],[84.0, 163.0, 140.0],[101.0, 180.0, 158.0],[119.0, 198.0, 175.0],[138.0, 217.0, 194.0],[158.0, 237.0, 214.0],[181.0, 255.0, 237.0],[0.0, 36.0, 0.0],[0.0, 48.0, 0.0],[0.0, 59.0, 0.0],[0.0, 71.0, 0.0],[0.0, 83.0, 10.0],[16.0, 97.0, 24.0],[30.0, 111.0, 39.0],[45.0, 126.0, 53.0],[55.0, 136.0, 64.0],[71.0, 152.0, 80.0],[89.0, 169.0, 97.0],[106.0, 186.0, 114.0],[123.0, 203.0, 132.0],[143.0, 222.0, 151.0],[163.0, 242.0, 171.0],[186.0, 255.0, 194.0],[0.0, 35.0, 0.0],[0.0, 47.0, 0.0],[0.0, 58.0, 0.0],[0.0, 70.0, 0.0],[17.0, 82.0, 0.0],[31.0, 96.0, 0.0],[46.0, 110.0, 0.0],[60.0, 124.0, 18.0],[71.0, 135.0, 28.0],[87.0, 151.0, 45.0],[104.0, 168.0, 62.0],[121.0, 185.0, 79.0],[138.0, 202.0, 97.0],[158.0, 221.0, 116.0],[178.0, 241.0, 137.0],[201.0, 255.0, 160.0],[0.0, 27.0, 0.0],[0.0, 39.0, 0.0],[15.0, 50.0, 0.0],[28.0, 62.0, 0.0],[40.0, 74.0, 0.0],[54.0, 88.0, 0.0],[68.0, 102.0, 0.0],[82.0, 117.0, 0.0],[93.0, 127.0, 0.0],[109.0, 143.0, 25.0],[126.0, 160.0, 43.0],[143.0, 177.0, 61.0],[160.0, 194.0, 78.0],[180.0, 214.0, 98.0],[200.0, 234.0, 118.0],[223.0, 255.0, 141.0],[17.0, 14.0, 0.0],[29.0, 26.0, 0.0],[41.0, 37.0, 0.0],[53.0, 49.0, 0.0],[65.0, 61.0, 0.0],[79.0, 75.0, 0.0],[93.0, 90.0, 0.0],[107.0, 104.0, 0.0],[118.0, 114.0, 11.0],[133.0, 130.0, 27.0],[151.0, 147.0, 45.0],[168.0, 164.0, 62.0],[185.0, 182.0, 80.0],[204.0, 201.0, 99.0],[224.0, 221.0, 119.0],[247.0, 244.0, 143.0],];
    // let kdtree: KdTree<f32, usize, 3, 32, u16> = KdTree::from(&items);
    let lut = std::fs::read("palette.lut").unwrap();
    println!("LUT loaded");

    let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

    let mut main_screen = Display::new(1);
    let (width, height) = (main_screen.width as u32, main_screen.height as u32);
    println!("{}x{}", width, height);

    let mut t = SystemTime::now();
    let mut cnt = 0;

    let config_str = std::fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(config_str.as_str()).unwrap();

    let tvs = config.TVs.iter().map(|tv| {
        TV::new(tv.id, tv.x, tv.y, tv.width, tv.height, tv.ip.clone())
    }).collect::<Vec<TV>>();


    tvs.iter().for_each(|tv| {
        println!("{}: {}", tv.id, tv.ip);
    });

    // let tvs =  [
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.54")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    //     TV::new(0, 5.0, 5.0, 5.0, 110.0, String::from("192.168.0.55")),
    // ];

    // let (wall_width, wall_height) = (tvs[0].len() as u32, tvs.len() as u32);
    // let (wall_res_width, wall_res_height) = (WIDTH * wall_width, HEIGHT * wall_height);
    // println!("{}x{}", wall_res_width, wall_res_height);

    // let kdtree = Arc::new(kdtree);
    // let socket = Arc::new(socket);

    loop {
        let sc = main_screen.capture().unwrap();
        if sc.len() == 0 {
            continue;
        }

        let img: image::DynamicImage = RgbaImage::from_raw(width, height, sc).unwrap().into();

        // let mut out = Array2::<u8>::zeros((wall_res_height as usize, wall_res_width as usize));
        // for (x, y, pixel) in resized.enumerate_pixels() {
        //     out[[y as usize, x as usize]] = kdtree
        //         .nearest_one(
        //             &[pixel.0[0] as f32, pixel.0[1] as f32, pixel.0[2] as f32],
        //             &squared_euclidean,
        //         )
        //         .1 as u8;
        // }

        // let resized = img.resize_exact(
        //     wall_res_width,
        //     wall_res_height,
        //     image::imageops::FilterType::Nearest,
        // );
        // let resized = Arc::new(resized);
        let resized = img;
        // resized.save("out.png").unwrap();


        tvs.par_iter().for_each(|tv| {
            let current_part = resized
                .crop_imm(
                    0,
                    0,
                    tv.width,
                    tv.height,
                    // (col_i as u32) * WIDTH + tv.sides,
                    // (row_i as u32) * HEIGHT + tv.top,
                    // WIDTH - (tv.sides * 2),
                    // HEIGHT - tv.top - tv.bottom,
                )
                .resize_to_fill(tv.width, tv.height, image::imageops::FilterType::Nearest)
                .into_rgba8();
            // let id = tv.id;
            // ori.save(format!("tv{id}.png")).unwrap();
            // current_part.save(format!("tv{id}_crop.png")).unwrap();
            let tv_out = current_part
                .pixels()
                .map(|p| {
                    let index = ((p.0[0] as i32) << 16) + ((p.0[1] as i32) << 8) + (p.0[2] as i32);
                    lut[index as usize]
                })
                .collect::<Vec<u8>>();

            let ip = tv.ip.clone();
            tv_out
                .chunks((tv.width*3) as usize)
                .enumerate()
                .for_each(|(i, r)| {
                    match socket.send_to(&[&[(i * 3) as u8], r].concat(), format!("{ip}:1234")) {
                        Ok(_) => (),
                        Err(e) => println!("Error: {}", e),
                    }
                    thread::sleep(time::Duration::from_micros(100));
                });
        });


        cnt += 1;
        if t.elapsed().unwrap().as_secs() >= 1 {
            println!("FPS: {}", cnt);
            cnt = 0;
            t = SystemTime::now();
        }
    }
}
