use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use serde_json_display_derive::JsonDisplay;

// 速度
#[derive(
    Component,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonDisplay,
    Deref,
    DerefMut,
    From,
    Into,
)]
pub struct Velocity (pub Vec2);
