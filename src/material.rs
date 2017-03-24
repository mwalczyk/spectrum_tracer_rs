use vector::Vector;
use ray::Ray;
use shape::DifferentialGeometry;

extern crate rand;
use rand::Rng;

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

impl Lambertian {
    pub fn new(a: &Vector) -> Lambertian {
        Lambertian { albedo: *a }
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

impl Metallic {
    pub fn new(a: &Vector, g: f64) -> Metallic {
        Metallic {
            albedo: *a,
            glossiness: g.min(1.0).max(0.0),
        }
    }
}

pub struct Dielectric {
    pub ior: f64,
}

impl Material for Dielectric {
    fn scatter(&self,
               incident: &Ray,
               intersection: &DifferentialGeometry,
               attenuation: &mut Vector)
               -> Ray {

        // The index of refraction (IOR) of a particular medium is defined
        // as the speed of light in a vacuum divided by the speed of light
        // in the medium: n = c / v
        //
        // Snell's law states: n_i * sin(theta_i) = n_t * sin(theta_t)
        // So, sin(theta_t) = (n_i / n_t) * sin(theta_i)
        let mut ior = self.ior;

        // R0 is the probability of reflection at normal incidence, which
        // is given by the equation:
        //              r0 = ((n1 - n2) / (n1 + n2))^2
        // Air in a vacuum has an IOR of 1.0, which is n1 in the equation
        // above
        let mut r0 = (1.0 - ior) / (1.0 + ior);
        r0 = r0 * r0;

        // Check if the incident ray is inside of the medium, in which case
        // flip the normal
        let mut outward_normal = intersection.normal;
        if incident.direction.dot(&outward_normal) > 0.0 {
            outward_normal *= -1.0;
            ior = 1.0 / ior;
        }
        ior = 1.0 / ior;

        // Calculate angles
        let cos_theta_i = incident.direction.dot(&outward_normal) * -1.0;
        let cos_theta_t = 1.0 - ior * ior * (1.0 - cos_theta_i * cos_theta_i);

        // Schlick's approximation
        let probability_of_reflection = r0 + (1.0 - r0) * (1.0 - cos_theta_i).powf(5.0);
        let mut rng = rand::thread_rng();
        let mut scattered: Vector;
        if cos_theta_t > 0.0 && rng.next_f64() > probability_of_reflection {
            // Refract
            scattered = ((incident.direction * ior) +
                         (outward_normal * (ior * cos_theta_i - cos_theta_t.sqrt())));
        } else {
            // Reflect
            scattered = incident.direction.reflect(&outward_normal);
        }

        *attenuation = Vector::one();
        let refracted = incident.direction.refract(&intersection.normal);
        Ray::new(&intersection.position,
                 &scattered,
                 incident.t_min,
                 incident.t_max)
    }
}

impl Dielectric {
    pub fn new(i: f64) -> Dielectric {
        Dielectric { ior: i }
    }
}
