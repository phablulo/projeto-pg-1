use super::base::{Vector,Point,Color};
use rand::Rng;
pub use super::config::Background;

pub struct Ray {
  origin: Point,
  direction: Vector
}
impl Ray {
  pub fn point_at(&self, t:f32) -> Point {
    &self.origin + &self.direction*t
  }
}

pub struct Material {
  pub rgb: Color,
  pub kd: f32, // fator de difusão
  pub ks: f32, // fator especular
  pub kt: f32, // fator de transmissão
  pub kr: f32, // fator reflexivo (ar = 1, vidro = 1.25)
  pub alpha: f32 // rugosidade
  // faltando o ke. O que ele faz?
}

pub trait Object {
  fn ray_intersection_distance(&self, ray: &Ray) -> Option<f32>;
  fn normal_at_point(&self, point: &Point) -> Vector;
  fn material(&self) -> &Material;
}
pub struct Sphere {
  material: Material,
  r: f32,
  xyz: Point
}
impl Sphere {
  pub fn from(material: Material, r: f32, xyz: Point) -> Sphere {
    Sphere {
      material, r, xyz
    }
  }
}
impl Object for Sphere {
  fn ray_intersection_distance(&self, ray: &Ray) -> Option<f32> {
    let tmp = &ray.origin - &self.xyz;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0*tmp.dot(&ray.direction);
    let c = tmp.dot(&tmp) - self.r*self.r;
    // encontra as soluções
    // Mas soluções negativas significam que a intersecção
    // está atrás de onde a câmera está olhando. Portanto,
    // vamos retornar apenas soluções positivas.
    let delta = b*b - 4.0*a*c;
    if delta < 0.0 {
      return None;
    }
    if delta == 0.0 {
      if a > 0.0 {
        return None;
      }
      return Some(-b/(2.0*a));
    }
    let delta = delta.sqrt();
    let a2 = 2.0*a;
    let t = (-b - delta)/a2;
    if t < 0.0 {
      let t = (-b + delta)/a2;
      if t < 0.0 {
        return None;
      }
      return Some(t);
    }
    return Some(t);
  }
  fn normal_at_point(&self, point: &Point) -> Vector {
    (point - &self.xyz).normalized()
  }
  fn material(&self) -> &Material {
    &self.material
  }
}
pub struct Plane {
  material: Material,
  normal: Vector,
  d: f32
}
impl Plane {
  pub fn from(material: Material, normal: Vector, point: &Point) -> Plane {
    Plane {
      material,
      d: normal.dot_point(point),
      normal
    }
  }
}
impl Object for Plane {
  fn ray_intersection_distance(&self, ray: &Ray) -> Option<f32> {
    let d = self.d - self.normal.dot_point(&ray.origin);
    let s = self.normal.dot(&ray.direction);
    if s == 0.0 {
      return None;
    }
    let t1 = s < 0.0;
    let t2 = d < 0.0;
    if t1 ^ t2 { // sinais opostos
      return None;
    }
    return Some(d / s);
  }
  fn normal_at_point(&self, _point: &Point) -> Vector {
    self.normal.clone()
  }
  fn material(&self) -> &Material {
    &self.material
  }
}

pub struct Light {
  pub xyz: Point,
  pub difuse: Color,
  pub specular: Color
}
/* never used
impl Light {
  pub fn from(xyz:Point, difuse: f32, specular: f32) -> Light {
    Light {
      xyz,
      difuse: Color{r: difuse, g: difuse, b: difuse},
      specular: Color{r: specular, g: specular, b: specular}
    }
  }
}
*/

