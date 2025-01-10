use bevy::prelude::*;
use rand::prelude::*;
use crate::components::*;
use super::GameAssets;

// Spawns bitcoins in the game based on difficulty
pub fn spawn_bitcoins(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    game_assets: Res<GameAssets>,
    difficulty: Res<Difficulty>,
) {
    spawn_timer.bitcoin_timer.tick(time.delta());

    if spawn_timer.bitcoin_timer.just_finished() {
        let mut rng = rand::thread_rng();
        
        // Adjust spawn probability based on difficulty
        let spawn_chance = 1.0 - (difficulty.level as f32 * 0.05).min(0.6);
        
        if rng.gen_range(0.0..1.0) < spawn_chance {
            let x = rng.gen_range(-350.0..350.0);
            
            // Spawn a bitcoin with specific properties
            commands.spawn((
                SpriteBundle {
                    texture: game_assets.bitcoin_sprite.clone(),
                    transform: Transform::from_xyz(x, 300.0, 0.0)
                        .with_scale(Vec3::splat(0.03)),
                    ..default()
                },
                Bitcoin { 
                    value: 1,
                    speed: 150.0 * (1.0 + (difficulty.level as f32 * 0.1)),
                },
            ));
        }
    }
}

// Handles bitcoin collection by the player
pub fn collect_bitcoins(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    bitcoin_query: Query<(Entity, &Transform, &Bitcoin)>,
) {
    let (mut player, player_transform) = player_query.single_mut();

    // Check if player is close enough to collect bitcoins
    for (bitcoin_entity, bitcoin_transform, bitcoin) in bitcoin_query.iter() {
        let distance = player_transform.translation.distance(bitcoin_transform.translation);
        
        if distance < 40.0 {
            player.score += bitcoin.value;
            commands.entity(bitcoin_entity).despawn(); // Remove collected bitcoin
        }
    }
}

// Moves bitcoins downward and despawns them when off-screen
pub fn move_bitcoins(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bitcoin)>,
    time: Res<Time>,
) {
    for (entity, mut transform, bitcoin) in query.iter_mut() {
        transform.translation.y -= bitcoin.speed * time.delta_seconds();
        
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn(); // Remove off-screen bitcoin
        }
    }
}
