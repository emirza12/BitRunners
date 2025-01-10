use bevy::prelude::*;
use crate::components::{Player, Obstacle, ObstacleType, ScoreText, HighScore, Difficulty, SpawnTimer};
use crate::game::GameState;

// Handle player movement based on keyboard input
pub fn player_movement(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let mut direction = 0.0;

    // Check if the player is pressing the left or right arrow keys
    if keyboard.pressed(KeyCode::Left) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new x position of the player
    let new_x = transform.translation.x + direction * player.speed * time.delta_seconds();
    transform.translation.x = new_x.clamp(-350.0, 350.0); // Ensure the player stays within bounds
}

// Check for collisions between the player and obstacles
pub fn check_obstacle_collision(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    obstacle_query: Query<(Entity, &Transform, &Obstacle)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    
    // Check distance between player and each obstacle
    for (obstacle_entity, obstacle_transform, obstacle) in obstacle_query.iter() {
        let distance = player_transform.translation.distance(obstacle_transform.translation);
        
        // If the player is close enough to an obstacle, process the collision
        if distance < 40.0 {
            match obstacle.obstacle_type {
                // Handle different obstacle types
                ObstacleType::Tax => {
                    player.score = player.score.saturating_sub(1); // Decrease score for tax obstacles
                },
                ObstacleType::TwitterTroll => {
                    player.score = player.score.saturating_sub(2); // Decrease score for twitter troll obstacles
                },
                ObstacleType::BearMarket => {
                    // Decrease lives for bear market obstacles
                    if player.lives > 0 {
                        player.lives -= 1;
                    } else {
                        game_state.set(GameState::GameOver); // Game over if no lives left
                    }
                }
            }
            
            // Remove the obstacle from the game world
            commands.entity(obstacle_entity).despawn();
            
            // End the game if player is out of score and lives
            if player.score == 0 && player.lives == 0 {
                game_state.set(GameState::GameOver);
            }
        }
    }
}

// Update the score and high score text on the screen
pub fn update_score_text(
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    high_score: Res<HighScore>,
) {
    let player = player_query.single();
    let mut text = text_query.single_mut();
    
    // Update the text fields with the current score, high score, and remaining lives
    text.sections[1].value = player.score.to_string();
    text.sections[3].value = high_score.0.to_string();
    text.sections[5].value = player.lives.to_string();
    
    // Highlight the score in orange if it's higher than the previous high score
    if player.score > high_score.0 {
        text.sections[1].style.color = Color::rgb(1.0, 0.5, 0.0);
    }
}

// Update the game's difficulty based on the player's score
pub fn update_difficulty(
    player_query: Query<&Player>,
    mut difficulty: ResMut<Difficulty>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    let player = player_query.single();
    let new_level = (player.score / difficulty.score_threshold) + 1;
    
    // If the player has reached a new level, update the difficulty
    if new_level != difficulty.level {
        difficulty.level = new_level;
        difficulty.spawn_speed *= 0.85; // Make spawning faster
        difficulty.obstacle_speed = (difficulty.obstacle_speed * 1.2).min(300.0); // Increase obstacle speed
        
        // Adjust the spawn timers to make the game harder
        spawn_timer.bitcoin_timer.set_duration(
            std::time::Duration::from_secs_f32(3.0 * difficulty.spawn_speed)
        );
        spawn_timer.obstacle_timer.set_duration(
            std::time::Duration::from_secs_f32(1.5 * difficulty.spawn_speed)
        );
    }
}
