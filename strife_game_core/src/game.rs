use crate::configures::GameConfig;
use crate::ecs::error::EcsError;
use crate::ecs::registries::core_registry::CoreRegistry;
use crate::ecs::registries::mirror_registry::MirrorRegistry;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct Game {
    core_registry: Arc<Mutex<CoreRegistry>>,
    mirror_registry: Arc<Mutex<MirrorRegistry>>,
    core_thread: Option<thread::JoinHandle<()>>,
    mirror_thread: Option<thread::JoinHandle<()>>,
    should_stop: Arc<AtomicBool>,

}
impl Game {
    pub fn new(config: GameConfig) -> Result<Self, EcsError> {
        let mut core_registry = CoreRegistry::new(config.clone())?;
        let mut mirror_registry =
            MirrorRegistry::new(config.clone(), core_registry.out_update_sender.clone())?;
        Ok(Self {
            core_registry: Arc::new(Mutex::new(core_registry)),
            mirror_registry: Arc::new(Mutex::new(mirror_registry)),
            core_thread: None,
            mirror_thread: None,
            should_stop: Arc::new(AtomicBool::new(false)),
        })
    }
    pub fn run(&mut self) {
        // 启动 core_registry 线程
        let core_registry = self.core_registry.clone();
        let should_stop = self.should_stop.clone();
        self.core_thread = Some(thread::spawn(move || {
            let mut core = core_registry.lock().unwrap();
            while !should_stop.load(Ordering::Relaxed) {

            }
        }));

        // 启动 mirror_registry 线程
        let mirror_registry = self.mirror_registry.clone();
        let should_stop = self.should_stop.clone();
        self.mirror_thread = Some(thread::spawn(move || {
            let mut mirror = mirror_registry.lock().unwrap();
            while !should_stop.load(Ordering::Relaxed) {

            }
        }));
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.core_thread.take() {
            handle.join().unwrap();
        }
        if let Some(handle) = self.mirror_thread.take() {
            handle.join().unwrap();
        }
    }
}
