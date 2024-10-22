use crate::render::{Intersectable, Ray};
use crate::vector3::Vector3;
use crate::Point;
use image::Rgba;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Element>,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Cube {
    pub center: Point,
    pub sidelength: f64,
    pub color: Color,
}

pub struct Plane {
    pub p: Point,
    pub normal: Vector3,
    pub color: Color,
}

pub enum Element {
    Sphere(Sphere),
    Cube(Cube),
    Plane(Plane),
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
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

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &(s.color),
            Element::Cube(ref c) => &(c.color),
            Element::Plane(ref p) => &(p.color),
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
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|o| o.intersect(ray).map(|d| Intersection::new(d, o)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
