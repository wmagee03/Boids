use macroquad::prelude::*;

const BOID_HEIGHT: f32 = 10.0;
// const BOID_BASE: f32 = 6.0;

// const TOO_FAR: f32 = .0;
// const TOO_CLOSE: f32 = 30.0;

// const SEPARATION_VAR: f32 = 0.3;
// const ALIGNMENT_VAR: f32 = 0.2;
// const COHESION_VAR: f32 = 0.1;
const MAX_VELOCITY: f32 = 5.0;
const TURNING_BIAS: f32 = 1.5;
pub struct BoidParams {
  pub separation_bias: f32,
  pub alignment_bias: f32,
  pub cohesion_bias: f32,
  // pub turning_bias: f32,
  pub min_distance_tolerable: f32,
  pub max_distance_visible: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Boid {
  pub id: usize,
  pub position: Vec2,
  pub velocity: Vec2,
  // pub rotation: f32,
}


impl Boid {
  pub fn new(bounds1: Vec2, bounds2: Vec2, id: usize) -> Self {
    let position = Vec2::new(
      rand::gen_range::<f32>(bounds1.x, bounds2.x),
      rand::gen_range::<f32>(bounds1.y, bounds2.y),
    );
    let velocity = Vec2::new(
      rand::gen_range(-1.0, 1.0),
      rand::gen_range(-1.0, 1.0),
    ) * MAX_VELOCITY;

    // let rotation = velocity.to_angle();

    Self {
      id,
      position,
      velocity,
      // rotation,
    }
  }

  pub fn draw_self(&self) {
    let Vec2 { x, y } = self.position;
    draw_circle_lines(x, y , BOID_HEIGHT, 2.0, BLUE);
    // let vertex_top = Vec2::new(
    //   self.position.x + self.rotation.sin() * BOID_HEIGHT / 2.0,
    //   self.position.y - self.rotation.cos() * BOID_HEIGHT / 2.0,
    // );
    // let vertex_base_1 = Vec2::new(
    //   self.position.x - self.rotation.cos() * BOID_BASE / 2.0 - self.rotation.sin() * BOID_HEIGHT / 2.0,
    //   self.position.y - self.rotation.sin() * BOID_BASE / 2.0 + self.rotation.cos() * BOID_HEIGHT / 2.0,
    // );
    // let vertex_base_2 = Vec2::new(
    //   self.position.x + self.rotation.cos() * BOID_BASE / 2.0 - self.rotation.sin() * BOID_HEIGHT / 2.0,
    //   self.position.y + self.rotation.sin() * BOID_BASE / 2.0 + self.rotation.cos() * BOID_HEIGHT / 2.0,
    // );
    // draw_triangle_lines(vertex_top, vertex_base_1, vertex_base_2, 2.0, BLACK);
  }

  pub fn update(&mut self) {
    if self.velocity.length() > MAX_VELOCITY {
      self.velocity = self.velocity.normalize() * MAX_VELOCITY;
    }
    // self.rotation = self.velocity.to_angle();
    self.position += self.velocity;

    // Turn around naturally when approaching edge of screen
    if self.position.x > screen_width() { self.velocity.x -= TURNING_BIAS; }
    else if self.position.x < 0.0 { self.velocity.x += TURNING_BIAS; }

    if self.position.y > screen_height() { self.velocity.y -= TURNING_BIAS; }
    else if self.position.y < 0.0 { self.velocity.y += TURNING_BIAS; }

    // Make sure to wrap around to other side
    // if self.position.x > screen_width() { self.position.x = 0.0; }
    // else if self.position.x < 0.0 { self.position.x = screen_width(); }

    // if self.position.y > screen_height() { self.position.y = 0.0; }
    // else if self.position.y < 0.0 { self.position.y = screen_height(); }
  }

  pub fn apply_rules(&mut self, all_boids: &Vec::<Boid>, params: &BoidParams) {
    let mut separation_dv = Vec2::ZERO;
    let mut avg_velocity = Vec2::ZERO;
    let mut avg_position = Vec2::ZERO;

    let mut neighboring_boids = 0;

    for boid in all_boids.iter() {
      if self.id != boid.id {
        let class = self.classify_distance(
          boid,
          params.max_distance_visible,
          params.min_distance_tolerable
        );
        if class < 1 {
          avg_velocity += boid.velocity;
          avg_position += boid.position;
          if class == -1 {
            separation_dv += self.velocity - boid.velocity;
          }
          neighboring_boids += 1;
        }
      }
    }

    // let old_velocity = self.velocity.to_owned();
    let mut updated_alignment_velocity = Vec2::ZERO;
    let mut updated_cohesion_velocity = Vec2::ZERO;

    if neighboring_boids > 0 {
      avg_velocity /= neighboring_boids as f32;
      avg_position /= neighboring_boids as f32;

      updated_alignment_velocity = (avg_velocity - self.velocity) * params.alignment_bias;
      updated_cohesion_velocity = (avg_position - self.position) * params.cohesion_bias;
    }

    self.velocity += updated_alignment_velocity + updated_cohesion_velocity + (separation_dv * params.separation_bias);
    // self.rotation = Vec2::Y.angle_between(self.velocity);
  }

  pub fn classify_distance(&self, neighbor: &Boid, too_far: f32, too_close: f32) -> i8 {
    let distance = self.position.distance(neighbor.position);
    if distance > too_far { 1 }
    else if distance <= too_close { -1 }
    else { 0 }
  }

  // pub fn separation(&mut self, all_boids: &Vec::<Boid>) {
  //   let mut close_dv = Vec2::ZERO;

  //   for boid in all_boids.iter() {
  //     if self.id != boid.id {
  //       let class = self.classify_distance(boid);
  //       if class == -1 {
  //         close_dv += self.velocity - boid.velocity;
  //       }
  //     }
  //   }

  //   // self.rotation += self.velocity.angle_between(close_dv);
  //   self.velocity += close_dv * SEPARATION_VAR;
  // }

  // pub fn alignment(&mut self, all_boids: &Vec::<Boid>) {
  //   let mut avg_velocity = Vec2::ZERO;
  //   let mut neighboring_boids = 0;

  //   for boid in all_boids.iter() {
  //     if self.id != boid.id {
  //       let class = self.classify_distance(boid);
  //       if class < 1 {
  //         avg_velocity += boid.velocity;
  //         neighboring_boids += 1;
  //       }
  //     }
  //   }

  //   if neighboring_boids > 0 {
  //     avg_velocity /= neighboring_boids as f32;
  //     self.velocity += (avg_velocity - self.velocity) * ALIGNMENT_VAR;
  //     // self.rotation += self.velocity.angle_between(avg_velocity);
  //   }
  // }

  // pub fn cohesion(&mut self, all_boids: &Vec::<Boid>) {
  //   let mut avg_position = Vec2::ZERO;
  //   let mut neighboring_boids = 0;

  //   for boid in all_boids.iter() {
  //     if self.id != boid.id {
  //       let class = self.classify_distance(boid);
  //       if class < 1 {
  //         avg_position += boid.position;
  //         neighboring_boids += 1;
  //       }
  //     }
  //   }

  //   if neighboring_boids > 0 {
  //     avg_position /= neighboring_boids as f32;
  //     let updated_velocity = (avg_position - self.position) * COHESION_VAR;
  //     // self.rotation += self.velocity.angle_between(updated_velocity);
  //     self.velocity += updated_velocity;
  //   }
  // }
}