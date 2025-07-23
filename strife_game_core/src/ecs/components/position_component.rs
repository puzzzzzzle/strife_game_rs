use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use serde_json_display_derive::JsonDisplay;

// 位置
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
pub struct Position(pub Vec2);
// TODO 添加网格追踪, 用于快速查找附近的单位, 可以考虑 bevy_spatial

#[test]
fn test_position() {
    let pos = Position(Vec2::new(1.0, 2.0));
    assert_eq!(pos.x, 1.0);
    println!("pos: {}", pos);
}
