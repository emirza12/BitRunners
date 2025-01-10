use bevy::prelude::*;
use rand::prelude::*;
use crate::components::*;
use super::GameAssets;

// Spawns obstacles in the game based on difficulty
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
        let x = rng.gen_range(-350.0..350.0); // Random position for the obstacle

        let bear_chance = (difficulty.level as f32 * 0.7).min(0.6);
        let random = rng.gen_range(0.0..1.0);

        // Determine the obstacle type and select the corresponding sprite
        let (obstacle_type, sprite_index) = if random < bear_chance {
            (ObstacleType::BearMarket, 2)
        } else if random < bear_chance + 0.2 {
            (ObstacleType::Tax, 0)
        } else {
            (ObstacleType::TwitterTroll, 1)
        };

        // Set obstacle scale based on its type
        let scale = match obstacle_type {
            ObstacleType::BearMarket => 0.15,
            ObstacleType::Tax => 0.19,
            ObstacleType::TwitterTroll => 0.19,
        };

        // Spawn the obstacle entity with the chosen properties
        commands.spawn((
            SpriteBundle {
                texture: game_assets.obstacle_sprites[sprite_index].clone(),
                transform: Transform::from_xyz(x, 300.0, 0.0).with_scale(Vec3::splat(scale)),
                ..default()
            },
            Obstacle {
                damage: 1.0, // Set obstacle damage
                obstacle_type, // Set the type of obstacle (BearMarket, Tax, etc.)
                speed: difficulty.obstacle_speed, // Speed influenced by difficulty
            },
        ));
    }
}

// Moves obstacles downward and despawns them when they go off-screen
pub fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Obstacle)>,
    time: Res<Time>,
) {
    for (entity, mut transform, obstacle) in query.iter_mut() {
        transform.translation.y -= obstacle.speed * time.delta_seconds(); // Move obstacle downwards
        
        // If the obstacle goes off-screen (below y = -300), despawn it
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}
