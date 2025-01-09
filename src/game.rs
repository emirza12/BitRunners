use bevy::prelude::*;
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, handle_menu.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(Update, handle_game_over.run_if(in_state(GameState::GameOver)))
            .add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(
                Update,
                (
                    player_movement,
                    spawn_bitcoins,
                    move_bitcoins,
                    collect_bitcoins,
                    spawn_obstacles,
                    move_obstacles,
                    check_obstacle_collision,
                    update_score_text,
                    update_difficulty,
                ).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
} 