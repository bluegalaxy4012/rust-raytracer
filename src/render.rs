use crate::scenedata::Cube;
use crate::scenedata::Plane;
use crate::vector3::Vector3;
//use crate::Point;
use crate::Scene;
use crate::Sphere;

pub static RAY_ORIGIN: Vector3 = Vector3::zero();

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        //transformam coord in float-uri intre -1 si 1
        //apoi se fixeaza pentru aspect ratio si fov
        let fov_fix = (scene.fov.to_radians() / 2.0).tan(); //camera la dist 1 de sensor
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let x_sensor =
            (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_fix;
        let y_sensor = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_fix; // y poz e in jos

        Ray {
            origin: RAY_ORIGIN,
            direction: Vector3 {
                x: x_sensor,
                y: y_sensor,
                z: -1.0, // sensor square e la 1 unit de camera
            }
            .normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let v: Vector3 = self.center - ray.origin;
        //construim o latura de lungime v cos(..) ca sa formam un triunghi dreptunghic
        //ca sa calculam distanta de la centrul cercului la ray
        let cateta = v.dot(&ray.direction);
        let d = v.dot(&v) - cateta * cateta;
        if d > self.radius * self.radius {
            return None;
        }

        let cut_d = (self.radius * self.radius - d).sqrt();
        let intersection_d1 = cateta - cut_d;
        let intersection_d2 = cateta + cut_d; //triunghi isoscel

        //daca e in spatele camerei
        if intersection_d1 < 0.0 && intersection_d2 < 0.0 {
            return None;
        }

        let distance: f64;
        if intersection_d1 > intersection_d2 {
            distance = intersection_d2;
        } else {
            distance = intersection_d1;
        }

        Some(distance)

        //d < self.radius * self.radius //si = ?
    }
}

impl Intersectable for Cube {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        //e intre centru +- 1/2 sidelength
        let half_sidelength = self.sidelength / 2.0;
        let unit_raydir_x = 1.0 / ray.direction.x;
        let unit_raydir_y = 1.0 / ray.direction.y;
        let unit_raydir_z = 1.0 / ray.direction.z;

        let min_x = self.center.x - half_sidelength;
        let max_x = self.center.x + half_sidelength;
        let min_y = self.center.y - half_sidelength;
        let max_y = self.center.y + half_sidelength;
        let min_z = self.center.z - half_sidelength;
        let max_z = self.center.z + half_sidelength;

        let t_min_x = (min_x - ray.origin.x) * unit_raydir_x;
        let t_max_x = (max_x - ray.origin.x) * unit_raydir_x;

        let t_min_y = (min_y - ray.origin.y) * unit_raydir_y;
        let t_max_y = (max_y - ray.origin.y) * unit_raydir_y;

        let t_min_z = (min_z - ray.origin.z) * unit_raydir_z;
        let t_max_z = (max_z - ray.origin.z) * unit_raydir_z;

        //in caz de negative ray direction
        let t_enter = (t_min_x.min(t_max_x))
            .max(t_min_y.min(t_max_y))
            .max(t_min_z.min(t_max_z));
        let t_exit = (t_max_x.max(t_min_x))
            .min(t_max_y.max(t_min_y))
            .min(t_max_z.max(t_min_z));

        //println!("enter {} si exit {} ", t_enter, t_exit);

        //nu se intersecteaza pe camera
        if t_enter > t_exit || t_exit < 0.0 {
            return None;
        }

        Some(t_enter)
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &(self.normal);
        let denominator = normal.dot(&ray.direction);

        //abs pentru ambele fete ale planului
        if denominator.abs() > 1e-6 {
            let v = self.p - ray.origin;
            let d = v.dot(&normal) / denominator;
            if d > 0.0 {
                return Some(d);
            }
        }
        None
    }
}
