use crate::render::Ray;
use crate::scenedata::{self, Color, Element, Light, Scene};
use crate::vector3::Vector3;
use image::{DynamicImage, GenericImage, Rgba};
use serde_json;
use std::fs;

pub struct SceneManager {
    pub scene: Scene,
}

impl SceneManager {
    pub fn new_empty(width: u32, height: u32, fov: f64, ray_origin: Vector3) -> SceneManager {
        SceneManager {
            scene: Scene {
                width,
                height,
                fov,
                ray_origin,
                lights: Vec::new(),
                objects: Vec::new(),
            },
        }
    }

    pub fn new(
        width: u32,
        height: u32,
        fov: f64,
        ray_origin: Vector3,
        lights: Vec<Light>,
        objects: Vec<Element>,
    ) -> SceneManager {
        SceneManager {
            scene: Scene {
                width,
                height,
                fov,
                ray_origin,
                lights,
                objects,
            },
        }
    }

    pub fn new_from_scene(scene: Scene) -> SceneManager {
        SceneManager { scene }
    }

    pub fn set_ray_origin(&mut self, origin: Vector3) {
        self.scene.ray_origin = origin;
    }

    pub fn add_light(&mut self, light: Light) {
        self.scene.lights.push(light);
    }

    pub fn remove_light(&mut self, index: usize) {
        self.scene.lights.remove(index);
    }

    pub fn add_object(&mut self, object: Element) {
        self.scene.objects.push(object);
    }

    pub fn remove_object(&mut self, index: usize) {
        self.scene.objects.remove(index);
    }

    pub fn save_to_json(&mut self, file_path: &str) {
        let json_data =
            serde_json::to_string_pretty(&self.scene).expect("Failed to serialize scene");
        fs::write(file_path, json_data).expect("Failed to write JSON file");
    }

    pub fn load_from_json(file_path: &str) -> Result<SceneManager, serde_json::Error> {
        let json_data = fs::read_to_string(file_path).expect("Failed to read JSON file");
        println!("loading json scene");
        let scenetest: Scene = serde_json::from_str(&json_data).expect("Failed to parse JSON coi");
        let scene: Scene = serde_json::from_str(&json_data)?;
        Ok(SceneManager { scene })
    }

    pub fn render(&self) -> DynamicImage {
        let mut img = DynamicImage::new_rgb8(self.scene.width, self.scene.height);
        let black = Rgba([0, 0, 0, 255]);

        for x in 0..self.scene.width {
            for y in 0..self.scene.height {
                let ray = Ray::create_prime(x, y, &self.scene);
                let i = self.scene.trace(&ray);
                if let Some(intersection) = i {
                    img.put_pixel(
                        x,
                        y,
                        Color::to_rgb(&self.scene.get_color(&ray, &intersection)),
                    );
                } else {
                    img.put_pixel(x, y, black);
                }
            }
        }
        img
    }
}
