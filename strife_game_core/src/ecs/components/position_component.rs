use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json_display_derive::JsonDisplay;
// 位置
#[derive(
    Component, Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, JsonDisplay,
)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
