mod base;
use base::{Vector, Color, Point};
mod objets;
use objets::{Camera, Material, Plane, Sphere, Object, Light};
use image::{ImageBuffer, Rgb};
use image;

fn main() {
  let mut camera = Camera {
    xyz: Point {x: 0., y: 0., z: -1.0},
    target: Vector {x: 0., y: 0., z: 0.},
    ambient_light: Color {r: 0.6, g: 0.6, b: 0.6},
    orientation: Vector {x: 0., y: 1., z: 0.}, // y é em cima
    fov: 0.,
    image_plane_distance: 0.0,
    width: 600,
    height: 384
  };
  camera.set_fov(std::f32::consts::PI/2.0);
  camera.set_target(Point {x: 0., y: 0., z: 0.});
  show_directions(&camera);
  show_sky(&camera);
  default_scenes(&mut camera);
}
fn show_sky(camera: &Camera) {
  let pixels = camera.take_picture(&vec![], &vec![]);
  save_image(pixels, "céu.png".to_string());
}
fn show_directions(camera: &Camera) {
  let pixels = camera.get_directions();
  save_image(pixels, "directions.png".to_string());
}
fn save_image(pixels: Vec<Vec<[u8; 3]>>, name: String) {
  let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(pixels.len() as u32, pixels[0].len() as u32);
  for x in 0..pixels.len() {
    for y in 0..pixels[0].len() {
      image.get_pixel_mut(x as u32, y as u32).data = pixels[x][y]; 
    }
  }
  image.get_pixel_mut(0, 0).data = [0, 0, 255];
  image.save(name).unwrap();
}
fn default_scenes(camera: &mut Camera) {
  let green = Material {rgb:Color {r: 0.1, g: 0.9, b: 0.3}, kd:0.4, ks:0.2, kt:0.0, kr:0.0, alpha:40.0};
  let blue  = Material {rgb:Color {r: 0.3, g: 0.2, b: 0.9}, kd:0.4, ks:0.2, kt:0.0, kr:0.0, alpha:40.0};
  let red   = Material {rgb:Color {r: 0.8, g: 0.3, b: 0.2}, kd:0.5, ks:0.6, kt:0.0, kr:0.0, alpha:40.0};
  let gray  = Material {rgb:Color {r: 0.8, g: 0.8, b: 0.8}, kd:0.5, ks:0.3, kt:0.9, kr:0.8, alpha:40.0};

  let spheres:Vec<Box<Object>> = vec! [
    Box::new(Plane::from(green, Vector::from(0.0, 1.0, 0.0), &Point::from(0.0, -1.0, 0.0))),
    Box::new(Sphere::from(red, 1.0,  Point::from(-0.8,  0.0, 4.0))),
    Box::new(Sphere::from(blue, 0.6, Point::from(-2.5, -0.4, 3.0))),
    Box::new(Sphere::from(gray, 4.0, Point::from( 3.0, 3.0, 6.0)))
  ];

  let lights:Vec<Light> = vec! [
    Light::from(Point::from(3.0, 2.0, 3.0), 0.3, 0.3),
    Light::from(Point::from(-3.0, 2.0, 2.0), 0.3, 0.3)
  ];

  let pixels = camera.take_picture(&spheres, &lights);
  save_image(pixels, "cena-1.png".to_string());

  camera.xyz = Point::from(5.0, 2.0, -2.0);
  camera.set_target(Point::from(-0.8, 0.0, 4.0));

  let pixels = camera.take_picture(&spheres, &lights);
  save_image(pixels, "cena-2.png".to_string());

  camera.set_image_plane_distance(4.0);
  save_image(camera.take_picture(&spheres, &lights), "cena-3.png".to_string());

  let length = camera.focal_length_for_point(&Point::from(-0.8,  0.0, 4.0));
  save_image(camera.take_dof_picture(&spheres, &lights, 0.1, length, 8), "cena-4.png".to_string());
  
  camera.set_image_plane_distance(4.0);
  camera.xyz = Point::from(10.8, 4.0, -8.0);
  save_image(camera.take_picture(&spheres, &lights), "cena-5.png".to_string());
}
