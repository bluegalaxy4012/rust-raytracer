pub mod point;
pub mod render;
pub mod scenedata;
pub mod vector3;

use image::{DynamicImage, GenericImage, Rgba};
use point::Point;
use render::{Intersectable, Ray};
use scenedata::{Color, Cube, Element, Plane, Scene, Sphere};
use vector3::Vector3;

#[test]
fn test_render() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec![
            Element::Sphere(Sphere {
                center: Point {
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
                center: Point {
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
            Element::Cube(Cube {
                center: Point {
                    x: -1.0,
                    y: 1.0,
                    z: -2.5,
                },
                sidelength: 1.3,
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                },
            }),
            Element::Plane(Plane {
                p: Point {
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
                    red: 0.3,
                    green: 0.0,
                    blue: 0.3,
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
    let sky = Rgba([179, 224, 242, 255]);

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
                image.put_pixel(x, y, Color::to_rgb(i.unwrap().object.color()));
            } else {
                image.put_pixel(x, y, sky);
            }
        }
    }
    image
}
