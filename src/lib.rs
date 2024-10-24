pub mod render;
pub mod scenedata;
pub mod vector3;

use image::{DynamicImage, GenericImage, Rgba};
use render::Ray;
use scenedata::{Color, Cube, DirectionalLight, Element, Light, Plane, PointLight, Scene, Sphere};
use vector3::Vector3;

#[test]
fn test_render() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        lights: vec![
            /*
                        Light::Directional(DirectionalLight {
                            direction: Vector3 {
                                x: -0.25,
                                y: -2.0,
                                z: -1.0,
                            }
                            .normalize(),
                            intensity: 1.0,
                            color: Color {
                                red: 1.0,
                                green: 1.0,
                                blue: 0.8,
                            },
                        }),
                        Light::Directional(DirectionalLight {
                            direction: Vector3 {
                                x: 0.25,
                                y: -1.5,
                                z: 0.25,
                            }
                            .normalize(),
                            intensity: 0.75,
                            color: Color {
                                red: 1.0,
                                green: 1.0,
                                blue: 0.8,
                            },
                        }),
            */
            Light::Point(PointLight {
                point: Vector3 {
                    x: 0.25,
                    y: 0.25,
                    z: -0.25,
                },
                intensity: 5.75,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.8,
                },
            }),
        ],
        objects: vec![
            Element::Sphere(Sphere {
                center: Vector3 {
                    x: 1.0,
                    y: -3.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 1.0,
                    green: 0.5,
                    blue: 0.0,
                },
            }),
            Element::Sphere(Sphere {
                center: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: -10.0,
                },
                radius: 2.0,
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
            }),
            Element::Sphere(Sphere {
                center: Vector3 {
                    x: 8.0,
                    y: 5.0,
                    z: -20.0,
                },
                radius: 2.0,
                color: Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
            }),
            Element::Cube(Cube {
                center: Vector3 {
                    x: -0.5,
                    y: -1.0,
                    z: -2.2,
                },
                sidelength: 1.3,
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
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
                color: Color {
                    red: 0.12,
                    green: 0.12,
                    blue: 0.12,
                },
            }),
        ],
    };

    let img = render(&scene); //dynamic image
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    match img.save("image.png") {
        Ok(_) => println!("succes saving image!"),
        Err(e) => eprintln!("error saving image: {}", e),
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let sky = Rgba([45, 70, 125, 255]);

    /*
    //momentan
    let color = Rgba([
        (scene.sphere.color.red * 255.0) as u8,
        (scene.sphere.color.green * 255.0) as u8,
        (scene.sphere.color.blue * 255.0) as u8,
        255,
    ]);
    */

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            let i = scene.trace(&ray);
            if i.is_some() {
                //image.put_pixel(x, y, Color::to_rgb(i.unwrap().object.color()));
                image.put_pixel(x, y, Color::to_rgb(&scene.get_color(&ray, &i.unwrap())));
            } else {
                image.put_pixel(x, y, sky);
            }
        }
    }
    image
}
