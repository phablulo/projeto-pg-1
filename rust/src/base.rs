use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign, Div, DivAssign, Neg};

#[derive(Debug)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32
}
#[derive(Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32
}
#[derive(Debug)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32
}
// métodos
impl Vector {
  pub fn norm(&self) -> f32 {
    self.dot(&self).sqrt()
  }
  pub fn normalized(&self) -> Vector {
    self / self.norm()
  }
  pub fn rotate_along(&self, axis: &Vector, angle: f32) -> Vector {
    let axis = axis.normalized();
    let axis = -axis * (angle / 2.0).sin();
    let a = (angle / 2.0).cos();
    let b = axis.x;
    let c = axis.y;
    let d = axis.z;
    let aa = a.powf(2.0);
    let bb = b.powf(2.0);
    let cc = c.powf(2.0);
    let dd = d.powf(2.0);

    let bc = b * c;
    let ad = a * d;
    let ac = a * c;
    let ab = a * b;
    let bd = b * d;
    let cd = c * d;

    let r0 = Vector {
      x: aa + bb - cc - dd,
      y: 2.0 * (bc + ad),
      z: 2.0 * (bd - ac)
    };
    let r1 = Vector {
      x: 2.0 * (bc - ad),
      y: aa + cc - bb - dd,
      z: 2.0 * (cd + ab)
    };
    let r2 = Vector {
      x: 2.0 * (bd + ac),
      y: 2.0 * (cd - ab),
      z: aa + dd - bb - cc
    };
    
    Vector {
      x: self.dot(&r0),
      y: self.dot(&r1),
      z: self.dot(&r2)
    }
  }
  fn sum(&self) -> f32 {
    self.x + self.y + self.z
  }
  pub fn dot(&self, other: &Vector) -> f32 {
    let m = self * other;
    m.sum()
  }
  pub fn dot_point(&self, point: &Point) -> f32 {
    self.x*point.x + self.y*point.y + self.z*point.z
  }
  pub fn clone(&self) -> Vector {
    Vector {
      x: self.x,
      y: self.y,
      z: self.z
    }
  }
  pub fn as_point(&self) -> Point {
    Point {
      x: self.x,
      y: self.y,
      z: self.z
    }
  }
  pub fn from(x: f32, y: f32, z:f32) -> Vector {
    Vector {x, y, z}
  }
}
impl Point {
  pub fn clone(&self) -> Point {
    Point {
      x: self.x,
      y: self.y,
      z: self.z
    }
  }
  pub fn as_vector(&self) -> Vector {
    Vector {
      x: self.x,
      y: self.y,
      z: self.z
    }
  }
  pub fn from(x: f32, y: f32, z:f32) -> Point {
    Point {x, y, z}
  }
}
impl Color {
  pub fn clip(&self) -> Color {
    Color {
      r: self.r.max(0.0).min(1.0),
      g: self.g.max(0.0).min(1.0),
      b: self.b.max(0.0).min(1.0)
    }
  }
  pub fn as_array(self) -> [u8; 3] {
    [(self.r*255.0) as u8, (self.g*255.0) as u8, (self.b*255.0) as u8]
  }
  pub fn from(r:f32, g: f32, b: f32) -> Color {
    Color {r, g, b}
  }
  pub fn black() -> Color {
    Color::from(0.0, 0.0, 0.0)
  }
}

