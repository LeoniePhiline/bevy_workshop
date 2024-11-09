use bevy::prelude::*;

use crate::{GameAssets, GameState};

mod player;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(player::player_plugin)
        .add_systems(OnEnter(GameState::Game), display_level);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ground;

fn display_level(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Sprite::from_atlas_image(
            assets.player_image.clone(),
            TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            },
        ),
        StateScoped(GameState::Game),
        Player,
    ));

    commands.spawn((
        Sprite::from_atlas_image(
            assets.ground_image.clone(),
            TextureAtlas {
                layout: assets.ground_layout.clone(),
                index: 14,
            },
        ),
        Transform::from_xyz(-128.0 * 3.0, -100.0, 0.0),
        Ground,
        StateScoped(GameState::Game),
    ));

    for i in -2..=2 {
        commands.spawn((
            Sprite::from_atlas_image(
                assets.ground_image.clone(),
                TextureAtlas {
                    layout: assets.ground_layout.clone(),
                    index: 7,
                },
            ),
            Transform::from_xyz(128.0 * (i as f32), -100.0, 0.0),
            Ground,
            StateScoped(GameState::Game),
        ));
    }

    commands.spawn((
        Sprite::from_atlas_image(
            assets.ground_image.clone(),
            TextureAtlas {
                layout: assets.ground_layout.clone(),
                index: 0,
            },
        ),
        Transform::from_xyz(128.0 * 3.0, -100.0, 0.0),
        Ground,
        StateScoped(GameState::Game),
    ));
}
