use bevy::prelude::*;
use rand::prelude::*;
use crate::components::*;
use super::GameAssets;

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
        
        // Réduit plus doucement la probabilité de spawn des bitcoins avec le niveau
        let spawn_chance = 1.0 - (difficulty.level as f32 * 0.05).min(0.6); // Min 40% de chance au lieu de 10%
        
        if rng.gen_range(0.0..1.0) < spawn_chance {
            let x = rng.gen_range(-300.0..300.0);
            
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

pub fn collect_bitcoins(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    bitcoin_query: Query<(Entity, &Transform, &Bitcoin)>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    
    for (bitcoin_entity, bitcoin_transform, bitcoin) in bitcoin_query.iter() {
        let distance = player_transform.translation.distance(bitcoin_transform.translation);
        
        if distance < 40.0 {
            player.score += bitcoin.value;
            commands.entity(bitcoin_entity).despawn();
        }
    }
}

pub fn move_bitcoins(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bitcoin)>,
    time: Res<Time>,
) {
    for (entity, mut transform, bitcoin) in query.iter_mut() {
        transform.translation.y -= bitcoin.speed * time.delta_seconds();
        
        if transform.translation.y < -300.0 {
            commands.entity(entity).despawn();
        }
    }
} 