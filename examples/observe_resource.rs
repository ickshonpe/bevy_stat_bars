use bevy::prelude::*;
use bevy_stat_bars_3::*;

// Spawns a red and navy statbar with a white border in the middle of the window.
// The left and right cursor keys decrease and increase the value of the bar.

struct ObservedResource(f32);

impl StatbarObservable for ObservedResource {
    fn get_statbar_value(&self) -> f32 {
        self.0
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_statbar(mut commands: Commands) {
    commands
        .spawn_bundle((
            Statbar::<ObservedResource> {
                color: Color::RED,
                empty_color: Color::NAVY,
                length: 500.,
                thickness: 50.,
                vertical: true,
                ..Default::default()
            },
            StatbarBorder::<ObservedResource>::all(Color::WHITE, 10.0),
        ))
        .insert_bundle(SpatialBundle::default());
}

fn adjust_value(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut my_resource: ResMut<ObservedResource>,
) {
    let delta = time.delta_seconds() * 0.25;
    if input.pressed(KeyCode::Down) {
        my_resource.0 -= delta;
    }
    if input.pressed(KeyCode::Up) {
        my_resource.0 += delta;
    }
    my_resource.0 = my_resource.0.clamp(0., 1.0);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ObservedResource(1.0))
        .add_statbar_resource_observer::<ObservedResource>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_statbar)
        .add_system(adjust_value)
        .run();
}
