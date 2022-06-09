use bevy::prelude::*;
use bevy_stat_bars::*;

fn spawn_a_stat_bar(mut commands: Commands) {
    commands.spawn_bundle((
        StatBar {
            value: 0.75,
            size: Vec2::new(200., 20.),
            ..Default::default()
        },
    ));
}

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        width: 500.,
        height: 300.,
        scale_factor_override: Some(2.0),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.1, 0.0)))
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })   
    .add_startup_system(spawn_a_stat_bar)
    .run();
}