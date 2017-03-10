#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rand;
use rand::Rng;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::thread;
use std::sync::Arc;

mod vector;
mod ray;
mod shape;
mod material;

use vector::Vector;
use ray::Ray;
use shape::Shape;
use shape::Intersection;
use shape::Sphere;
use shape::ShapeAggregate;
use material::Material;
use material::Lambertian;
use material::Metallic;

// output resolution
const RES_X: u32 = 800;
const RES_Y: u32 = 800;
const SAMPLES: u32 = 64;
const MAX_DEPTH: u32 = 5;
const NUMBER_OF_THREADS: u32 = 40;
const GAMMA: f64 = 1.0 / 2.2;

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

fn color_scene(r: &Ray, scene: &ShapeAggregate, depth: u32) -> Vector {
    let intersection = scene.intersect(&r, 0.001, std::f64::MAX);
    match intersection {
        Intersection::Hit { position, normal, ref material, .. } => {
            let mut attenuation = Vector::one();
            if depth < MAX_DEPTH {
                if let Some(bounce_ray) = material.scatter(&r, &intersection, &mut attenuation) {
                    return attenuation * color_scene(&bounce_ray, &scene, depth + 1);
                } else {
                    Vector::zero()
                }
            } else {
                Vector::zero()
            }

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

fn threaded_color(start: (u32, u32), end: (u32, u32), scene: Arc<ShapeAggregate>) -> Vec<Color> {
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
                let ray = Ray::new(&ORIGIN,
                                   &mut (LOWER_LEFT_CORNER + HORIZONTAL * u + VERTICAL * v));
                col += color_scene(&ray, &scene, 0);
            }

            col /= SAMPLES as f64;
            let gamma_corrected = Vector {
                x: col.x.powf(GAMMA),
                y: col.y.powf(GAMMA),
                z: col.z.powf(GAMMA),
            };

            // convert colors to 0..255
            let ir = (255.99 * gamma_corrected.x) as u32;
            let ig = (255.99 * gamma_corrected.y) as u32;
            let ib = (255.99 * gamma_corrected.z) as u32;

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
    let mat_0 = Arc::new(Lambertian {
        albedo: Vector {
            x: 1.0,
            y: 0.98,
            z: 0.96,
        },
    });

    let mat_1 = Arc::new(Lambertian {
        albedo: Vector {
            x: 1.0,
            y: 0.3,
            z: 0.1,
        },
    });

    let mut scene = ShapeAggregate::new();
    let sphere_0 = Box::new(Sphere {
        center: Vector {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: mat_1.clone(),
    });
    let sphere_1 = Box::new(Sphere {
        center: Vector {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: mat_0.clone(),
    });
    scene.items.push(sphere_0);
    scene.items.push(sphere_1);

    // wrap the scene in an automatic reference counter so that
    // it can be shared immutably across multiple threads
    let shared_scene = Arc::new(scene);

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
