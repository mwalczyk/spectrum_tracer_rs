use shape::Shape;
use shape::DifferentialGeometry;
use material::Material;

use std::sync::Arc;

pub struct Geometry {
    pub shape: Arc<Shape>,
    pub material: Arc<Material>,
}

impl Geometry {
    pub fn intersect() -> Option<(DifferentialGeometry, Arc<Material>)>;
}
