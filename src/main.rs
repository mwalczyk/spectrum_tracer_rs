#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rand;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::thread;
use std::sync::Arc;

use rand::Rng;

mod vector;
mod ray;
mod hitable;

use vector::Vector;
use ray::Ray;
use hitable::Hitable;
use hitable::Intersection;
use hitable::Sphere;
use hitable::HitableList;

// output resolution
const RES_X: u32 = 800;
const RES_Y: u32 = 800;
const SAMPLES: u32 = 4;
const NUMBER_OF_THREADS: u32 = 40;

// direction vectors for generating rays from uv-coordinates
const LOWER_LEFT_CORNER: Vector = Vector {
    x: -1.0,
    y: -1.0,
    z: -1.0,
};
const HORIZONTAL: Vector = Vector {
    x: 2.0,
    y: 0.0,
    z: 0.0,
};
const VERTICAL: Vector = Vector {
    x: 0.0,
    y: 2.0,
    z: 0.0,
};
const ORIGIN: Vector = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

fn color_scene(r: &Ray, scene: &HitableList) -> Vector {
    let intersection = scene.hit(&r, 0.0, std::f64::MAX);
    match intersection {
        Intersection::Hit { position, normal, .. } => {
            let target = position + normal + Vector::random_in_unit_sphere();
            let bounce_ray = Ray {
                origin: position,
                direction: target - position,
            };
            color_scene(&bounce_ray, &scene) * 0.5
        }
        Intersection::Miss => {
            let mut unit_direction: Vector = r.direction;
            unit_direction.normalize();
            let t: f64 = 0.5 * (unit_direction.y + 1.0);
            let white = Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let blue = Vector {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };
            white * (1.0 - t) + blue * t
        }
    }
}

struct Color(u32, u32, u32);

fn threaded_color(start: (u32, u32), end: (u32, u32), scene: Arc<HitableList>) -> Vec<Color> {
    let mut colors = Vec::new();
    let mut rng = rand::thread_rng();

    for y in start.1..end.1 {
        // each row
        for x in start.0..end.0 {
            // each col
            let mut col = Vector {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            // perform anti-aliasing
            for s in 0..SAMPLES {
                // the uv-coordinates of the current pixel with random offsets
                // note that we flip the y-axis
                let u = (x as f64 + rng.next_f64()) / RES_X as f64;
                let v = ((RES_Y - y) as f64 + rng.next_f64()) / RES_Y as f64;
                let ray = Ray {
                    origin: ORIGIN,
                    direction: LOWER_LEFT_CORNER + HORIZONTAL * u + VERTICAL * v,
                };
                col += color_scene(&ray, &scene);
            }

            col /= SAMPLES as f64;

            // convert colors to 0..255
            let ir = (255.99 * col.x) as u32;
            let ig = (255.99 * col.y) as u32;
            let ib = (255.99 * col.z) as u32;

            colors.push(Color(ir, ig, ib));
        }
    }
    colors
}

fn main() {
    let path = Path::new("output/render.ppm");
    let display = path.display();
    let mut file = File::create(&path).expect("couldn't create file");

    // use the time module to record how long it takes to render the entire scene
    let start = Instant::now();
    println!("starting render: {} x {} px", RES_X, RES_Y);

    // build a scene
    let mut scene = HitableList::new();
    let sphere_0 = Box::new(Sphere {
        center: Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
    });
    let sphere_1 = Box::new(Sphere {
        center: Vector {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
    });
    scene.items.push(sphere_0);
    scene.items.push(sphere_1);

    // wrap the scene in an automatic reference counter so that
    // it can be shared immutably across multiple threads
    let mut shared_scene = Arc::new(scene);

    let mut file_contents: String = format!("P3\n{} {}\n255\n", RES_X, RES_Y);
    let mut child_threads = vec![];
    for i in 0..NUMBER_OF_THREADS {
        let start: (u32, u32) = (0, i * (RES_X / NUMBER_OF_THREADS));
        let end: (u32, u32) = (RES_Y, (i + 1) * (RES_X / NUMBER_OF_THREADS));
        let s = shared_scene.clone();
        child_threads.push(thread::spawn(move || threaded_color(start, end, s)));
    }

    // re-join threads and write ppm pixel data
    for child in child_threads {
        let res = child.join().unwrap();
        for item in res {
            let pixel = format!("{} {} {}\n", item.0, item.1, item.2);
            file_contents.push_str(&pixel);
        }
    }


    let elapsed = start.elapsed();

    // write to the file
    match file.write_all(file_contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => {
            println!("successfully wrote to {}, finished in {:?} seconds",
                     display,
                     elapsed.as_secs())
        }
    }
}
