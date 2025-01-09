use bevy::prelude::*;
use crate::game::GameState;
use super::GameAssets;

#[derive(Component)]
pub struct MenuText;

pub fn setup_menu(mut commands: Commands, game_assets: Res<GameAssets>) {
    let text_style = TextStyle {
        font_size: 30.0,
        color: Color::WHITE,
        ..default()
    };

    let text_style_red = TextStyle {
        font_size: 30.0,
        color: Color::RED,
        ..default()
    };

    commands.spawn(NodeBundle {
        style: Style {
            left: Val::Px(200.0),
            top: Val::Px(120.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..default()
    }).with_children(|parent| {
        // Titre
        parent.spawn(TextBundle::from_section(
            "BitRunners\n\n",
            TextStyle {
                font_size: 50.0,
                color: Color::GOLD,
                ..default()
            },
        ));

        // Bitcoin section
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|row| {
            row.spawn(ImageBundle {
                image: UiImage::new(game_assets.bitcoin_sprite.clone()),
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
            row.spawn(TextBundle::from_section(
                "Collect to increase score\n\n",
                text_style.clone(),
            ));
        });

        // Avoid text
        parent.spawn(TextBundle::from_section(
            "Dangers:\n\n",
            text_style.clone(),
        ));

        // Tax section
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|row| {
            row.spawn(ImageBundle {
                image: UiImage::new(game_assets.obstacle_sprites[0].clone()),
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
            row.spawn(TextBundle::from_section(
                "Tax collector (lose 1 point)\n\n",
                text_style_red.clone(),
            ));
        });

        // Troll section
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|row| {
            row.spawn(ImageBundle {
                image: UiImage::new(game_assets.obstacle_sprites[1].clone()),
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
            row.spawn(TextBundle::from_section(
                "Twitter Trolls (lose 2 points)\n\n",
                text_style_red.clone(),
            ));
        });

        // Bear section
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }).with_children(|row| {
            row.spawn(ImageBundle {
                image: UiImage::new(game_assets.obstacle_sprites[2].clone()),
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    margin: UiRect::right(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            });
            row.spawn(TextBundle::from_section(
                "Bear Market (lose 1 life)\n\n",
                text_style_red.clone(),
            ));
        });

        // Instructions de déplacement
        parent.spawn(TextBundle::from_section(
            "\nUse <- -> arrows to move\n\n",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Start text
        parent.spawn(TextBundle::from_section(
            "Press SPACE to start",
            TextStyle {
                font_size: 40.0,
                color: Color::GREEN,
                ..default()
            },
        ));
    }).insert(MenuText);
}

pub fn handle_menu(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    menu_query: Query<Entity, With<MenuText>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // Remove all menu elements
        for entity in menu_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        // Switch to Playing state
        next_state.set(GameState::Playing);
    }
} 