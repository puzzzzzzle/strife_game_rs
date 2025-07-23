use crate::configures::GameConfig;
use crate::ecs::registries::core_registry::CoreRegistry;
use crate::ecs::registries::mirror_registry::mirror_registry;
use crate::ecs::error::EcsError;

pub struct Game {
    core_registry: CoreRegistry,
    // mirror_registry: Registry,
}
impl Game {
    pub fn new(config: GameConfig) -> Result<Self,EcsError> {
        let mut core_registry = CoreRegistry::new()?;
        core_registry.registry.world.insert_resource(config.clone());
        // let mut mirror_registry = mirror_registry::create_registry();
        // mirror_registry.world.insert_resource(config.clone());
        Ok(Self {
            core_registry,
            // mirror_registry,
        })
    }
}