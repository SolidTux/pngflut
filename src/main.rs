extern crate image;
extern crate rand;

use image::*;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use rand::distributions::{IndependentSample, Range};

fn main() {
    let image = open("image.png").expect("image").to_rgba();

    let x0 = 0;
    let y0 = 0;

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

    let nthreads = 1000;

    println!("Starting.");

    let mut rng = rand::thread_rng();
    loop {
        let mut handles = Vec::new();
        for n in 0..nthreads {
            let d = data.clone();
            let between = Range::new(0, d.len());
            // let indoff = n * d.len() / nthreads;
            let indoff = between.ind_sample(&mut rng);
            handles.push(thread::spawn(move || {
                loop {
                    // match TcpStream::connect("94.45.231.39:1234") {
                    match TcpStream::connect("151.217.47.77:8080") {
                        Ok(mut stream) => {
                            stream.set_nodelay(true).expect("set_nodelay call failed");
                            let mut i = indoff;
                            loop {
                                i = (i + 1) % d.len();
                                match stream.write_fmt(format_args!("{}", d[i])) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        println!("Write error, connecting again ..");
                                        break;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            println!("Connection error, trying again ...");
                            thread::sleep(Duration::from_secs(1))
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
