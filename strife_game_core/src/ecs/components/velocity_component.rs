use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json_display_derive::JsonDisplay;

// 速度
#[derive(
    Component, Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, JsonDisplay,
)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
