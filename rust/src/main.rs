mod base;
use base::{Vector, Color, Point};
mod objets;
use objets::{Camera, Material, Plane, Sphere, Object, Light};
use image::{ImageBuffer, Rgb};
use image;
mod config;
use std::process::exit;
use std::fs;

fn main() {
  let contents = fs::read_to_string("scenes.json").unwrap_or_else(|err| {
    eprintln!("Erro ao ler arquivo de cenas: {}", err);
    exit(1);
  });
  let scenes = config::parse(contents).unwrap_or_else(|e| {
    eprintln!("Erro no arquivo de cenas: {}", e);
    exit(1);
  });
  for scene in scenes {
    render_scene(scene);
  }
}
fn render_scene(scene:config::Scene) {
  println!("Renderizando cena {}", scene.name);
  let mut camera = Camera {
    xyz: Point::from_array(&scene.camera.position),
    target: Vector::from(0.0, 0.0, 0.0),
    ambient_light: Color::from_array(&scene.camera.ambient_light),
    orientation: Vector::from_array(&scene.camera.orientation),
    fov: 0.0,
    image_plane_distance: 0.0,
    width: scene.camera.width,
    height: scene.camera.height,
    background: scene.background
  };
  camera.set_target(Point::from_array(&scene.camera.target));
  if scene.camera.fov > 0.0 {
    camera.set_fov(scene.camera.fov);
    if scene.camera.image_plane_distance > 0.0 {
      println!("  Distancia pro plano de fundo foi ignorada em favor do FoV");
      println!("  Para me fazer usar a distância especificada, faça FoV = 0");
    }
  }
  else if scene.camera.image_plane_distance > 0.0 {
    camera.set_image_plane_distance(scene.camera.image_plane_distance);
  }
  else {
    eprintln!("Erro: A distancia do plano de fundo ou o FoV precisa ser maior que zero.");
    exit(2);
  }

  let mut objects:Vec<Box<Object>> = Vec::new();
  let mut lights:Vec<Light> = Vec::new();

  for object in scene.objects {
    match object.kind {
      config::ObjectType::Plane => {
        let material = Material {
          rgb: Color::from_array(&object.material.rgb),
          kd: object.material.kd,
          ks: object.material.ks,
          kt: object.material.kt,
          kr: object.material.kr,
          alpha: object.material.alpha
        };
        let obj = Plane::from(material, Vector::from_array(&object.normal), &Point::from_array(&object.point));
        objects.push(Box::new(obj));
      },
      config::ObjectType::Sphere => {
        let material = Material {
          rgb: Color::from_array(&object.material.rgb),
          kd: object.material.kd,
          ks: object.material.ks,
          kt: object.material.kt,
          kr: object.material.kr,
          alpha: object.material.alpha
        };
        let obj = Sphere::from(material, object.r, Point::from_array(&object.center));
        objects.push(Box::new(obj));
      }
    }
  }
  for light in scene.lights {
    lights.push(Light {
      xyz: Point::from_array(&light.position),
      difuse: Color::from_array(&light.difuse),
      specular: Color::from_array(&light.specular)
    })
  }
  
  let filename = (scene.name+".png").to_string();
  let mut pixels:Vec<Vec<[u8; 3]>>;
  if scene.dof.active {
    pixels = camera.take_dof_picture(&objects, &lights, scene.dof.r, scene.dof.focal_length, scene.dof.samples);
  }
  else {
    pixels = camera.take_picture(&objects, &lights);
  }
  save_image(pixels, &filename);
  println!("Salvo em {}", filename);
}
fn save_image(pixels: Vec<Vec<[u8; 3]>>, name: &String) {
  let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(pixels.len() as u32, pixels[0].len() as u32);
  for x in 0..pixels.len() {
    for y in 0..pixels[0].len() {
      image.get_pixel_mut(x as u32, y as u32).data = pixels[x][y]; 
    }
  }
  image.get_pixel_mut(0, 0).data = [0, 0, 255];
  image.save(name).unwrap();
}