use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use bevy_stat_bars::*;

struct StatbarMarker<const N: usize>;

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_statbars(mut commands: Commands) {
    let length = 16.;
    let space = 2.;
    let thickness = 2.;
    let s = -0.5 * vec2(100. * (length + space), 200. * (space + space));
    let mut displacement = s;

    for _ in 0..100 {
        let mut entity_commands = commands.spawn_bundle(SpatialBundle::default());
        seq_macro::seq!(N in 0 .. 200 {
            entity_commands.insert(Statbar::<StatbarMarker<N>> {
                color: Color::WHITE,
                empty_color: Color::BLUE,
                length,
                thickness,
                displacement,
                ..Default::default()
            })
            .insert(StatbarColorLerp::<StatbarMarker<N>>::new(Color::RED, Color::WHITE)) ;
            displacement.y += thickness + space;
        });
        displacement.y = s.y;
        displacement.x += length + space;
    }
}

fn adjust_stats<const N: usize>(
    time: Res<Time>,
    mut statbar: Query<&mut Statbar<StatbarMarker<N>>>,
) {
    statbar.for_each_mut(|mut bar| {
        bar.value = time.time_since_startup().as_secs_f32().sin().abs();
    });
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.5, 0.0)))
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Immediate,
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_statbars);

    seq_macro::seq!(N in 0  .. 200 {
        app.add_standalone_statbar::<StatbarMarker<N>>()
            .add_system(adjust_stats::<N>);
    });

    app.run();
}
