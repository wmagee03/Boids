mod entities;
mod windows;

use entities::{Boid, BoidParams};
use windows::{ParameterWindow, Parameters};
use macroquad::prelude::*;
use macroquad::ui::{hash, widgets, root_ui};

#[macroquad::main("Boids")]
async fn main() {
  let window_size = Vec2::new(screen_width(), screen_height());
  clear_background(WHITE);

  let mut separation_bias: f32 = 0.0;
  let mut alignment_bias: f32 = 0.0;
  let mut cohesion_bias: f32 = 0.0;
  let mut turning_bias: f32 = 0.5;
  let mut min_distance_tolerable: f32 = 30.0;
  let mut max_distance_visible: f32 = 60.0;

  let mut param_vec: Vec<Parameters<f32>> = Vec::from([
    Parameters::<f32> { name: "Separation Bias", value: 0.0, bounds: 0.0..1.0 },
    Parameters::<f32> { name: "Alignment Bias", value: 0.0, bounds: 0.0..1.0 },
    Parameters::<f32> { name: "Cohesion Bias", value: 0.0, bounds: 0.0..1.0 },
    Parameters::<f32> { name: "Turning Bias", value: 0.1, bounds: 0.1..1.0 },
    Parameters::<f32> { name: "Min Distance", value: 30.0, bounds: 0.0..100.0 },
    Parameters::<f32> { name: "Max Distance", value: 60.0, bounds: 0.0..100.0 },
  ]);


  let mut all_boids: Vec<Boid> = Vec::new();
  for idx in 0..100 {
    let boid = Boid::new(Vec2::ZERO, window_size, idx);
    boid.draw_self();
    all_boids.push(boid);
  }

  loop {
    let mut all_boids_copy = all_boids.to_owned();
    clear_background(WHITE);

    widgets::Window::new(hash!(), Vec2::new(100.0, 100.0), Vec2::new(400.0, 132.0))
      .label("Boid Params")
      .titlebar(true)
      .movable(true)
      .ui(&mut root_ui(), |ui| {
        ui.slider(hash!(), "Separation Bias", 0.0..1.0, &mut separation_bias);
        ui.slider(hash!(), "Alignment Bias", 0.0..1.0, &mut alignment_bias);
        ui.slider(hash!(), "Cohesion Bias", 0.0..1.0, &mut cohesion_bias);
        // ui.slider(hash!(), "Turning Bias", 0.1..1.0, &mut turning_bias);
        ui.slider(hash!(), "Min Distance", 0.0..100.0, &mut min_distance_tolerable);
        ui.slider(hash!(), "Max Distance", 0.0..100.0, &mut max_distance_visible);
      });

    let params = BoidParams {
      separation_bias,
      alignment_bias,
      cohesion_bias,
      // turning_bias,
      min_distance_tolerable,
      max_distance_visible,
    };
    for boid in all_boids.iter_mut() {
      boid.draw_self();
      boid.apply_rules(&all_boids_copy, &params);
      boid.update();

      all_boids_copy[boid.id] = boid.to_owned();
    }
    next_frame().await
  }
}
