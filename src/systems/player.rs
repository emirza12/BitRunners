use bevy::prelude::*;
use crate::components::{Player, Obstacle, ObstacleType, ScoreText, HighScore, Difficulty, SpawnTimer};
use crate::game::GameState;

pub fn player_movement(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_x = transform.translation.x + direction * player.speed * time.delta_seconds();
    transform.translation.x = new_x.clamp(-350.0, 350.0);
}

pub fn check_obstacle_collision(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    
    for (obstacle_entity, obstacle_transform, obstacle) in obstacle_query.iter() {
        let distance = player_transform.translation.distance(obstacle_transform.translation);
        
        if distance < 40.0 {
            match obstacle.obstacle_type {
                ObstacleType::Tax => {
                    player.score = player.score.saturating_sub(1);
                },
                ObstacleType::TwitterTroll => {
                    player.score = player.score.saturating_sub(2);
                },
                ObstacleType::BearMarket => {
                    if player.lives > 0 {
                        player.lives -= 1;
                    } else {
                        game_state.set(GameState::GameOver);
                    }
                }
            }
            
            commands.entity(obstacle_entity).despawn();
            
            if player.score == 0 && player.lives == 0 {
                game_state.set(GameState::GameOver);
            }
        }
    }
}

pub fn update_score_text(
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    high_score: Res<HighScore>,
) {
    let player = player_query.single();
    let mut text = text_query.single_mut();
    
    text.sections[1].value = player.score.to_string();
    text.sections[3].value = high_score.0.to_string();
    text.sections[5].value = player.lives.to_string();
    
    if player.score > high_score.0 {
        text.sections[1].style.color = Color::rgb(1.0, 0.5, 0.0);
    }
}

pub fn update_difficulty(
    player_query: Query<&Player>,
    mut difficulty: ResMut<Difficulty>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    let player = player_query.single();
    let new_level = (player.score / difficulty.score_threshold) + 1;
    
    if new_level != difficulty.level {
        difficulty.level = new_level;
        difficulty.spawn_speed *= 0.85;
        difficulty.obstacle_speed = (difficulty.obstacle_speed * 1.2).min(300.0);
        
        // Shorter timers
        spawn_timer.bitcoin_timer.set_duration(
            std::time::Duration::from_secs_f32(3.0 * difficulty.spawn_speed)
        );
        spawn_timer.obstacle_timer.set_duration(
            std::time::Duration::from_secs_f32(1.5 * difficulty.spawn_speed)
        );
    }
} 