use crate::ecs::components::{MonsterAIComponent, Position, Velocity};
use bevy_ecs::prelude::*;

#[derive(Event, Debug)]
pub struct MonsterAIDecision {
    entity: Entity,
    decide: MonsterAIComponent,
}

// AI决策, 后续开销会很大, 并行运行
pub fn monster_ai_decide_system(
    query: Query<(Entity, &MonsterAIComponent, &Position, &Velocity)>,
    mut writer: EventWriter<MonsterAIDecision>,
) {
    for (entity,&ai, &pos, &vel) in query {
        // TODO 行为树
        let decide= MonsterAIComponent{
            target: pos.into(),
        };
        writer.write(MonsterAIDecision {
            entity,
            decide
        });
    }
}

// AI执行, 串行写阶段
pub fn monster_ai_execute_system(
    mut query: Query<(&mut Position, &mut Velocity)>,
    mut reader: EventReader<MonsterAIDecision>,
) {
    for decision in reader.read() {
        if let Ok((mut pos, mut vel)) = query.get_mut(decision.entity) {
            // 修改朝向
            // 根据当前位置 和 目标位置 计算方向, 并修改速度
            let direction = (decision.decide.target - pos.0).normalize();
            let speed = 3.0;
            vel.0 = direction * speed;
        }
    }
}
