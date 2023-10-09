


use bevy::prelude::*;
use crate::components::{Player, Velocity};
use crate::{BASE_SPEED, TIME_STEP};

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
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("Character_animation/priests_idle/priest1/v2/priest1_v2_1.png"),
        transform: Transform {
            scale: Vec3::new(5., 5., 1.),
            ..default()
        },

        //sprite: Sprite {
        //    custom_size: Some(Vec2::new(60.0, 100.0)),
        //    ..default()
        //},
        ..default()
    })
        .insert(Player)
        .insert(Velocity {x: 0.0, y: 0.0 } );
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