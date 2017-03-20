use vector::Vector;
use ray::Ray;
use material::Material;
use material::Lambertian;

use std::sync::Arc;

const EPSILON: f64 = 0.001;

#[derive(Clone)]
pub struct DifferentialGeometry<'a> {
    // how far along the ray
    pub t: f64,
    // point of intersection
    pub position: Vector,
    // normal at point of intersection
    pub normal: Vector,
    // shape that was hit
    pub shape: &'a Shape,
}

/// / HOW DO YOU IMPOSE A CERTAIN MEMBER VARIABLE ON A TRAIT OBJECT???

pub trait Shape: Sync + Send {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<DifferentialGeometry>;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Shape for Sphere {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<DifferentialGeometry> {
        // sphere: dot((p - c), (p - c)) = r * r;
        // ray: a + b * t = p
        // substitute: dot((a + b * t - c), (a + b * t - c)) = r * r
        // expand: dot(b, b) * t * t + 2 * t * dot(b, a - c) + dot(a - c, a - c) - r * r = 0

        // the discriminant of the resulting quadratic equation will either be
        // positive (two real solutions), negative (no real solutions), or zero
        // (one real solution)
        let b = ((r.origin - self.center) * 2.0).dot(&r.direction);
        let c = (r.origin - self.center).dot(&(r.origin - self.center)) - self.radius * self.radius;
        let mut discriminant = b * b - 4.0 * c;

        if discriminant < 0.0 {
            return None;
        } else {
            discriminant = discriminant.sqrt();
        }

        let solution_0 = -b + discriminant;
        let solution_1 = -b - discriminant;

        if solution_1 > EPSILON {
            let t: f64 = solution_1 * 0.5;
            let position = r.point_at(t);
            let normal = (position - self.center) / self.radius;
            return Some(DifferentialGeometry {
                t: t,
                position: position,
                normal: normal,
                shape: self,
            });
        } else if solution_0 > EPSILON {
            let t: f64 = solution_0 * 0.5;
            let position = r.point_at(t);
            let normal = (position - self.center) / self.radius;
            return Some(DifferentialGeometry {
                t: t,
                position: position,
                normal: normal,
                shape: self,
            });
        } else {
            None
        }
    }
}

impl Default for Sphere {
    fn default() -> Sphere {
        Sphere {
            center: Vector::origin(),
            radius: 1.0,
            material: Arc::new(Lambertian { albedo: Vector::one() }),
        }
    }
}

pub struct ShapeAggregate {
    pub items: Vec<Box<Shape>>,
}

impl ShapeAggregate {
    pub fn new() -> ShapeAggregate {
        ShapeAggregate { items: Vec::new() }
    }
}

impl Shape for ShapeAggregate {
    fn intersect(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<DifferentialGeometry> {
        let mut closest_intersection: Option<DifferentialGeometry> = None;
        let mut closest_t = t_max;

        // test against every object and find the closest point of intersection
        for i in &self.items {
            if let Some(intersect) = i.intersect(&r, t_min, t_max) {
                if intersect.t < closest_t {
                    closest_t = intersect.t;
                    closest_intersection = Some(DifferentialGeometry {
                        t: intersect.t,
                        position: intersect.position,
                        normal: intersect.normal,
                        shape: intersect.shape,
                    });
                }
            }
        }
        closest_intersection
    }
}
