use std::f64::consts::PI;

const G: f64 = 1.0; // wrong, just hacking this in for now.

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64
}
pub type Vector = Point;

pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
pub const RESTING: Vector = Vector { x: 0.0, y: 0.0 };

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Body {
  pub name: &'static str,
  pub position: Point,
  pub velocity: Vector,
  pub mass: f64
}

struct Force<'a> {
  body: &'a Body,
  vector: Vector
}

// Implements the newtonian equation F = G * (m1^2 + m2^2) / d^2
pub fn get_gravitational_force (body1: Body, body2: Body) -> f64 {
  let distance_squared = (body2.position.x - body1.position.x).powi(2) + (body2.position.y - body1.position.y).powi(2);
  let m1_squared = body1.mass.powi(2);
  let m2_squared = body2.mass.powi(2);

  return G * (m1_squared + m2_squared) / distance_squared;
}

pub fn get_angle (point1: Point, point2: Point) -> f64 {
  let distance_difference_x = point2.x - point1.x;
  let distance_difference_y = point2.y - point1.y;

  if distance_difference_x == 0.0 {
    // if body2 is above body1, return pi / 2. Otherwise return pi + pi/2
    if distance_difference_y >= 0.0 {
      return PI / 2.0;
    } else {
      return PI + PI / 2.0;
    }
  } else {
    if distance_difference_x >= 0.0 {
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

pub fn update_bodies(bodies: Vec<Body>, time_step: f64) -> Vec<Body> {
  let mut forces : Vec<Force> = Vec::new();
  // For each body, calculate forces exterted on it by each other body
  for body1 in bodies.iter() {
    let mut sum_forces = Force { body: body1, vector: Vector {x: 0.0, y: 0.0}};
    for body2 in bodies.iter() {
      // ignore itself
      if body1 == body2 {
        continue; 
      }

      let force_magnitude: f64 = get_gravitational_force(*body1, *body2);
      let force_angle: f64 = get_angle((*body1).position,(*body2).position);

      let force_vector = Vector {
        x: force_magnitude * force_angle.cos(), 
        y: force_magnitude * force_angle.sin()
      };

      sum_forces = Force {
        vector: Vector {
          x: sum_forces.vector.x + force_vector.x, 
          y: sum_forces.vector.y + force_vector.y
        },
        ..sum_forces
      };
    }

    forces.push(sum_forces);
  }

  return forces.iter().map(|force| Body { 
    position: Point {
      x: (*force.body).position.x + (*force.body).velocity.x * time_step,
      y: (*force.body).position.y + (*force.body).velocity.y * time_step
    }, 
    velocity: Vector {
      x: (*force.body).velocity.x + force.vector.x * time_step / (*force.body).mass,
      y: (*force.body).velocity.y + force.vector.y * time_step / (*force.body).mass
    },
    ..(*force.body)
  }).collect();

}