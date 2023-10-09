mod player;
mod components;

use bevy::prelude::*;
use player::PlayerPlugin;
use bevy_sprite3d::Sprite3dPlugin;
use bevy_asset_loader::prelude::AssetCollection;
const TIME_STEP: f32 = 1./60.;
const BASE_SPEED: f32 = 200.;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "Character_animation/priests_idle/priest1/v1/priest1_v1_1.png")]
    pub player_sprite: Handle<Image>,
}

pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some( Window {
                        title: "Rogue".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
        )
        .add_plugin(Sprite3dPlugin)
        .add_systems(Startup, setup)
        .add_plugin(PlayerPlugin)
        .run()
}



fn setup(
    mut commands: Commands,
) {
    //let win_size = WinSize { w: 640.0, h: 480.0 };
    //commands.insert_resource(win_size);
    commands.spawn(Camera3dBundle::default())
        .insert(Transform::from_xyz(0., 0., 5.));

}