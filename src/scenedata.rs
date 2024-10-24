use std::ops::{Add, Mul};

use crate::render::{Intersectable, Ray};
use crate::vector3::Vector3;
use image::Rgba;

pub const AMBIENT_LIGHT_INTENSITY: f32 = 0.075;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub lights: Vec<Light>,
    pub objects: Vec<Element>,
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct PointLight {
    pub point: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub color: Color,
}

pub struct Cube {
    pub center: Vector3,
    pub sidelength: f64,
    pub color: Color,
}

pub struct Plane {
    pub p: Vector3,
    pub normal: Vector3,
    pub color: Color,
}

pub enum Element {
    Sphere(Sphere),
    Cube(Cube),
    Plane(Plane),
}

#[derive(Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Color {
    pub fn to_rgb(&self) -> Rgba<u8> {
        Rgba([
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
            255,
        ])
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.clamp(0.0, 1.0),
            green: self.green.clamp(0.0, 1.0),
            blue: self.blue.clamp(0.0, 1.0),
        }
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Element) -> Intersection<'b> {
        Intersection { distance, object }
    }
}

impl Light {
    pub fn intensity(&self, intersection_point: &Vector3) -> f32 {
        match *self {
            Light::Directional(ref dlight) => dlight.intensity + AMBIENT_LIGHT_INTENSITY,
            Light::Point(ref plight) => {
                plight.intensity
                    / (4.0
                        * ::std::f32::consts::PI
                        * ((plight.point - *intersection_point).norm() as f32))
                    + AMBIENT_LIGHT_INTENSITY
            }
        }
    }

    pub fn color(&self) -> &Color {
        match *self {
            Light::Directional(ref dlight) => &dlight.color,
            Light::Point(ref plight) => &plight.color,
        }
    }

    pub fn dir_to_light(&self, intersection_point: &Vector3) -> Vector3 {
        match *self {
            Light::Directional(ref dlight) => -dlight.direction.normalize(),
            Light::Point(ref plight) => (plight.point - *intersection_point).normalize(),
        }
    }

    pub fn lit(
        &self,
        traced_shadow_checker: &Option<Intersection>,
        intersection_point: &Vector3,
    ) -> bool {
        match *self {
            Light::Directional(ref _dlight) => traced_shadow_checker.is_none(),
            Light::Point(ref plight) => {
                traced_shadow_checker.is_none()
                    || traced_shadow_checker.as_ref().unwrap().distance
                        > (plight.point - *intersection_point).norm()
            }
        }
    }
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &(s.color),
            Element::Cube(ref c) => &(c.color),
            Element::Plane(ref p) => &(p.color),
        }
    }

    pub fn surface_normal(&self, hit_point: &Vector3) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => (*hit_point - s.center).normalize(),
            Element::Cube(ref c) => {
                let half_sidelength = c.sidelength / 2.0;
                let min_x = c.center.x - half_sidelength;
                let max_x = c.center.x + half_sidelength;
                let min_y = c.center.y - half_sidelength;
                let max_y = c.center.y + half_sidelength;
                let min_z = c.center.z - half_sidelength;
                let max_z = c.center.z + half_sidelength;

                let d_min_x = (hit_point.x - min_x).abs();
                let d_max_x = (hit_point.x - max_x).abs();
                let d_min_y = (hit_point.y - min_y).abs();
                let d_max_y = (hit_point.y - max_y).abs();
                let d_min_z = (hit_point.z - min_z).abs();
                let d_max_z = (hit_point.z - max_z).abs();

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
            Element::Plane(ref p) => -p.normal,
        }
    }

    pub fn albedo(&self) -> f32 {
        1.0
    }
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Cube(ref c) => c.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|o| o.intersect(ray).map(|d| Intersection::new(d, o)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }

    pub fn get_color(&self, ray: &Ray, intersection: &Intersection) -> Color {
        let intersection_point: Vector3 = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.object.surface_normal(&intersection_point);

        let mut color = Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };
        for light in &self.lights {
            let dir_to_light = light.dir_to_light(&intersection_point);
            //shadow acne, nudge ca sa nu trasam din interiorul obiectului afara
            let outside_intersection_point = intersection_point + (surface_normal * 1e-7);
            let shadow_checker = Ray {
                origin: outside_intersection_point,
                direction: dir_to_light,
            };

            let lit = light.lit(&self.trace(&shadow_checker), &intersection_point);
            let light_intensity = if !lit {
                AMBIENT_LIGHT_INTENSITY
            } else {
                (surface_normal.dot(&dir_to_light) as f32).max(0.0)
                    * light.intensity(&intersection_point)
            };
            let light_reflected = intersection.object.albedo() / std::f32::consts::PI;

            //println!("int {:?} refl {:?}", light_intensity, light_reflected);

            color = color
                + intersection.object.color().clone()
                    * light.color().clone()
                    * light_intensity
                    * light_reflected;
        }
        color.clamp()
    }
}
