use bevy::prelude::*;
use rand::prelude::*;
use crate::components::*;
use super::GameAssets;

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    game_assets: Res<GameAssets>,
    difficulty: Res<Difficulty>,
) {
    spawn_timer.obstacle_timer.tick(time.delta());

    if spawn_timer.obstacle_timer.just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-300.0..300.0);
        
        let bear_chance = (difficulty.level as f32 * 0.1).min(0.6);
        let random = rng.gen_range(0.0..1.0);
        
        let (obstacle_type, sprite_index) = if random < bear_chance {
            (ObstacleType::BearMarket, 2)
        } else if random < bear_chance + 0.2 {
            (ObstacleType::Tax, 0)
        } else {
            (ObstacleType::TwitterTroll, 1)
        };

        let scale = match obstacle_type {
            ObstacleType::BearMarket => 0.15,
            ObstacleType::Tax => 0.19,
            ObstacleType::TwitterTroll => 0.19,
        };

        commands.spawn((
            SpriteBundle {
                texture: game_assets.obstacle_sprites[sprite_index].clone(),
                transform: Transform::from_xyz(x, 300.0, 0.0)
                    .with_scale(Vec3::splat(scale)),
                ..default()
            },
            Obstacle {
                damage: 1.0,
                obstacle_type,
                speed: difficulty.obstacle_speed,
            },
        ));
    }
}

pub fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Obstacle)>,
    time: Res<Time>,
) {
    for (entity, mut transform, obstacle) in query.iter_mut() {
        transform.translation.y -= obstacle.speed * time.delta_seconds();
        
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
} 