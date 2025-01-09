use bevy::prelude::*;
use crate::components::*;
use crate::game::GameState;

pub fn setup_game_over(
    mut commands: Commands,
    player_query: Query<&Player>,
    mut high_score: ResMut<HighScore>,
) {
    let player = player_query.single();
    if player.score > high_score.0 {
        high_score.0 = player.score;
    }

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                format!("Game Over!\nScore: {}\nBest: {}\n\nPress SPACE to restart", 
                    player.score, high_score.0),
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(35.0),
            top: Val::Percent(40.0),
            ..default()
        }),
        GameOverText,
    ));
}

pub fn handle_game_over(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_over_text: Query<Entity, With<GameOverText>>,
    player_query: Query<Entity, With<Player>>,
    obstacles: Query<Entity, With<Obstacle>>,
    bitcoins: Query<Entity, With<Bitcoin>>,
    mut difficulty: ResMut<Difficulty>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Nettoyer l'écran
        for entity in game_over_text.iter() {
            commands.entity(entity).despawn();
        }

        // Nettoyer les entités
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in obstacles.iter() {
            commands.entity(entity).despawn();
        }
        for entity in bitcoins.iter() {
            commands.entity(entity).despawn();
        }

        // Réinitialiser la difficulté
        difficulty.reset();
        
        // Réinitialiser les timers
        spawn_timer.bitcoin_timer.set_duration(std::time::Duration::from_secs_f32(1.5));
        spawn_timer.obstacle_timer.set_duration(std::time::Duration::from_secs_f32(2.0));

        // Redémarrer le jeu
        next_state.set(GameState::Playing);
    }
} 