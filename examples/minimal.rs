use bevy::prelude::*;
use bevy_stat_bars::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(|mut commands: Commands| { 
        commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle((StatBar {
            value: 0.75,
            size: Vec2::new(200., 20.),
            ..Default::default()
        },));
    })   
    .run();
}