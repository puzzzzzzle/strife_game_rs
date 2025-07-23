use bevy_ecs::prelude::Resource;
use crossbeam::channel;
use crate::ecs::registries::registry::Registry;

// mirror/other -> core 发送修改请求, 在单线程上串行执行
pub trait UpdateEventTrait: Send + Sync {
    fn update(&self, registry: &mut Registry);
}
pub type UpdateEvent = Box<dyn UpdateEventTrait>;
pub type UpdateEventSender = channel::Sender<UpdateEvent>;
pub type UpdateEventReceiver = channel::Receiver<UpdateEvent>;

#[derive(Resource)]
pub struct EventSender(pub UpdateEventSender);

// core -> mirror 发送数据更新

// other -> mirror 发送查询请求, 在多线程上并行执行
pub trait QueryEventTrait: Send + Sync {
    fn query(&self, registry: & Registry);
}
pub type QueryEvent = Box<dyn QueryEventTrait>;
pub type QueryEventSender = channel::Sender<QueryEvent>;
pub type QueryEventReceiver = channel::Receiver<QueryEvent>;