use bevy_ecs::prelude::*;
use crate::ecs::components::{Position, Velocity};

// 移动系统
pub fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.x;
        pos.y += vel.y;
        println!("Moved to: {}", *pos);
    }
}
