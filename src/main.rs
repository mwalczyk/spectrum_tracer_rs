#![allow(dead_code)]
#![allow(unused_variables)]

// External crates
extern crate rand;
use rand::Rng;

// Standard library
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::thread;
use std::sync::Arc;

// Bring custom modules into global scope
mod vector;
mod ray;
mod shape;
mod material;
mod primitive;
mod scene;

// Custom modules
use vector::Vector;
use ray::Ray;
use shape::Shape;
use shape::DifferentialGeometry;
use shape::Sphere;
use material::Material;
use material::Lambertian;
use material::Metallic;
use material::Dielectric;
use primitive::Primitive;
use scene::Scene;

// Output resolution
const RES_X: u32 = 800;
const RES_Y: u32 = 800;
const SAMPLES: u32 = 100;
const MAX_DEPTH: u32 = 5;
const NUMBER_OF_THREADS: u32 = 10;
const GAMMA: f64 = 1.0 / 2.2;

// Direction vectors for generating rays from uv-coordinates
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

fn trace(r: &Ray, scene: &Scene, depth: u32) -> Vector {
    let surface_interaction = scene.intersect(&r);
    match surface_interaction {
        // Hit
        Some((dg, mtl)) => {
            let mut attenuation = Vector::one();
            if depth < MAX_DEPTH {
                let bounce_ray = mtl.scatter(&r, &dg, &mut attenuation);
                return attenuation * trace(&bounce_ray, &scene, depth + 1);
            } else {
                Vector::zero()
            }
        }
        // Miss
        None => {
            let unit_direction = r.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            let white = Vector::one();
            let blue = Vector::new(0.5, 0.7, 1.0);
            white.lerp(&blue, t)
        }
    }
}

struct Color(u32, u32, u32);

fn threaded_color(start: (u32, u32), end: (u32, u32), scene: Arc<Scene>) -> Vec<Color> {
    let mut colors = Vec::new();
    let mut rng = rand::thread_rng();

    for y in start.1..end.1 {
        // Each row
        for x in start.0..end.0 {
            // Each col
            let mut col = Vector::zero();
            // Perform anti-aliasing
            for s in 0..SAMPLES {
                // The uv-coordinates of the current pixel with random offsets
                // (note that we flip the y-axis)
                let u = (x as f64 + rng.next_f64()) / RES_X as f64;
                let v = ((RES_Y - y) as f64 + rng.next_f64()) / RES_Y as f64;
                let r = Ray::new(&ORIGIN,
                                 &(LOWER_LEFT_CORNER + HORIZONTAL * u + VERTICAL * v),
                                 0.001,
                                 std::f64::MAX);
                col += trace(&r, &scene, 0);
            }

            col /= SAMPLES as f64;
            let gamma_corrected = col.powf(GAMMA);

            // Convert colors to 0..255
            let ir = (255.99 * gamma_corrected.x) as u32;
            let ig = (255.99 * gamma_corrected.y) as u32;
            let ib = (255.99 * gamma_corrected.z) as u32;

            colors.push(Color(ir, ig, ib));
        }
    }
    colors
}

fn map(v: f64, fmin: f64, fmax: f64, tmin: f64, tmax: f64) -> f64 {
    (v - fmin) / (tmin - fmin) * (tmax - fmax) + fmax
}

fn main() {
    let path = Path::new("output/render.ppm");
    let display = path.display();
    let mut file = File::create(&path).expect("couldn't create file");

    // Use the time module to record how long it takes to render the entire scene
    let start = Instant::now();
    println!("starting render: {} x {} px", RES_X, RES_Y);

    // Build a scene
    let mut scene = Scene::new();
    let mtl_large = Arc::new(Lambertian::new(&Vector::new(1.0, 0.1, 0.05)));
    let mtl_glass = Arc::new(Dielectric::new(1.5));

    let sph_large = Arc::new(Sphere::new(&Vector::new(0.0, -100.5, -1.0), 100.0));
    let sph_small_0 = Arc::new(Sphere::new(&Vector::new(0.0, 0.5, -2.0), 1.0));
    let sph_small_1 = Arc::new(Sphere::new(&Vector::new(0.0, 0.5, -2.0), -0.95));

    scene.items.push(Primitive::new(sph_large, mtl_large.clone()));
    scene.items.push(Primitive::new(sph_small_0, mtl_glass.clone()));
    scene.items.push(Primitive::new(sph_small_1, mtl_glass.clone()));

    // Wrap the scene in an automatic reference counter so that
    // it can be shared immutably across multiple threads
    let shared_scene = Arc::new(scene);

    let mut file_contents: String = format!("P3\n{} {}\n255\n", RES_X, RES_Y);
    let mut child_threads = vec![];
    for i in 0..NUMBER_OF_THREADS {
        let start: (u32, u32) = (0, i * (RES_X / NUMBER_OF_THREADS));
        let end: (u32, u32) = (RES_Y, (i + 1) * (RES_X / NUMBER_OF_THREADS));
        let cloned_scene = shared_scene.clone();
        child_threads.push(thread::spawn(move || threaded_color(start, end, cloned_scene)));
    }

    // Re-join threads and write ppm pixel data
    for child in child_threads {
        let res = child.join().unwrap();
        for item in res {
            let pixel = format!("{} {} {}\n", item.0, item.1, item.2);
            file_contents.push_str(&pixel);
        }
    }

    let elapsed = start.elapsed();

    // Write to the file
    match file.write_all(file_contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => {
            println!("successfully wrote to {}, finished in {:?} seconds",
                     display,
                     elapsed.as_secs())
        }
    }
}
