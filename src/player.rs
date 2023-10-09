


use bevy::prelude::*;
use bevy_sprite3d::*;
use bevy_asset_loader::prelude::*;
use crate::components::{Player, Velocity};
use crate::{BASE_SPEED, TIME_STEP, WinSize, ImageAssets};
use bevy_sprite3d::Sprite3dParams;
use bevy_sprite3d::Sprite3d;
pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_spawn_system);
        app.add_system(player_movement_system);
        app.add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    image_assets: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
    //win_size: Res<WinSize>,
) {
    commands.spawn(Sprite3d {
        image: image_assets.player_sprite.clone(),

        pixels_per_metre: 400.,

        alpha_mode: AlphaMode::Blend,

        unlit: true,

        // transform: Transform::from_xyz(0., 0., 0.),
        // pivot: Some(Vec2::new(0.5, 0.5)),

        ..default()
    }.bundle(&mut sprite_params));
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.0
        } else if kb.pressed(KeyCode::Right) {
            1.0
        } else {
            0.0
        };

        velocity.y = if kb.pressed(KeyCode::Up) {
            1.0
        } else if kb.pressed(KeyCode::Down) {
            -1.0
        } else {
            0.0
        };
    }
}