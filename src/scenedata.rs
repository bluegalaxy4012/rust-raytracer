use crate::render::{Intersectable, Ray};
use crate::vector3::Vector3;
use image::{open, DynamicImage, GenericImageView, Rgba};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::ops::{Add, Mul};
use std::path::PathBuf;

pub const AMBIENT_LIGHT_INTENSITY: f32 = 0.075;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub ray_origin: Vector3,
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub lights: Vec<Light>,
    pub objects: Vec<Element>,
}

#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PointLight {
    pub point: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Serialize, Deserialize)]
pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
}

#[derive(Serialize, Deserialize)]
pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Texture {
    pub path: PathBuf,

    #[serde(skip_serializing, skip_deserializing, default = "default_texture")]
    pub texture: DynamicImage,
}

fn default_texture() -> DynamicImage {
    DynamicImage::new_rgb8(0, 0)
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub enum Coloration {
    Color(Color),
    Texture(#[serde(deserialize_with = "load_texture")] Texture),
}

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub struct Cube {
    pub center: Vector3,
    pub sidelength: f64,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub p: Vector3,
    pub normal: Vector3,
    pub material: Material,
}

#[derive(Serialize, Deserialize)]
pub enum Element {
    Sphere(Sphere),
    Cube(Cube),
    Plane(Plane),
}

#[derive(Clone, Serialize, Deserialize)]
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

    pub fn from_rgb(rgba: Rgba<u8>) -> Color {
        Color {
            red: rgba[0] as f32 / 255.0,
            green: rgba[1] as f32 / 255.0,
            blue: rgba[2] as f32 / 255.0,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.clamp(0.0, 1.0),
            green: self.green.clamp(0.0, 1.0),
            blue: self.blue.clamp(0.0, 1.0),
        }
    }
}

fn wrap(val: f32, max: u32) -> u32 {
    let signed_max = max as i32;
    let wrapped_coord = (val * max as f32) as i32 % signed_max;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_max) as u32
    } else {
        wrapped_coord as u32
    }
}

fn load_texture<'de, D>(deserializer: D) -> Result<Texture, D::Error>
where
    D: Deserializer<'de>,
{
    let path: PathBuf = PathBuf::deserialize(deserializer)?;

    match open(&path) {
        Ok(img) => Ok(Texture { path, texture: img }),
        Err(err) => Err(de::Error::custom(format!(
            "Unable to open texture file {:?}: {}",
            path, err
        ))),
    }
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match *self {
            Coloration::Color(ref c) => c.clone(),
            Coloration::Texture(ref tex) => {
                let x = wrap(texture_coords.x, tex.texture.width());
                let y = wrap(texture_coords.y, tex.texture.height());
                Color::from_rgb(tex.texture.get_pixel(x, y))
            } /* Color::from_rgb(tex.get_pixel(
                  wrap(texture_coords.x, tex.width()),
                  wrap(texture_coords.y, tex.height()),
              )), */
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
    /*
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &(s.material.color),
            Element::Cube(ref c) => &(c.material.color),
            Element::Plane(ref p) => &(p.material.color),
        }
    }


    pub fn albedo(&self) -> f32 {
        1.0
    }
    */
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match *self {
            Element::Sphere(ref s) => s.material.coloration.color(texture_coords),
            Element::Cube(ref c) => c.material.coloration.color(texture_coords),
            Element::Plane(ref p) => p.material.coloration.color(texture_coords),
        }
    }

    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Cube(ref c) => c.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
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

    fn texture_coords(&self, intersection_point: &Vector3) -> TextureCoords {
        match *self {
            Element::Sphere(ref s) => s.texture_coords(intersection_point),
            Element::Cube(ref c) => c.texture_coords(intersection_point),
            Element::Plane(ref p) => p.texture_coords(intersection_point),
        }
    }

    fn surface_normal(&self, intersection_point: &Vector3) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(intersection_point),
            Element::Cube(ref c) => c.surface_normal(intersection_point),
            Element::Plane(ref p) => p.surface_normal(intersection_point),
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
        let texture_coords = intersection.object.texture_coords(&intersection_point);

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
                + intersection.object.color(&texture_coords).clone()
                    * light.color().clone()
                    * light_intensity
                    * light_reflected;
        }
        color.clamp()
    }
}