// ---
#[derive(Debug)]
struct Bounds {
  tl: Point,
  tr: Point,
  bl: Point,
  br: Point
}
pub struct Camera {
  pub xyz: Point,
  pub target: Vector,
  pub ambient_light: Color,
  pub orientation: Vector,
  pub fov: f32,
  pub image_plane_distance: f32,
  pub width: i32,
  pub height: i32,
  pub background: Background
}
impl Camera {
  pub fn set_target(&mut self, target: Point) {
    self.target = (target - &self.xyz).normalized();
  }
  pub fn set_fov(&mut self, fov: f32) {
    self.fov = fov;
    self.image_plane_distance = 1.0/(fov / 2.0).tan();
  }
  pub fn set_image_plane_distance(&mut self, dist: f32) {
    self.image_plane_distance = dist;
    self.fov = 2.0 * (1.0 / dist).atan();
  }
  fn left_orientation(&self) -> Vector {
    self.orientation.rotate_along(&self.target, -std::f32::consts::PI/2.0)
  }
  fn image_plane_bounds(&self) -> Bounds {
    let ratio = (self.height as f32) / (self.width as f32);
    let w = 2.0; // sempre 2
    let h = w * ratio;
    // encontra o centro do plano de imagem
    let center = &self.xyz + self.image_plane_distance*&self.target;
    let vert = &self.orientation*(h/2.0);
    let hori = self.left_orientation()*(w/2.0);

    Bounds {
      tl: &center + ( &vert - &hori),
      tr: &center + ( &vert + &hori),
      bl: &center + (-&vert - &hori),
      br: &center + (-&vert + &hori)
    }
  }
  fn ray_for_pixel(&self, x: f32, y: f32, bounds: &Bounds) -> Ray {
    let xt = x / (self.width as f32);
    let yt = y / (self.height as f32);
    
    let top    = (1.0 - xt)*&bounds.tl + xt*&bounds.tr;
    let bottom = (1.0 - xt)*&bounds.bl + xt*&bounds.br;
    let point  = (1.0 - yt)*top + yt*bottom; // é um vetor!
    let point  = point.as_point();

    Ray {
      direction: (&point - &self.xyz).normalized(),
      origin: point
    }
  }
  fn bg_color_for_ray(&self, ray: &Ray) -> Color {
    match &self.background {
      Background::Sky => {
        let direction = ray.direction.normalized();
        let alignment = direction.dot(&self.orientation);
        let alignment = alignment.max(0.0).min(1.0);
        let r = 1.0 - 0.35294117647*alignment;
        let g = 1.0 - 0.20784313725*alignment;
        Color {
          r,
          g,
          b: 1.0
        }
      },
      Background::Black => Color::black()
    }
  }
  fn closest_object_index(&self, ray: &Ray, objects: &Vec<Box<Object>>) -> (i32, f32) {
    let mut sml:f32 = -1.0;
    let mut index:i32 = -1;
    for (i, item) in objects.iter().enumerate() {
      let t = item.ray_intersection_distance(ray);
      match t {
        Some(dist) => {
          if sml == -1.0 || dist < sml {
            sml = dist;
            index = i as i32;
          }
        },
        None => ()
      }
    }
    return (index, sml);
  }
  fn is_shadowed(&self, ray: &Ray, objects: &Vec<Box<Object>>, ignore_index: usize) -> bool {
    let mut sml = -1.0;
    for (i, item) in objects.iter().enumerate() {
      if i == ignore_index {
        continue;
      }
      if let Some(t) = item.ray_intersection_distance(ray) {
        if sml == -1.0 || t < sml {
          sml = t;
        }
      }
    }
    return sml > 0.0 && sml <= 1.0;
  }
  fn color_for_ray(&self, ray: &Ray, objects: &Vec<Box<Object>>, lights: &Vec<Light>, depth: u32, kr: f32) -> Color {
    let (closest_index, t) = self.closest_object_index(ray, objects);
    if closest_index == -1 {
      return self.bg_color_for_ray(ray);
    }
    let item = &objects[closest_index as usize];
    let material = item.material();
    let intersection = ray.point_at(t);
    let normal = item.normal_at_point(&intersection);
    let eye_direction = (&self.xyz - &intersection).normalized();

    let mut color = Color {r: 0.0, g: 0.0, b: 0.0};

    // refração
    let mut refrated = false;
    if material.kt > 0.0 {
      let rfactor = kr / material.kr;
      let c1 = normal.dot(&ray.direction.normalized());
      let tmp = 1.0 - (rfactor*rfactor)*(1.0 - c1*c1);
      if tmp > 0.0 {
        // refrata!
        let c2 = tmp.sqrt();
        let transmission = rfactor*&ray.direction + (rfactor*c1 - c2)*&normal;
        let new_ray = Ray {
          origin: &intersection + &transmission*0.001,
          direction: transmission
        };
        let new_kr = if kr != 1.0 { 1.0 } else { material.kr };
        refrated = true;
        color = self.color_for_ray(&new_ray, objects, lights, depth, new_kr);
      }
    }
    if refrated == false {
      color = &material.rgb * &self.ambient_light;
    }

    // iluminação
    for light in lights.iter() {
      let light_direction = (&light.xyz - &intersection).normalized();
      if self.is_shadowed(
        &Ray{origin: intersection.clone(), direction: light_direction.clone()},
        objects,
        closest_index as usize
      ) {
        continue;
      }
      let light_normal = normal.dot(&light_direction);
      if light_normal < 0.0 {
        continue; // está vindo por dentro do objeto
      }
      let difuse = material.kd * &light.difuse * light_normal;
      // especular
      let reflectance = 2.0 * light_normal * &normal - &light_direction;
      let specular = material.ks * &light.specular;
      let specular = specular * eye_direction.dot(&reflectance).powf(material.alpha);
      color = color + difuse + specular;
    }
    
    // reflexão
    if refrated == false && depth > 0 {
      let rd = (-&ray.direction).normalized();
      let reflectance = 2.0 * normal.dot(&rd) * &normal - rd;
      let reflection_ray = Ray {
        origin: intersection + &reflectance*0.001,
        direction: reflectance
      };
      let reflected = self.color_for_ray(&reflection_ray, objects, lights, depth-1, 1.0);
      color = color + reflected*material.ks;
    }
    return color.clip();
  }
  /* never used
  pub fn get_directions(&self) -> Vec<Vec<[u8; 3]>> {
    let bounds = self.image_plane_bounds();
    let w = self.width as usize;
    let h = self.height as usize;
    let mut colors = vec![vec![[0f32; 2]; h]; w];
    let mut ray:Ray;
    let mut x_bigger  = -1.0;
    let mut x_smaller =  1.0;
    let mut y_bigger  = -1.0;
    let mut y_smaller =  1.0;

    for x in 0..w {
      for y in 0..h {
        ray = self.ray_for_pixel(x as f32, y as f32, &bounds);
        let tmp_x = ray.direction.x;
        let tmp_y = ray.direction.y;
        if tmp_x > x_bigger {
          x_bigger = tmp_x;
        }
        if tmp_x < x_smaller {
          x_smaller = tmp_x;
        }
        if tmp_y > y_bigger {
          y_bigger = tmp_y;
        }
        if tmp_y < y_smaller {
          y_smaller = tmp_y;
        }
        colors[x][y][0] = tmp_x;
        colors[x][y][1] = tmp_y;
      }
    }

    let mut colors2 = vec![vec![[0u8; 3]; h]; w];
    for x in 0..w {
      for y in 0..h {
        let mut c = colors[x][y];
        c[0] -= x_smaller;
        c[0] /= x_bigger - x_smaller;
        c[1] -= y_smaller;
        c[1] /= y_bigger - y_smaller;
        
        colors2[x][y] = [(c[0]*255.0) as u8, (c[1]*255.0) as u8, 0u8];
      }
    }
    return colors2;
  }
  */
  pub fn take_picture(&self, objects: &Vec<Box<Object>>, lights: &Vec<Light>) -> Vec<Vec<[u8; 3]>> {
    let bounds = self.image_plane_bounds();
    let w = self.width as usize;
    let h = self.height as usize;
    let mut colors = vec![vec![[0u8; 3]; h]; w];
    let mut ray:Ray;
    for x in 0..w {
      for y in 0..h {
        ray = self.ray_for_pixel(x as f32, y as f32, &bounds);
        colors[x][y] = self.color_for_ray(&ray, objects, lights, 5, 1.0).as_array();
      }
    }

    return colors;
  }
  pub fn take_dof_picture(&self, objects: &Vec<Box<Object>>, lights: &Vec<Light>, r: f32, focal_length: f32, samples: u32) -> Vec<Vec<[u8; 3]>> {
    let bounds = self.image_plane_bounds();
    let w = self.width as usize;
    let h = self.height as usize;
    let mut colors = vec![vec![[0u8; 3]; h]; w];
    let mut ray:Ray;
    let mut rng = rand::thread_rng();
    let up = &self.orientation;
    let left = self.left_orientation();

    for x in 0..w {
      for y in 0..h {
        ray = self.ray_for_pixel(x as f32, y as f32, &bounds);
        let mut color = Color::black();
        let focal_point = ray.point_at(focal_length);
        for _ in 0..samples {
          let rx:f32 = rng.gen();
          let ry:f32 = rng.gen();
          let dx = (rx*2.0*r - r) * &left;
          let dy = (ry*2.0*r - r) * up;
          let new_origin = &ray.origin + (dx + dy);
          let new_direction = &focal_point - &new_origin;
          let new_ray = Ray {
            origin: new_origin,
            direction: new_direction.normalized()
          };
          color += self.color_for_ray(&new_ray, objects, lights, 5, 1.0);  
        }
        color = color/(samples as f32);
        colors[x][y] = color.as_array();
      }
    }
    return colors;
  }
  /* never used
  pub fn focal_length_for_point(&self, point: &Point) -> f32 {
    let center = &self.xyz + self.image_plane_distance*&self.target;
    let vector = point - center;
    let projection_size = vector.dot(&self.target);
    return projection_size;
  }
  */
}
