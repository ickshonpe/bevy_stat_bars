use bevy::prelude::*;
use bevy_stat_bars::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(|mut commands: Commands| { 
        commands
        .spawn_bundle(Camera2dBundle::default())
        .commands()
        .spawn_bundle(StatBarBundle::new(
            StatBar {
                value: 0.75,
                length: 200.,
                thickness: 20.,
                ..Default::default()
            }
        ));
    })
    .run();
}