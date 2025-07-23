use bevy_ecs::component::Component;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use serde_json_display_derive::JsonDisplay;
#[derive(
    Component, Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, JsonDisplay,
)]
pub struct MonsterAIComponent {
    pub target: Vec2,
}