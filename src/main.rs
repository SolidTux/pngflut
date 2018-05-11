extern crate image;
extern crate rand;

use image::*;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {
    let image = open("image.png").expect("image").to_rgba();

    let x0 = 200;
    let y0 = 200;

    let mut data = Vec::new();

    for x in 0..image.width() {
        for y in 0..image.height() {
            let (r, g, b, a) = image.get_pixel(x, y).channels4();
            if a > 10 {
                data.push(format!(
                    "PX {} {} {:02X}{:02X}{:02X}\n",
                    x0 + x,
                    y0 + y,
                    r,
                    g,
                    b
                ));
            }
        }
    }

    let nthreads = 100;

    let boxed_data = Box::new(data);
    loop {
        let mut handles = Vec::new();
        let global_indv: Vec<usize> = (0..boxed_data.len()).collect();
        for n in 0..nthreads {
            println!("Starting {:2} / {:2}", n, nthreads);
            thread::sleep(Duration::from_millis(
                (1000. * rand::random::<f32>()) as u64,
            ));
            let d = boxed_data.clone();
            let mut indv = global_indv.clone();
            rand::thread_rng().shuffle(&mut indv);
            handles.push(thread::spawn(move || {
                loop {
                    let mut indices = indv.iter().cycle();
                    match TcpStream::connect("94.45.234.189:1234") {
                        Ok(mut stream) => {
                            stream.set_nodelay(true).expect("set_nodelay call failed");
                            loop {
                                // i = (i + 1) % d.len();
                                match indices.next() {
                                    Some(i) => match stream.write_fmt(format_args!("{}", d[*i])) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            println!("Write error, connecting again ..");
                                            break;
                                        }
                                    },
                                    None => {
                                        break;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            println!("Connection error, trying again ...");
                            thread::sleep(Duration::from_millis(
                                (100. * rand::random::<f32>()) as u64,
                            ));
                        }
                    }
                }
            }));
        }
        for h in handles.into_iter() {
            h.join().unwrap();
        }
    }
}