// operadores unários
// -- vetores
impl Neg for Vector {
  type Output = Vector;
  fn neg(self) -> Vector {
    Vector {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
impl Neg for &Vector {
  type Output = Vector;
  fn neg(self) -> Vector {
    Vector {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
// -- pontos
impl Neg for Point {
  type Output = Point;
  fn neg(self) -> Point {
    Point {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
impl Neg for &Point {
  type Output = Point;
  fn neg(self) -> Point {
    Point {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}
// operações binárias
// -- vetores
impl Add<Vector> for Vector {
  type Output = Vector;
  fn add(self, other: Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<Vector> for &Vector {
  type Output = Vector;
  fn add(self, other: Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Vector> for Vector {
  type Output = Vector;
  fn add(self, other: &Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Vector> for &Vector {
  type Output = Vector;
  fn add(self, other: &Vector) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl AddAssign<Vector> for Vector {
  fn add_assign(&mut self, other: Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl AddAssign<&Vector> for Vector {
  fn add_assign(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl Sub<Vector> for Vector {
  type Output = Vector;
  fn sub(self, other: Vector) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<Vector> for &Vector {
  type Output = Vector;
  fn sub(self, other: Vector) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Vector> for Vector {
  type Output = Vector;
  fn sub(self, other: &Vector) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Vector> for &Vector {
  type Output = Vector;
  fn sub(self, other: &Vector) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl SubAssign<Vector> for Vector {
  fn sub_assign(&mut self, other: Vector) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}
impl SubAssign<&Vector> for Vector {
  fn sub_assign(&mut self, other: &Vector) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}
impl Mul<Vector> for Vector {
  type Output = Vector;
  fn mul(self, other: Vector) -> Vector {
    Vector {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}
impl Mul<Vector> for &Vector {
  type Output = Vector;
  fn mul(self, other: Vector) -> Vector {
    Vector {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}
impl Mul<&Vector> for Vector {
  type Output = Vector;
  fn mul(self, other: &Vector) -> Vector {
    Vector {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}
impl Mul<&Vector> for &Vector {
  type Output = Vector;
  fn mul(self, other: &Vector) -> Vector {
    Vector {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}
impl MulAssign<Vector> for Vector {
  fn mul_assign(&mut self, other: Vector) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
  }
}
impl MulAssign<&Vector> for Vector {
  fn mul_assign(&mut self, other: &Vector) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
  }
}
impl Div<Vector> for Vector {
  type Output = Vector;
  fn div(self, other: Vector) -> Vector {
    Vector {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}
impl Div<Vector> for &Vector {
  type Output = Vector;
  fn div(self, other: Vector) -> Vector {
    Vector {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}
impl Div<&Vector> for Vector {
  type Output = Vector;
  fn div(self, other: &Vector) -> Vector {
    Vector {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}
impl Div<&Vector> for &Vector {
  type Output = Vector;
  fn div(self, other: &Vector) -> Vector {
    Vector {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}
impl DivAssign<Vector> for Vector {
  fn div_assign(&mut self, other: Vector) {
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
  }
}
impl DivAssign<&Vector> for Vector {
  fn div_assign(&mut self, other: &Vector) {
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
  }
}
// -- pontos
impl Add<Point> for Point {
  type Output = Vector;
  fn add(self, other: Point) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<Point> for &Point {
  type Output = Vector;
  fn add(self, other: Point) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Point> for Point {
  type Output = Vector;
  fn add(self, other: &Point) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Point> for &Point {
  type Output = Vector;
  fn add(self, other: &Point) -> Vector {
    Vector {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Sub<Point> for Point {
  type Output = Vector;
  fn sub(self, other: Point) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<Point> for &Point {
  type Output = Vector;
  fn sub(self, other: Point) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Point> for Point {
  type Output = Vector;
  fn sub(self, other: &Point) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Point> for &Point {
  type Output = Vector;
  fn sub(self, other: &Point) -> Vector {
    Vector {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
// -- mistura
impl Add<Point> for Vector {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<Point> for &Vector {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Point> for Vector {
  type Output = Point;
  fn add(self, other: &Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Point> for &Vector {
  type Output = Point;
  fn add(self, other: &Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl AddAssign<Point> for Vector {
  fn add_assign(&mut self, other: Point) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl AddAssign<&Point> for Vector {
  fn add_assign(&mut self, other: &Point) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl Sub<Point> for Vector {
  type Output = Point;
  fn sub(self, other: Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<Point> for &Vector {
  type Output = Point;
  fn sub(self, other: Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Point> for Vector {
  type Output = Point;
  fn sub(self, other: &Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Point> for &Vector {
  type Output = Point;
  fn sub(self, other: &Point) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl SubAssign<Point> for Vector {
  fn sub_assign(&mut self, other: Point) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}
impl SubAssign<&Point> for Vector {
  fn sub_assign(&mut self, other: &Point) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}
impl Add<Vector> for Point {
  type Output = Point;
  fn add(self, other: Vector) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<Vector> for &Point {
  type Output = Point;
  fn add(self, other: Vector) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Vector> for Point {
  type Output = Point;
  fn add(self, other: &Vector) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl Add<&Vector> for &Point {
  type Output = Point;
  fn add(self, other: &Vector) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}
impl AddAssign<Vector> for Point {
  fn add_assign(&mut self, other: Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl AddAssign<&Vector> for Point {
  fn add_assign(&mut self, other: &Vector) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}
impl Sub<Vector> for Point {
  type Output = Point;
  fn sub(self, other: Vector) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<Vector> for &Point {
  type Output = Point;
  fn sub(self, other: Vector) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Vector> for Point {
  type Output = Point;
  fn sub(self, other: &Vector) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl Sub<&Vector> for &Point {
  type Output = Point;
  fn sub(self, other: &Vector) -> Point {
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}
impl SubAssign<Vector> for Point {
  fn sub_assign(&mut self, other: Vector) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}
impl SubAssign<&Vector> for Point {
  fn sub_assign(&mut self, other: &Vector) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}
// -- escalar
impl Mul<f32> for Vector {
  type Output = Vector;
  fn mul(self, other: f32) -> Vector {
    Vector {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other
    }
  }
}
impl Mul<f32> for &Vector {
  type Output = Vector;
  fn mul(self, other: f32) -> Vector {
    Vector {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other
    }
  }
}
impl MulAssign<f32> for Vector {
  fn mul_assign(&mut self, other: f32) {
    self.x *= other;
    self.y *= other;
    self.z *= other;
  }
}
impl Div<f32> for Vector {
  type Output = Vector;
  fn div(self, other: f32) -> Vector {
    Vector {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other
    }
  }
}
impl Div<f32> for &Vector {
  type Output = Vector;
  fn div(self, other: f32) -> Vector {
    Vector {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other
    }
  }
}
impl DivAssign<f32> for Vector {
  fn div_assign(&mut self, other: f32) {
    self.x /= other;
    self.y /= other;
    self.z /= other;
  }
}
impl Mul<Vector> for f32 {
  type Output = Vector;
  fn mul(self, other: Vector) -> Vector {
    Vector {
      x: other.x * self,
      y: other.y * self,
      z: other.z * self
    }
  }
}
impl Mul<&Vector> for f32 {
  type Output = Vector;
  fn mul(self, other: &Vector) -> Vector {
    Vector {
      x: other.x * self,
      y: other.y * self,
      z: other.z * self
    }
  }
}
impl Div<Vector> for f32 {
  type Output = Vector;
  fn div(self, other: Vector) -> Vector {
    Vector {
      x: other.x / self,
      y: other.y / self,
      z: other.z / self
    }
  }
}
impl Div<&Vector> for f32 {
  type Output = Vector;
  fn div(self, other: &Vector) -> Vector {
    Vector {
      x: other.x / self,
      y: other.y / self,
      z: other.z / self
    }
  }
}
impl Mul<f32> for Point {
  type Output = Point;
  fn mul(self, other: f32) -> Point {
    Point {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other
    }
  }
}
impl Mul<f32> for &Point {
  type Output = Point;
  fn mul(self, other: f32) -> Point {
    Point {
      x: self.x * other,
      y: self.y * other,
      z: self.z * other
    }
  }
}
impl MulAssign<f32> for Point {
  fn mul_assign(&mut self, other: f32) {
    self.x *= other;
    self.y *= other;
    self.z *= other;
  }
}
impl Div<f32> for Point {
  type Output = Point;
  fn div(self, other: f32) -> Point {
    Point {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other
    }
  }
}
impl Div<f32> for &Point {
  type Output = Point;
  fn div(self, other: f32) -> Point {
    Point {
      x: self.x / other,
      y: self.y / other,
      z: self.z / other
    }
  }
}
impl DivAssign<f32> for Point {
  fn div_assign(&mut self, other: f32) {
    self.x /= other;
    self.y /= other;
    self.z /= other;
  }
}
impl Mul<Point> for f32 {
  type Output = Point;
  fn mul(self, other: Point) -> Point {
    Point {
      x: other.x * self,
      y: other.y * self,
      z: other.z * self
    }
  }
}
impl Mul<&Point> for f32 {
  type Output = Point;
  fn mul(self, other: &Point) -> Point {
    Point {
      x: other.x * self,
      y: other.y * self,
      z: other.z * self
    }
  }
}
impl Div<Point> for f32 {
  type Output = Point;
  fn div(self, other: Point) -> Point {
    Point {
      x: other.x / self,
      y: other.y / self,
      z: other.z / self
    }
  }
}
impl Div<&Point> for f32 {
  type Output = Point;
  fn div(self, other: &Point) -> Point {
    Point {
      x: other.x / self,
      y: other.y / self,
      z: other.z / self
    }
  }
}
// -- cores
impl Add<Color> for Color {
  type Output = Color;
  fn add(self, other: Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b
    }
  }
}
impl Add<Color> for &Color {
  type Output = Color;
  fn add(self, other: Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b
    }
  }
}
impl Add<&Color> for Color {
  type Output = Color;
  fn add(self, other: &Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b
    }
  }
}
impl Add<&Color> for &Color {
  type Output = Color;
  fn add(self, other: &Color) -> Color {
    Color {
      r: self.r + other.r,
      g: self.g + other.g,
      b: self.b + other.b
    }
  }
}
impl AddAssign<Color> for Color {
  fn add_assign(&mut self, other: Color) {
    self.r += other.r;
    self.g += other.g;
    self.b += other.b;
  }
}
impl AddAssign<&Color> for Color {
  fn add_assign(&mut self, other: &Color) {
    self.r += other.r;
    self.g += other.g;
    self.b += other.b;
  }
}
impl Sub<Color> for Color {
  type Output = Color;
  fn sub(self, other: Color) -> Color {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b
    }
  }
}
impl Sub<Color> for &Color {
  type Output = Color;
  fn sub(self, other: Color) -> Color {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b
    }
  }
}
impl Sub<&Color> for Color {
  type Output = Color;
  fn sub(self, other: &Color) -> Color {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b
    }
  }
}
impl Sub<&Color> for &Color {
  type Output = Color;
  fn sub(self, other: &Color) -> Color {
    Color {
      r: self.r - other.r,
      g: self.g - other.g,
      b: self.b - other.b
    }
  }
}
impl SubAssign<Color> for Color {
  fn sub_assign(&mut self, other: Color) {
    self.r -= other.r;
    self.g -= other.g;
    self.b -= other.b;
  }
}
impl SubAssign<&Color> for Color {
  fn sub_assign(&mut self, other: &Color) {
    self.r -= other.r;
    self.g -= other.g;
    self.b -= other.b;
  }
}

impl Mul<f32> for Color {
  type Output = Color;
  fn mul(self, other: f32) -> Color {
    Color {
      r: self.r * other,
      g: self.g * other,
      b: self.b * other
    }
  }
}
impl Mul<f32> for &Color {
  type Output = Color;
  fn mul(self, other: f32) -> Color {
    Color {
      r: self.r * other,
      g: self.g * other,
      b: self.b * other
    }
  }
}
impl MulAssign<f32> for Color {
  fn mul_assign(&mut self, other: f32) {
    self.r *= other;
    self.g *= other;
    self.b *= other;
  }
}
impl Div<f32> for Color {
  type Output = Color;
  fn div(self, other: f32) -> Color {
    Color {
      r: self.r / other,
      g: self.g / other,
      b: self.b / other
    }
  }
}
impl Div<f32> for &Color {
  type Output = Color;
  fn div(self, other: f32) -> Color {
    Color {
      r: self.r / other,
      g: self.g / other,
      b: self.b / other
    }
  }
}
impl DivAssign<f32> for Color {
  fn div_assign(&mut self, other: f32) {
    self.r /= other;
    self.g /= other;
    self.b /= other;
  }
}
impl Mul<Color> for f32 {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    Color {
      r: other.r * self,
      g: other.g * self,
      b: other.b * self
    }
  }
}
impl Mul<&Color> for f32 {
  type Output = Color;
  fn mul(self, other: &Color) -> Color {
    Color {
      r: other.r * self,
      g: other.g * self,
      b: other.b * self
    }
  }
}
impl Div<Color> for f32 {
  type Output = Color;
  fn div(self, other: Color) -> Color {
    Color {
      r: other.r / self,
      g: other.g / self,
      b: other.b / self
    }
  }
}
impl Div<&Color> for f32 {
  type Output = Color;
  fn div(self, other: &Color) -> Color {
    Color {
      r: other.r / self,
      g: other.g / self,
      b: other.b / self
    }
  }
}
impl Mul<Color> for Color {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b
    }
  }
}
impl Mul<Color> for &Color {
  type Output = Color;
  fn mul(self, other: Color) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b
    }
  }
}
impl Mul<&Color> for Color {
  type Output = Color;
  fn mul(self, other: &Color) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b
    }
  }
}
impl Mul<&Color> for &Color {
  type Output = Color;
  fn mul(self, other: &Color) -> Color {
    Color {
      r: self.r * other.r,
      g: self.g * other.g,
      b: self.b * other.b
    }
  }
}
impl MulAssign<Color> for Color {
  fn mul_assign(&mut self, other: Color) {
    self.r *= other.r;
    self.g *= other.g;
    self.b *= other.b;
  }
}
impl MulAssign<&Color> for Color {
  fn mul_assign(&mut self, other: &Color) {
    self.r *= other.r;
    self.g *= other.g;
    self.b *= other.b;
  }
}
impl Div<Color> for Color {
  type Output = Color;
  fn div(self, other: Color) -> Color {
    Color {
      r: self.r / other.r,
      g: self.g / other.g,
      b: self.b / other.b
    }
  }
}
impl Div<Color> for &Color {
  type Output = Color;
  fn div(self, other: Color) -> Color {
    Color {
      r: self.r / other.r,
      g: self.g / other.g,
      b: self.b / other.b
    }
  }
}
impl Div<&Color> for Color {
  type Output = Color;
  fn div(self, other: &Color) -> Color {
    Color {
      r: self.r / other.r,
      g: self.g / other.g,
      b: self.b / other.b
    }
  }
}
impl Div<&Color> for &Color {
  type Output = Color;
  fn div(self, other: &Color) -> Color {
    Color {
      r: self.r / other.r,
      g: self.g / other.g,
      b: self.b / other.b
    }
  }
}
impl DivAssign<Color> for Color {
  fn div_assign(&mut self, other: Color) {
    self.r /= other.r;
    self.g /= other.g;
    self.b /= other.b;
  }
}
impl DivAssign<&Color> for Color {
  fn div_assign(&mut self, other: &Color) {
    self.r /= other.r;
    self.g /= other.g;
    self.b /= other.b;
  }
}

/*
 Script que fiz pra me ajudar a criar esse arquivo:

var tr = string => string.replace(/([a-z])([A-Z])/g, '$1_$2').toLowerCase()
var cap = x => x[0].toUpperCase() + x.slice(1)
function generatePartial(left, right, result, ops, gen, isPrim2, isPrim1) {
  const g = (op) => {
    signal = op[1]
    let si
    op = op[0]
    const ig = tr(op) !== op
    let s = ''
    for (const x of ['','&']) {
      if (isPrim1 && x === '&') continue;
      for (const y of ['','&']) {
        if (y === '&' && (ig || isPrim2)) continue;
        if (ig) {
          si = signal+'='
          re = ''
        }
        else {
          si = signal
          re = `-> ${result} `
        }
        s+= `impl ${cap(op)}<${x}${right}> for ${y}${left} {\n`
        if (!ig) s+= `  type Output = ${result};\n`
        s+= `  fn ${tr(op)}(${ig?'&mut ':''}self, other: ${x}${right}) ${re}{\n`
        if (ig) {
          s+= `    self.x ${si} ${gen('x')};\n`
          s+= `    self.y ${si} ${gen('y')};\n`
          s+= `    self.z ${si} ${gen('z')};\n`
        }
        else {
          s+= `    ${result} {\n`
          s+= `      x: self.x ${si} ${gen('x')},\n`
          s+= `      y: self.y ${si} ${gen('y')},\n`
          s+= `      z: self.z ${si} ${gen('z')}\n`
          s+= `    }\n`
        }
        s+= `  }\n`
        s+= `}\n`
      }
    }
    return s
  }
  const otherOps = []
  for (const op of ops) {
    otherOps.push(op)
    otherOps.push([op[0]+'Assign', op[1]]) 
  }
  return otherOps.map(g).join('')
}
function generate(left, right, result, ops, isPrim1, isPrim2, gen=i=>`other.${i}`) {
  const p1 = generatePartial(left, right, result, ops, gen, isPrim1, isPrim2)
  const p2 = left === right ? '' : generatePartial(right, left, result, ops, gen, isPrim2, isPrim1)
  
  return p1 + p2
}
generate('Vector','Vector', 'Vector', [['add','+'], ['sub','-']])
generate('Vector','Vector', 'Vector', [['mul','*'], ['div','/']])

*/