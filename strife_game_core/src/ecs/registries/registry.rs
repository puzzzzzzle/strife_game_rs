use crate::ecs::error::EcsError;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ExecutorKind;

// world + schedule 基本等于entt的registry
pub struct Registry {
    pub world: World,
    pub schedule_readonly: Schedule,
    pub schedule_rw: Schedule,
    pub name: String,
}
impl Registry {
    pub fn new(name: &str) -> Self {
        let world = World::new();
        let mut schedule_readonly = Schedule::default();
        schedule_readonly.set_executor_kind(ExecutorKind::MultiThreaded);
        let mut schedule_rw = Schedule::default();
        schedule_rw.set_executor_kind(ExecutorKind::SingleThreaded);
        Registry {
            world,
            schedule_readonly,
            schedule_rw,
            name: name.into(),
        }
    }
    /**
     * 添加数新的system后, 最好重写init下
     */
    pub fn initlize(&mut self) -> Result<(), EcsError> {
        self.schedule_readonly
            .initialize(&mut self.world)
            .or(Err(EcsError::CreateRegistryFailed(
                "initialize schedule_readonly failed".into(),
            )))?;
        self.schedule_rw.initialize(&mut self.world).or(Err(
            EcsError::CreateRegistryFailed("initialize schedule_rw failed".into()),
        ))?;
        Ok(())
    }
}
