use vector::Vector;
use ray::Ray;
use shape::DifferentialGeometry;

pub trait Material: Sync + Send {
    // produce a scattered ray unless the incident
    // ray is absorbed, in which case None is returned
    fn scatter(&self,
               incident: &Ray,
               intersection: &DifferentialGeometry,
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
               intersection: &DifferentialGeometry,
               attenuation: &mut Vector)
               -> Option<Ray> {

        let target = intersection.position + intersection.normal + Vector::random_in_unit_sphere();
        let scattered = Ray::new(&intersection.position,
                                 &mut (target - intersection.position));

        *attenuation = self.albedo;

        Some(scattered)
    }
}

pub struct Metallic {
    pub albedo: Vector,
    pub glossiness: f64,
}

impl Material for Metallic {
    fn scatter(&self,
               incident: &Ray,
               intersection: &DifferentialGeometry,
               attenuation: &mut Vector)
               -> Option<Ray> {

        let reflected = incident.direction.normalize().reflect(&intersection.normal);
        let scattered = Ray::new(&intersection.position,
                                 &(reflected + Vector::random_in_unit_sphere() * self.glossiness));

        *attenuation = self.albedo;

        // if scattered.direction.dot(&normal) > 0.0 {
        return Some(scattered);
        //}Noneintersection

    }
}
