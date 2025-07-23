use bevy_ecs::prelude::Query;
use crate::GameConfig;
use crate::ecs::error::EcsError;
use crate::ecs::registries::registry::Registry;
use crate::ecs::registries::sync_data_structs::*;
use crossbeam::channel;

pub struct MirrorRegistry {
    pub registry: Registry,
    pub out_query_sender: QueryEventSender,
    out_query_receiver: QueryEventReceiver,
}
impl MirrorRegistry {
    // 创建
    pub fn new(
        config: GameConfig,
        core_event_sender: UpdateEventSender,
    ) -> std::result::Result<Self, EcsError> {
        let mut registry = Registry::new("mirror");
        registry
            .world
            .insert_resource(EventSender(core_event_sender.clone()));
        registry.initlize()?;

        let (out_query_sender, out_query_receiver) = channel::unbounded();

        Ok(Self {
            registry,
            out_query_sender,
            out_query_receiver,
        })
    }
    /**
     * 以有限的频率运行, CPU不够用时, 会降频
     */
    pub fn limit_update(&mut self) {
        // TODO 更新core数据到mirror

        // 并行执行只读的system
        self.registry
            .schedule_readonly
            .run(&mut self.registry.world);

        // 串行执行读写的system, 一般没有
        self.registry.schedule_rw.run(&mut self.registry.world);

        // 并行执行只读的查询请求
        // TODO 并行
        while let Ok(event) = self.out_query_receiver.try_recv()
        {
            event.query(&self.registry)
        }
    }
}
