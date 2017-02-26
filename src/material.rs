use vector::Vector;
use ray::Ray;
use hitable::Intersection;

pub trait Material {
    // produce a scattered ray unless the incident
    // ray is absorbed, in which case None is returned
    fn scatter(&self,
               incident: &Ray,
               intersection: &Intersection,
               attenuation: &mut Vector)
               -> Option<Ray>;
}

#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub albedo: Vector,
}

impl Material for Lambertian {
    fn scatter(&self,
               incident: &Ray,
               intersection: &Intersection,
               attenuation: &mut Vector)
               -> Option<Ray> {

        match *intersection {
            Intersection::Hit { position, normal, .. } => {
                let target = position + normal + Vector::random_in_unit_sphere();
                let scattered = Ray {
                    origin: position,
                    direction: target - position,
                };

                *attenuation = self.albedo;

                Some(scattered)
            }
            _ => None,
        }

    }
}
