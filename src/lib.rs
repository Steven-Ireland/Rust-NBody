mod world;

use std::f64::consts::PI;

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_angles_on_axis() {
    let p1 = world::Point {x: 0.0, y: 0.0};
    
    // Edge cases
    let p2 = world::Point {x: 0.0, y: 1.0};
    assert_eq!(PI / 2.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: -1.0, y: 0.0};
    assert_eq!(PI, world::get_angle(p1, p2));

    let p2 = world::Point {x: 0.0, y: -1.0};
    assert_eq!(PI + PI / 2.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: 1.0, y: 0.0};
    assert_eq!(0.0, world::get_angle(p1, p2));
  }

  #[test]
  fn test_45_degree_angles() {
    let p1 = world::Point {x: 0.0, y: 0.0};
    
    let p2 = world::Point {x: 1.0, y: 1.0};
    assert_eq!(PI / 4.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: -1.0, y: 1.0};
    assert_eq!(PI - PI / 4.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: -1.0, y: -1.0};
    assert_eq!(PI + PI / 4.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: 1.0, y: -1.0};
    assert_eq!(2.0 * PI - PI / 4.0, world::get_angle(p1, p2));
  }

  #[test]
  fn test_offset_angles() {
    let p1 = world::Point {x: 2.0, y: 2.0};
    
    let p2 = world::Point {x: 3.0, y: 3.0};
    assert_eq!(PI / 4.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: 1.0, y: 3.0};
    assert_eq!(PI - PI / 4.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: 1.0, y: 1.0};
    assert_eq!(PI + PI / 4.0, world::get_angle(p1, p2));

    let p2 = world::Point {x: 3.0, y: 1.0};
    assert_eq!(2.0 * PI - PI / 4.0, world::get_angle(p1, p2));
  }

  #[test]
  fn test_world_update() {
    let body1 = world::Body {
      name: "body1",
      position: world::Point { x: 0.0, y: 0.0 },
      velocity: world::Vector { x: 0.0, y: 0.0 },
      mass: 1.0
    };

    let body2 = world::Body {
      name: "body2",
      position: world::Point { x: 100.0, y: 0.0 },
      velocity: world::Vector { x: 0.0, y: 0.0 },
      mass: 2.0
    };

    let mut bodies = vec![body1, body2];
    bodies = world::update_bodies(bodies);

    // body 1 should have twice the velocity of body 2 in the opposite direction
    assert_eq!(-2.0 * bodies[1].velocity.x, bodies[0].velocity.x);
  }
}