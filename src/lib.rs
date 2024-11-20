pub mod render;
pub mod scenedata;
pub mod scenemanager;
pub mod vector3;

use image::{DynamicImage, GenericImage, Rgba};
use minifb::{Key, Window, WindowOptions};
use render::Ray;
use scenedata::{
    Color, Coloration, Cube, Element, Light, Material, Plane, PointLight, Scene, Sphere, Texture,
};
use scenemanager::SceneManager;
use std::path::PathBuf;
use vector3::Vector3;

#[test]
fn test_render() {
    //in the future , interactable scene manager , moving camera etc

    let rainbowtexture = Texture {
        path: PathBuf::from("resources/rainbowtexture.png"),
        texture: image::open("resources/rainbowtexture.png").expect("failed to open texture"),
    };
    /*
        let scene = Scene {
            width: 1200,
            height: 900,
            fov: 100.0,
            ray_origin: Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            lights: vec![Light::Point(PointLight {
                point: Vector3 {
                    x: 1.00,
                    y: 0.25,
                    z: 0.25,
                },
                intensity: 160.75,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.8,
                },
            })],
            objects: vec![
                Element::Sphere(Sphere {
                    center: Vector3 {
                        x: 1.0,
                        y: -3.0,
                        z: -5.0,
                    },
                    radius: 1.0,
                    material: Material {
                        coloration: Coloration::Color(Color {
                            red: 1.0,
                            green: 0.5,
                            blue: 0.0,
                        }),
                        albedo: 1.00,
                    },
                }),
                Element::Sphere(Sphere {
                    center: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: -10.0,
                    },
                    radius: 2.0,
                    material: Material {
                        coloration: Coloration::Color(Color {
                            red: 1.0,
                            green: 0.0,
                            blue: 0.0,
                        }),
                        albedo: 0.18,
                    },
                }),
                Element::Sphere(Sphere {
                    center: Vector3 {
                        x: 8.0,
                        y: 5.0,
                        z: -15.0,
                    },
                    radius: 2.0,
                    material: Material {
                        coloration: Coloration::Color(Color {
                            red: 0.0,
                            green: 1.0,
                            blue: 0.0,
                        }),
                        albedo: 0.98,
                    },
                }),
                Element::Cube(Cube {
                    center: Vector3 {
                        x: -0.5,
                        y: -1.0,
                        z: -2.2,
                    },
                    sidelength: 1.3,
                    material: Material {
                        /*                    color: Color {
                                                red: 0.0,
                                                green: 0.0,
                                                blue: 1.0,
                        */
                        coloration: Coloration::Texture(rainbowtexture),
                        albedo: 1.0,
                    },
                }),
                Element::Plane(Plane {
                    p: Vector3 {
                        x: 0.0,
                        y: -3.0,
                        z: -4.0,
                    },
                    normal: Vector3 {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    material: Material {
                        coloration: Coloration::Color(Color {
                            red: 0.12,
                            green: 0.12,
                            blue: 0.12,
                        }),
                        albedo: 1.0,
                    },
                }),
                Element::Plane(Plane {
                    p: Vector3 {
                        x: 0.0,
                        y: -3.0,
                        z: -20.0,
                    },
                    normal: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
                    material: Material {
                        coloration: Coloration::Color(Color {
                            red: 0.17,
                            green: 0.36,
                            blue: 0.5,
                        }),
                        albedo: 1.0,
                    },
                }),
            ],
        };
    */
    //make the light/objects vector as above
    //
    let lights = vec![Light::Point(PointLight {
        point: Vector3 {
            x: 1.00,
            y: 0.25,
            z: 0.25,
        },
        intensity: 160.75,
        color: Color {
            red: 1.0,
            green: 1.0,
            blue: 0.8,
        },
    })];

    let objects = vec![
        Element::Sphere(Sphere {
            center: Vector3 {
                x: 1.0,
                y: -3.0,
                z: -5.0,
            },
            radius: 1.0,
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 1.0,
                    green: 0.5,
                    blue: 0.0,
                }),
                albedo: 1.00,
            },
        }),
        Element::Sphere(Sphere {
            center: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -10.0,
            },
            radius: 2.0,
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                }),
                albedo: 0.18,
            },
        }),
        Element::Sphere(Sphere {
            center: Vector3 {
                x: 8.0,
                y: 5.0,
                z: -15.0,
            },
            radius: 2.0,
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                }),
                albedo: 0.98,
            },
        }),
        Element::Cube(Cube {
            center: Vector3 {
                x: -0.5,
                y: -1.0,
                z: -2.2,
            },
            sidelength: 1.3,
            material: Material {
                coloration: Coloration::Texture(rainbowtexture),
                albedo: 1.0,
            },
        }),
        Element::Plane(Plane {
            p: Vector3 {
                x: 0.0,
                y: -3.0,
                z: -4.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 0.12,
                    green: 0.12,
                    blue: 0.12,
                }),
                albedo: 1.0,
            },
        }),
        Element::Plane(Plane {
            p: Vector3 {
                x: 0.0,
                y: -3.0,
                z: -20.0,
            },
            normal: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            material: Material {
                coloration: Coloration::Color(Color {
                    red: 0.17,
                    green: 0.36,
                    blue: 0.5,
                }),
                albedo: 1.0,
            },
        }),
    ];

    let mut scenemgr = SceneManager::new(
        1200,
        900,
        100.0,
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        lights,
        objects,
    );
    scenemgr.save_to_json("resources/scene1.json");

    //let scenemgr =
    //    SceneManager::load_from_json("resources/scene1.json").expect("failed to load scene");

    //save scene to json
    let scene = scenemgr.scene;
    let img = render(&scene);

    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    let mut buffer: Vec<u32>; // = image_to_buffer(&img);

    let mut window = Window::new(
        "Scene",
        scene.width as usize,
        scene.height as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed: {}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer = image_to_buffer(&render(&scene));
        window
            .update_with_buffer(&buffer, scene.width as usize, scene.height as usize)
            .unwrap();

        //handle_input(&mut window, &scenemgr);
    }
}

fn handle_input(window: &mut Window, scenemgr: &mut SceneManager) {
    if window.is_key_down(Key::W) {
        scenemgr.scene.ray_origin.z += 0.1;
    }
}

fn image_to_buffer(img: &DynamicImage) -> Vec<u32> {
    let rgb_image = img.to_rgb8();
    let mut buffer = vec![0; (rgb_image.width() * rgb_image.height()) as usize];

    for (x, y, pixel) in rgb_image.enumerate_pixels() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;

        buffer[(y * rgb_image.width() + x) as usize] = (r << 16) | (g << 8) | b;
    }
    buffer
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba([0, 0, 0, 255]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let i = scene.trace(&ray);
            if let Some(intersection) = i {
                image.put_pixel(x, y, Color::to_rgb(&scene.get_color(&ray, &intersection)));
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}
