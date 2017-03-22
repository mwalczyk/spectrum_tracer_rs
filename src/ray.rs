use vector::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    pub t_min: f64,
    pub t_max: f64,
}

impl Ray {
    pub fn new(o: &Vector, d: &Vector, t_min: f64, t_max: f64) -> Ray {
        Ray {
            origin: *o,
            direction: d.normalize(),
            t_min: t_min,
            t_max: t_max,
        }
    }

    pub fn point_at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
