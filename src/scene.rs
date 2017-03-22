use shape::DifferentialGeometry;
use ray::Ray;
use material::Material;
use primitive::Primitive;

use std::sync::Arc;

// Scenes contain a list of primitives
pub struct Scene {
    pub items: Vec<Primitive>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { items: Vec::new() }
    }

    pub fn intersect(&self, incident: &Ray) -> Option<(DifferentialGeometry, Arc<Material>)> {
        let mut closest_intersection = None;
        let mut closest_t = incident.t_max;

        // Test against every object and find the closest point of intersection
        for item in &self.items {
            if let Some((dg, mtl)) = item.intersect(&incident) {
                if dg.t < closest_t {
                    closest_t = dg.t;
                    closest_intersection = Some((dg, mtl));
                }
            }
        }
        closest_intersection
    }
}
