use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub score: u32,
    pub lives: u32,
}

#[derive(Component)]
pub struct Bitcoin {
    pub value: u32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Obstacle {
    pub damage: f32,
    pub obstacle_type: ObstacleType,
    pub speed: f32,
}

pub enum ObstacleType {
    Tax,
    TwitterTroll,
    BearMarket,
}

#[derive(Resource)]
pub struct SpawnTimer {
    pub bitcoin_timer: Timer,
    pub obstacle_timer: Timer,
}

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Resource)]
pub struct HighScore(pub u32);

#[derive(Resource)]
pub struct Difficulty {
    pub level: u32,
    pub score_threshold: u32,
    pub spawn_speed: f32,
    pub obstacle_speed: f32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            level: 1,
            score_threshold: 3,
            spawn_speed: 1.0,
            obstacle_speed: 100.0,
        }
    }
}

impl Difficulty {
    pub fn reset(&mut self) {
        self.level = 1;
        self.spawn_speed = 1.0;
        self.obstacle_speed = 100.0;
    }
}

#[derive(Component)]
pub struct OnGameScreen; 