
use std::ops::Range;
// use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::ui::widgets::Window;
use macroquad::ui::{hash, widgets, root_ui};


#[derive(Debug, Clone)]
pub struct Parameters<T> {
  pub name: &'static str,
  pub value: T,
  pub bounds: Range<T>
}

pub struct ParameterWindow<T> {
  pub params: Vec<Parameters<T>>,
}

pub const SLIDER_HEIGHT: f32 = 22.0;

impl ParameterWindow<f32> {
  pub fn new(pos: Vec2, label: &'static str, params: &mut Vec<Parameters<f32>>) -> Self {
    widgets::Window::new(hash!(), pos, Vec2 { x: 400.0, y: SLIDER_HEIGHT * (params.len() as f32) })
      .label(label)
      .titlebar(true)
      .movable(true)
      .ui(&mut root_ui(), |ui| {
        for p in params.iter_mut() {
          let Range { start, end } = p.bounds;
          ui.slider(hash!(), &p.name, start..end, &mut p.value);
        }
      });

    Self { params: params.to_vec() }
  }
}