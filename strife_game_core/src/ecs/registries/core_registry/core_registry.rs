use crate::GameConfig;
use crate::ecs::error::EcsError;
use crate::ecs::registries::registry::Registry;
use crate::ecs::systems::monster_ai_system::{monster_ai_decide_system, monster_ai_execute_system};
use crate::ecs::systems::movement_system;
use crate::ecs::registries::sync_data_structs::*;
use crossbeam::channel;

pub struct CoreRegistry {
    pub registry: Registry,
    pub out_update_sender: UpdateEventSender,
    out_update_receiver: UpdateEventReceiver,
}

impl CoreRegistry {
    // 创建
    pub fn new(config: GameConfig) -> Result<CoreRegistry, EcsError> {
        let mut registry = Registry::new("core");
        registry.world.insert_resource(config.clone());

        // 移动
        registry
            .schedule_rw
            .add_systems(movement_system::movement_system);

        // 怪物AI
        // 并行决策
        registry
            .schedule_readonly
            .add_systems(monster_ai_decide_system);
        // 串行执行
        registry.schedule_rw.add_systems(monster_ai_execute_system);

        registry.initlize()?;

        let (out_update_sender, out_update_receiver) = channel::unbounded();
        Ok(CoreRegistry {
            registry,
            out_update_sender,
            out_update_receiver,
        })
    }
    /**
     * 需要以固定频率调用来运行的物理逻辑
     */
    pub fn fixed_update(&mut self) {
        // 串行执行外部传入的更新请求
        while let Ok(event) = self.out_update_receiver.try_recv() {
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
