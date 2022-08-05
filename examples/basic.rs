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
            translation: 40.0 * Vec2::Y,
            length: 40.,
            thickness: 6.,
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
        translation: 24.0 * Vec2::X,
        length: 40.,
        thickness: 6.,
        rotation: 0.5 * std::f32::consts::PI,        
        style: StatBarStyle {
            bar_color: Color::RED.into(),
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
        translation: -16.0 * Vec2::Y,
        length: 80.,
        thickness: 12.,
        style: StatBarStyle {
            bar_color: Color::CYAN.into(),
            empty_color: Color::NAVY,
            border: StatBarBorder::new(Color::RED, 2.0).into(),
        },
        ..Default::default()
    })
    .insert_bundle(StatBarsBundle::new( StatBars {
        bars: vec![
            StatBar {
                value: 0.9,
                translation: -12.0 * Vec2::Y,
                style: StatBarStyle {
                    bar_color: Color::GREEN.into(),
                    empty_color: Color::OLIVE,
                    border: StatBarBorder::new(Color::SEA_GREEN, 1.0).into(),
                },
                ..Default::default()
            },
            StatBar {
                value: 0.6,
                style: StatBarStyle {
                    bar_color: Color::GREEN.into(),
                    empty_color: Color::OLIVE,
                    border: StatBarBorder::new(Color::SEA_GREEN, 1.0).into(),
                },
                ..Default::default()
            },
            StatBar {
                value: 0.75,
                translation: 12.0 * Vec2::Y,
                style: StatBarStyle {
                    bar_color: Color::GREEN.into(),
                    empty_color: Color::OLIVE,
                    border: StatBarBorder::new(Color::SEA_GREEN, 1.0).into(),
                },
                ..Default::default()
            },
        ],
        rotation: -0.75 * std::f32::consts::PI,
        ..Default::default()
    }));
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
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(Camera2dBundle::default()); })   
    .add_startup_system(setup)
    .run();
}