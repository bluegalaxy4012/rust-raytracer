use crate::scenedata::Cube;
use crate::scenedata::Plane;
use crate::scenedata::TextureCoords;
use crate::vector3::Vector3;
//use crate::Point;
use crate::Scene;
use crate::Sphere;

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
            origin: scene.ray_origin,
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
    fn surface_normal(&self, intersection_point: &Vector3) -> Vector3;
    fn texture_coords(&self, intersection_point: &Vector3) -> TextureCoords;
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

    fn texture_coords(&self, intersection_point: &Vector3) -> TextureCoords {
        let intersection_vec = *intersection_point - self.center;
        TextureCoords {
            x: (1.0 + (intersection_vec.z.atan2(intersection_vec.x) as f32) / std::f32::consts::PI)
                / 2.0,
            y: (intersection_vec.y / self.radius).acos() as f32 / std::f32::consts::PI,
        }
    }

    fn surface_normal(&self, intersection_point: &Vector3) -> Vector3 {
        (*intersection_point - self.center).normalize()
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

    //cum am impl si pentru sfera
    fn texture_coords(&self, intersection_point: &Vector3) -> TextureCoords {
        let half_sidelength = self.sidelength / 2.0;
        let intersection_vec = *intersection_point - self.center;
        let abs_x = intersection_vec.x.abs();
        let abs_y = intersection_vec.y.abs();
        let abs_z = intersection_vec.z.abs();

        let face = if abs_x > abs_y && abs_x > abs_z {
            if intersection_vec.x > 0.0 {
                0
            } else {
                1
            }
        } else if abs_y > abs_z {
            if intersection_vec.y > 0.0 {
                2
            } else {
                3
            }
        } else if intersection_vec.z > 0.0 {
            4
        } else {
            5
        };

        let mut texture_coords = TextureCoords { x: 0.0, y: 0.0 };

        match face {
            0 => {
                texture_coords.x =
                    ((intersection_vec.z + half_sidelength) / self.sidelength) as f32;
                texture_coords.y =
                    ((intersection_vec.y + half_sidelength) / self.sidelength) as f32;
            }
            1 => {
                texture_coords.x =
                    ((intersection_vec.z + half_sidelength) / self.sidelength) as f32;
                texture_coords.y =
                    ((intersection_vec.y + half_sidelength) / self.sidelength) as f32;
            }
            2 => {
                texture_coords.x =
                    ((intersection_vec.x + half_sidelength) / self.sidelength) as f32;
                texture_coords.y =
                    ((intersection_vec.z + half_sidelength) / self.sidelength) as f32;
            }
            3 => {
                texture_coords.x =
                    ((intersection_vec.x + half_sidelength) / self.sidelength) as f32;
                texture_coords.y =
                    ((intersection_vec.z + half_sidelength) / self.sidelength) as f32;
            }
            4 => {
                texture_coords.x =
                    ((intersection_vec.x + half_sidelength) / self.sidelength) as f32;
                texture_coords.y =
                    ((intersection_vec.y + half_sidelength) / self.sidelength) as f32;
            }
            5 => {
                texture_coords.x =
                    ((intersection_vec.x + half_sidelength) / self.sidelength) as f32;
                texture_coords.y =
                    ((intersection_vec.y + half_sidelength) / self.sidelength) as f32;
            }
            _default => {}
        }
        texture_coords
    }

    fn surface_normal(&self, intersection_point: &Vector3) -> Vector3 {
        let half_sidelength = self.sidelength / 2.0;
        let min_x = self.center.x - half_sidelength;
        let max_x = self.center.x + half_sidelength;
        let min_y = self.center.y - half_sidelength;
        let max_y = self.center.y + half_sidelength;
        let min_z = self.center.z - half_sidelength;
        let max_z = self.center.z + half_sidelength;

        let d_min_x = (intersection_point.x - min_x).abs();
        let d_max_x = (intersection_point.x - max_x).abs();
        let d_min_y = (intersection_point.y - min_y).abs();
        let d_max_y = (intersection_point.y - max_y).abs();
        let d_min_z = (intersection_point.z - min_z).abs();
        let d_max_z = (intersection_point.z - max_z).abs();

        let min_d = d_min_x
            .min(d_max_x)
            .min(d_min_y.min(d_max_y))
            .min(d_min_z.min(d_max_z));

        if min_d == d_min_x {
            Vector3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }
        } else if min_d == d_max_x {
            Vector3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            }
        } else if min_d == d_min_y {
            Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            }
        } else if min_d == d_max_y {
            Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            }
        } else if min_d == d_min_z {
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        } else {
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        }
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

    fn texture_coords(&self, intersection_point: &Vector3) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });

        if x_axis.norm() < 0.01 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }

        let y_axis = self.normal.cross(&x_axis);

        let intersection_vec = *intersection_point - self.p;
        TextureCoords {
            x: intersection_vec.dot(&x_axis) as f32,
            y: intersection_vec.dot(&y_axis) as f32,
        }
    }

    fn surface_normal(&self, _intersection_point: &Vector3) -> Vector3 {
        -self.normal
    }
}
