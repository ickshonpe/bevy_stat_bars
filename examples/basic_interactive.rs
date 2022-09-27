use bevy::prelude::*;
use bevy_stat_bars_3::*;

// Spawns a red and navy statbar with a white border in the middle of the window.
// The left and right cursor keys decrease and increase the value of the bar.

/// A minimal newtype struct that implements `StatbarObservable`
#[derive(Copy, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct ObservedValue(pub f32);

impl StatbarObservable for ObservedValue {
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
            Statbar::<ObservedValue> {
                color: Color::RED,
                empty_color: Color::NAVY,
                length: 400.,
                thickness: 40.,
                ..Default::default()
            },
            StatbarBorder::<ObservedValue>::all(Color::WHITE, 4.0),
            ObservedValue(0.35),
        ))
        .insert_bundle(SpatialBundle::default());
}

fn adjust_value(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut observed_values: Query<&mut ObservedValue>,
) {
    let delta = time.delta_seconds() * 0.25;
    observed_values.for_each_mut(|mut value| {
        if input.pressed(KeyCode::Left) {
            value.0 -= delta;
        }
        if input.pressed(KeyCode::Right) {
            value.0 += delta;
        }
        value.0 = value.0.clamp(0., 1.0);
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_statbar_component_observer::<ObservedValue>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_statbar)
        .add_system(adjust_value)
        .run();
}
