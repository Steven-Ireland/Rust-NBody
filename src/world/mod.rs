use std::f64::consts::PI;

const G: f64 = 1.0; // wrong, just hacking this in for now.

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64
}

type Vector = Point;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Body {
  pub name: &'static str,
  pub position: Point,
  pub mass: f64
}

struct Force {
  body: Body,
  vector: Vector
}

pub struct BodyCollection {
  pub bodies: Vec<Body>
}

trait WorldSimulation {
  fn update(&self);
  fn add_body(&mut self, body: Body);
}

// Implements the newtonian equation F = G * (m1^2 + m2^2) / d^2
pub fn get_gravitational_force (body1: Body, body2: Body) -> f64 {
  let distance_squared = (body2.position.x - body1.position.x).powi(2) + (body2.position.y - body1.position.y).powi(2);
  let m1_squared = body1.mass.powi(2);
  let m2_squared = body2.mass.powi(2);

  return G * (m1_squared + m2_squared) / distance_squared;
}

pub fn get_force_angle (point1: Point, point2: Point) -> f64 {
  let distance_difference_x = point2.x - point1.x;
  let distance_difference_y = point2.y - point1.y;

  if (distance_difference_x == 0.0) {
    // if body2 is above body1, return pi / 2. Otherwise return pi + pi/2
    if (distance_difference_y >= 0.0) {
      return PI / 2.0;
    } else {
      return PI + PI / 2.0;
    }
  } else {
    if (distance_difference_x >= 0.0) {
      if distance_difference_y < 0.0 {
        return 2.0 * PI + (distance_difference_y / distance_difference_x).atan();
      } else {
        return (distance_difference_y / distance_difference_x).atan();
      }
    } else {
      return PI + (distance_difference_y / distance_difference_x).atan();
    }
  }
}

impl WorldSimulation for BodyCollection {
  fn update(&self) {
    let forces : Vec<Force> = Vec::new();
    for body1 in &self.bodies {
      // For each body, calculate forces exterted on it by every other body
      for body2 in &self.bodies {
        // ignore itself
        if body1 == body2 {
          continue; 
        }

        let force_magnitude = get_gravitational_force(*body1, *body2);
      }
    }
  }

  fn add_body(&mut self, body: Body) {
    self.bodies.push(body);
  }

}