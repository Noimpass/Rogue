mod player;
mod components;

use bevy::prelude::*;
use player::PlayerPlugin;

const TIME_STEP: f32 = 1./60.;
const BASE_SPEED: f32 = 500.;

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
        .add_systems(Startup, setup)
        .add_plugin(PlayerPlugin)
        .run()
}



fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

}