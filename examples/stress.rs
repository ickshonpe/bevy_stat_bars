use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowMode;
use bevy_stat_bars::*;
use std::marker::PhantomData;

// spawn 10_000 wizards with health and magic statbars
const GRID_SIZE: usize = 100; // 100 * 100 = 10_000

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct WizardCharacter;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Stat<T>
where
    T: 'static,
{
    pub value: f32,
    pub max: f32,
    #[reflect(ignore)]
    phantom: PhantomData<fn() -> T>,
}

impl<T> Default for Stat<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self {
            value: 10.0,
            max: 10.0,
            phantom: PhantomData,
        }
    }
}

impl<T> Stat<T>
where
    T: Component,
{
    fn new_full(value: f32) -> Self {
        assert!(0. < value);
        Self {
            value,
            max: value,
            ..Default::default()
        }
    }
}

impl<T> StatbarObservable for Stat<T>
where
    T: Component,
{
    fn get_statbar_value(&self) -> f32 {
        self.value / self.max
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct HealthValue;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct MagicValue;

type Health = Stat<HealthValue>;
type Magic = Stat<MagicValue>;

fn spawn_camera(mut commands: Commands) {
    let mut c = Camera2dBundle::default();
    c.transform.scale.x = 2.5;
    c.transform.scale.y = 2.5;
    commands.spawn_bundle(c);
}

fn spawn_wizards(mut commands: Commands, asset_server: Res<AssetServer>) {
    let s = 16.0;
    let t = (-0.5 * 1.5 * GRID_SIZE as f32 * s * Vec2::ONE).extend(0.0);
    let mut transform = Transform::from_translation(t);
    let l = s;
    let w = 3.0;
    for _ in 0..GRID_SIZE {
        for _ in 0..GRID_SIZE {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(s * Vec2::ONE),
                        ..Default::default()
                    },
                    texture: asset_server.load("wizard.png"),
                    transform,
                    ..Default::default()
                })
                .insert_bundle((
                    WizardCharacter,
                    Health::new_full(20.0),
                    Magic::new_full(17.0),
                    Statbar::<Health> {
                        empty_color: Color::rgb(0., 0.1, 0.),
                        length: l,
                        thickness: w,
                        displacement: (0.75 * s - 3.) * Vec2::Y,
                        ..Default::default()
                    },
                    StatbarBorder::<Health>::all(Color::DARK_GRAY, 1.0),
                    StatbarColorSwitch::<Health>::new(0.33, Color::RED, Color::rgb(0., 0.8, 0.)),
                    Statbar::<Magic> {
                        empty_color: Color::rgb(0.1, 0.0, 0.1),
                        length: l,
                        thickness: w,
                        displacement: (0.75 * s + 3.) * Vec2::Y,
                        ..Default::default()
                    },
                    StatbarBorder::<Magic>::all(Color::DARK_GRAY, 1.0),
                    StatbarColorLerp::<Magic>::new(Color::rgb(0.5, 0.0, 0.5), Color::FUCHSIA),
                ));
            transform.translation.x += 1.5 * s;
        }
        transform.translation.x = t.x;
        transform.translation.y += 1.5 * s;
    }
}

fn adjust_stats(
    time: Res<Time>,
    mut hp_query: Query<&mut Health>,
    mut mp_query: Query<&mut Magic>,
) {
    hp_query.for_each_mut(|mut hp| {
        hp.value = hp.max * time.time_since_startup().as_secs_f32().sin().abs();
    });
    mp_query.for_each_mut(|mut mp| {
        mp.value = mp.max * time.time_since_startup().as_secs_f32().cos().abs();
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(bevy::render::texture::ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Immediate,
            mode: WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .register_type::<Health>()
        .register_type::<Magic>()
        .register_type::<WizardCharacter>()
        .add_statbar_component_observer::<Health>()
        .add_statbar_component_observer::<Magic>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_wizards)
        .add_system(adjust_stats)
        .run();
}
