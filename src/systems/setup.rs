use bevy::prelude::*;
use crate::components::*;
use crate::game::GameState;

// Asset storage structure
#[derive(Resource)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub bitcoin_sprite: Handle<Image>,
    pub obstacle_sprites: Vec<Handle<Image>>,
}

pub fn setup(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    // Load assets
    let game_assets = GameAssets {
        player_sprite: asset_server.load("player.png"),
        bitcoin_sprite: asset_server.load("bitcoin.png"),
        obstacle_sprites: vec![
            asset_server.load("tax.png"),
            asset_server.load("troll.png"),
            asset_server.load("bear.png"),
        ],
    };
    commands.insert_resource(game_assets);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Initialize timers
    commands.insert_resource(SpawnTimer {
        bitcoin_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        obstacle_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
    });

    // Add invisible walls to prevent player from exiting the screen
    let wall_thickness = 20.0;
    let arena_width = 600.0;
    
    // Left wall
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-arena_width / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(wall_thickness, 600.0)),
            ..default()
        },
        ..default()
    });

    // Right wall
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(arena_width / 2.0, 0.0, 0.0),
        sprite: Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(wall_thickness, 600.0)),
            ..default()
        },
        ..default()
    });

    // Modify score text to include best score
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
            TextSection::new(
                " | Best: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..default()
                },
            ),
            TextSection::new(
                " | Life: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "1",
                TextStyle {
                    font_size: 30.0,
                    color: Color::RED,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    // Start with menu instead of game
    next_state.set(GameState::Menu);

    // Initialize high score
    commands.insert_resource(HighScore(0));

    // Initialize difficulty
    commands.insert_resource(Difficulty::default());
}

pub fn setup_player(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: game_assets.player_sprite.clone(),
            transform: Transform::from_xyz(0.0, -250.0, 0.0)
                .with_scale(Vec3::splat(0.5)),
            sprite: Sprite {
                ..default()
            },
            ..default()
        },
        Player {
            speed: 300.0,
            score: 0,
            lives: 1,
        },
    ));
} 