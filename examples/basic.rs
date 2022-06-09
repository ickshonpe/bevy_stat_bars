use bevy::math::vec2;
use bevy::prelude::*;
use bevy_stat_bars::*;

fn setup(
    mut commands: Commands
) {
    commands
        .spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(vec2(32.0, 64.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(-60. * Vec3::X),
                ..Default::default()
            },
        )
        .insert(StatBar {
            value: 0.75,
            displacement: 40.0 * Vec2::Y,
            size: vec2(40., 6.),
            ..Default::default()
        });

    commands
    .spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(vec2(32.0, 64.0)),                
                ..Default::default()
            },
            transform: Transform::from_translation(-25. * Vec3::Y),
            ..Default::default()
        },
    )
    .insert(StatBar {
        value: 0.5,
        displacement: 24.0 * Vec2::X,
        size: vec2(40., 6.),
        rotation: 0.5 * std::f32::consts::PI,        
        style: StatBarStyle {
            full_color: Color::RED,
            empty_color: Color::MAROON,
            border: StatBarBorder::new(Color::DARK_GRAY, 2.0).into(),
        },
        ..Default::default()
    });

    commands
    .spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(vec2(32.0, 64.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(60. * Vec2::ONE.extend(0.)),
            ..Default::default()
        }
    )
    .insert(StatBar {
        value: 0.3,
        displacement: -16.0 * Vec2::Y,
        size: vec2(80., 12.),
        style: StatBarStyle {
            full_color: Color::CYAN,
            empty_color: Color::NAVY,
            border: StatBarBorder::new(Color::RED, 2.0).into(),
        },
        ..Default::default()
    })
    .insert(StatBars {
        bars: vec![
            StatBar {
                value: 0.9,
                displacement: -12.0 * Vec2::Y,
                style: StatBarStyle {
                    full_color: Color::GREEN,
                    empty_color: Color::OLIVE,
                    border: StatBarBorder::new(Color::SEA_GREEN, 1.0).into(),
                },
                ..Default::default()
            },
            StatBar {
                value: 0.6,
                style: StatBarStyle {
                    full_color: Color::GREEN,
                    empty_color: Color::OLIVE,
                    border: StatBarBorder::new(Color::SEA_GREEN, 1.0).into(),
                },
                ..Default::default()
            },
            StatBar {
                value: 0.75,
                displacement: 12.0 * Vec2::Y,
                style: StatBarStyle {
                    full_color: Color::GREEN,
                    empty_color: Color::OLIVE,
                    border: StatBarBorder::new(Color::SEA_GREEN, 1.0).into(),
                },
                ..Default::default()
            },
        ],
        rotation: -0.75 * std::f32::consts::PI,
        ..Default::default()
    });
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
    .add_startup_system(setup)
    .run();
}