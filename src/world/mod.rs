use std::f64::consts::PI;

const G: f64 = 1.0; // wrong, just hacking this in for now.

#[derive(PartialEq, Debug, Copy, Clone)]
struct Body {
  name: &'static str,
  x: f64,
  y: f64,
  mass: f64
}

struct Force {
  body: Body,
  x: f64,
  y: f64
}

pub struct BodyCollection {
  bodies: Vec<Body>
}

trait World {
    fn update(&self);
    fn add_body(&mut self, body: Body);
}

// Implements the newtonian equation F = G * (m1^2 + m2^2) / d^2
fn get_gravitational_force (body1: Body, body2: Body) -> f64 {
  let distance_squared = (body2.x - body1.x).powi(2) + (body2.y - body1.y).powi(2);
  let m1_squared = body1.mass.powi(2);
  let m2_squared = body2.mass.powi(2);

  return G * (m1_squared + m2_squared) / distance_squared;
}

fn get_force_angle(body1: Body, body2: Body) -> f64 {
  let distance_difference_x = body2.x - body1.x;
  let distance_difference_y = body2.y - body1.y;

  if distance_difference_x == 0.0 {
    // if body2 is above body1, return pi / 2. Otherwise return pi + pi/2
    if distance_difference_y >= 0.0 {
      return PI / 2.0;
    } else {
      return PI + PI / 2.0;
    }
  } else {
    if distance_difference_x >= 0.0 {
      return (distance_difference_y / distance_difference_x).atan();
    } else {
      return PI + (distance_difference_y / distance_difference_x).atan();
    }
  }
}

impl World for BodyCollection {
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