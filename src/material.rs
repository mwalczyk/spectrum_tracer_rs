use vector::Vector;
use ray::Ray;
use shape::DifferentialGeometry;

pub trait Material: Sync + Send {
    // Produce a scattered ray
    fn scatter(&self,
               incident: &Ray,
               intersection: &DifferentialGeometry,
               attenuation: &mut Vector)
               -> Ray;
}

pub struct Lambertian {
    pub albedo: Vector,
}

impl Material for Lambertian {
    fn scatter(&self,
               incident: &Ray,
               intersection: &DifferentialGeometry,
               attenuation: &mut Vector)
               -> Ray {

        let target = intersection.position + intersection.normal + Vector::random_in_unit_sphere();
        let scattered = Ray::new(&intersection.position,
                                 &mut (target - intersection.position),
                                 incident.t_min,
                                 incident.t_max);

        *attenuation = self.albedo;
        scattered
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
               -> Ray {

        let reflected = incident.direction.normalize().reflect(&intersection.normal);
        let scattered = Ray::new(&intersection.position,
                                 &(reflected + Vector::random_in_unit_sphere() * self.glossiness),
                                 incident.t_min,
                                 incident.t_max);

        *attenuation = self.albedo;
        scattered
    }
}
