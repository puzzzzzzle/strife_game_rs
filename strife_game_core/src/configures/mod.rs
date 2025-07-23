use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use serde_json_display_derive::JsonDisplay;

#[derive(Resource,Debug, Clone, Serialize, Deserialize, JsonDisplay)]
pub struct GameConfig {}
