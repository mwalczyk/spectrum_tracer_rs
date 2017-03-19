use vector::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(o: &Vector, d: &Vector) -> Ray {
        Ray {
            origin: *o,
            direction: d.normalize(),
        }
    }

    pub fn point_at(&self, t: f64) -> Vector {
        self.origin + self.direction * t
    }
}
