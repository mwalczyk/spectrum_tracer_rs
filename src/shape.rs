use vector::Vector;
use ray::Ray;

const EPSILON: f64 = 0.001;

#[derive(Clone)]
pub struct DifferentialGeometry<'a> {
    // How far along the ray
    pub t: f64,
    // Point of intersection
    pub position: Vector,
    // Normal at point of intersection
    pub normal: Vector,
    // Shape that was hit
    pub shape: &'a Shape,
}

impl<'a> DifferentialGeometry<'a> {
    pub fn new(t: f64, p: &Vector, n: &Vector, s: &'a Shape) -> DifferentialGeometry<'a> {
        DifferentialGeometry {
            t: t,
            position: *p,
            normal: *n,
            shape: s,
        }
    }
}

pub trait Shape: Sync + Send {
    fn intersect(&self, r: &Ray) -> Option<DifferentialGeometry>;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
}

impl Shape for Sphere {
    fn intersect(&self, r: &Ray) -> Option<DifferentialGeometry> {
        // Sphere: dot((p - c), (p - c)) = r * r;
        // Ray: a + b * t = p
        // Substitute: dot((a + b * t - c), (a + b * t - c)) = r * r
        // Expand: dot(b, b) * t * t + 2 * t * dot(b, a - c) + dot(a - c, a - c) - r * r = 0

        // The discriminant of the resulting quadratic equation will either be
        // positive (two real solutions), negative (no real solutions), or zero
        // (one real solution)
        let b = ((r.origin - self.center) * 2.0).dot(&r.direction);
        let c = (r.origin - self.center).dot(&(r.origin - self.center)) - self.radius * self.radius;
        let mut discriminant = b * b - 4.0 * c;

        if discriminant < 0.0 {
            return None;
        }
        discriminant = discriminant.sqrt();

        let solution_0 = -b + discriminant;
        let solution_1 = -b - discriminant;

        if solution_1 > EPSILON {
            let t: f64 = solution_1 * 0.5;
            let position = r.point_at(t);
            let normal = (position - self.center) / self.radius;
            Some(DifferentialGeometry::new(t, &position, &normal, self))
        } else if solution_0 > EPSILON {
            let t: f64 = solution_0 * 0.5;
            let position = r.point_at(t);
            let normal = (position - self.center) / self.radius;
            Some(DifferentialGeometry::new(t, &position, &normal, self))
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
        }
    }
}

impl Sphere {
    pub fn new(c: &Vector, r: f64) -> Sphere {
        Sphere {
            center: *c,
            radius: r,
        }
    }
}

#[derive(Clone)]
pub struct Plane {
    pub center: Vector,
    pub normal: Vector,
}

impl Shape for Plane {
    fn intersect(&self, r: &Ray) -> Option<DifferentialGeometry> {
        // Ignore cases where the ray direction is parallel to the plane
        let denominator = r.direction.dot(&self.normal);
        if denominator.abs() > EPSILON {
            let p_minus_l = self.center - r.origin;
            let t = p_minus_l.dot(&self.normal) / denominator;

            // TODO: this is not correct - planes should be infinite
            if t >= EPSILON && r.point_at(t).y < 1.0 {
                return Some(DifferentialGeometry::new(t, &r.point_at(t), &self.normal, self));
            }
        }
        None
    }
}

impl Default for Plane {
    fn default() -> Plane {
        Plane {
            center: Vector::origin(),
            normal: Vector::new(0.0, 1.0, 0.0),
        }
    }
}

impl Plane {
    pub fn new(c: &Vector, n: &Vector) -> Plane {
        Plane {
            center: *c,
            normal: *n,
        }
    }
}
