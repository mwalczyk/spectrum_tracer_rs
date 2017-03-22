use shape::Shape;
use shape::DifferentialGeometry;
use ray::Ray;
use material::Material;

use std::sync::Arc;

// Primitives are instances of renderable geometry
pub struct Primitive {
    pub shape: Arc<Shape>,
    pub material: Arc<Material>,
}

impl Primitive {
    pub fn new(s: Arc<Shape>, m: Arc<Material>) -> Primitive {
        Primitive {
            shape: s,
            material: m,
        }
    }

    pub fn intersect(&self, incident: &Ray) -> Option<(DifferentialGeometry, Arc<Material>)> {
        if let Some(dg) = self.shape.intersect(incident) {
            return Some((dg, self.material.clone()));
        };
        None
    }
}
