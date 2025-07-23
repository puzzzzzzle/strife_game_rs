use crate::ecs::error::EcsError;
use crate::ecs::registries::registry::Registry;
use crate::ecs::systems::movement_system;
use crate::ecs::systems::monster_ai_system::{monster_ai_decide_system, monster_ai_execute_system};

pub trait OutUpdateEventTrait {
    fn update(&self, registry: &mut Registry);
}
pub struct CoreRegistry {
    pub registry: Registry,
    outgoing_events: Vec<Box<dyn OutUpdateEventTrait>>,
}

impl CoreRegistry {
    // 创建
    pub fn new() -> Result<CoreRegistry, EcsError> {
        let mut registry = Registry::new("core");
        // 移动
        registry.schedule_rw.add_systems(movement_system::movement_system);
        
        // 怪物AI
        // 并行决策
        registry.schedule_readonly.add_systems(monster_ai_decide_system);
        // 串行执行
        registry.schedule_rw.add_systems(monster_ai_execute_system);

        registry.initlize()?;
        Ok(CoreRegistry {
            registry,
            outgoing_events: Vec::new(),
        })
    }
    /**
     * 以固定频率运行的物理逻辑
     */
    pub fn fixed_update(&mut self) {
        // 串行执行外部传入的更新请求
        for event in self.outgoing_events.drain(..) {
            event.update(&mut self.registry);
        }

        // 并行执行只读的system
        self.registry
            .schedule_readonly
            .run(&mut self.registry.world);

        // 串行执行读写的system
        self.registry.schedule_rw.run(&mut self.registry.world);

        // 收集这一帧内发生的变化
    }
}
