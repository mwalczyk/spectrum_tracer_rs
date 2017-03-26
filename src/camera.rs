use vector::Vector;
use ray::Ray;

use std::f64;

pub struct Camera {
    // The vertical field of view, in degrees
    pub fov: f64,
    // The aspect ratio of the image plane, i.e. 4:3
    pub aspect_ratio: f64,
    // The position of the camera, in world-space
    origin: Vector,
    // A position vector describing the lower-left corner of the image plane
    lower_left_corner: Vector,
    // A direction vector that runs along the horizontal edge of the image plane
    horizontal: Vector,
    // A direction vector that runs along the vertical edge of the image plane
    vertical: Vector,
}

impl Camera {
    pub fn new(fov: f64, aspect_ratio: f64) -> Camera {
        // Convert the field of view to radians
        let theta = fov * (f64::consts::PI / 180.0);
        let half_height = (theta * 0.5).tan();
        let half_width = aspect_ratio * half_height;
        Camera {
            fov: fov,
            aspect_ratio: aspect_ratio,
            origin: Vector::zero(),
            lower_left_corner: Vector::new(-half_width, -half_height, -1.0),
            horizontal: Vector::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vector::new(0.0, 2.0 * half_height, 0.0),
        }
    }

    pub fn generate_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(&self.origin,
                 &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin),
                 0.001,
                 f64::MAX)
    }
}
