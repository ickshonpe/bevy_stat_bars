use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_stat_bars::*;

#[derive(Component)]
struct PlayerCharacter;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Hp { current: i32, max: i32 }

#[derive(Component)]
struct Mp { current: i32, max: i32 }

fn spawn_player(
    mut commands: Commands
) {
    let displacement = [-40.0 * Vec2::X, 50.0 * Vec2::Y];
    let local = [8.0 * Vec2::Y, 8.0 * Vec2::Y];
    for i in 0..2 {
        let mut e = commands
            .spawn_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(vec2(32.0, 64.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(100.0 * Vec3::X * if i == 0 { -1. } else { 1. }),
                    ..Default::default()
                },
            );
        e.insert_bundle((
            Speed(250.0),
            Hp { current: 30, max: 30 },
            Mp { current: 12, max: 15 },
            StatBars{
                displacement: displacement[i],
                rotation: 0.0,
                bars: vec![
                    StatBar {                    
                        size: vec2(48.0, 8.0),
                        displacement: -local[i],
                        ..Default::default()
                    },
                    StatBar {
                        style: StatBarStyle { 
                            full_color: Color::PURPLE, 
                            empty_color: Color::DARK_GREEN, 
                            ..Default::default()
                        },
                        displacement: local[i],
                        size: vec2(48.0, 8.0),
                        ..Default::default()
                    },
                ],
            }
        ));
        if i == 0 {
            e.insert(PlayerCharacter);
        }
    }   
}

fn move_player(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<PlayerCharacter>>
) {
    query.for_each_mut(|(mut transform, player_speed)| {
        let mut m = Vec3::ZERO;
        if keyboard.pressed(KeyCode::A) {
            m -= Vec3::X
        } 
        if keyboard.pressed(KeyCode::D) {
            m += Vec3::X
        }
        if keyboard.pressed(KeyCode::S) {
            m -= Vec3::Y
        }
        if keyboard.pressed(KeyCode::W) {
            m += Vec3::Y
        }
        transform.translation += time.delta_seconds() * player_speed.0 * m.normalize_or_zero();
    });
}

fn update_stats(
    mut cooldown: Local<f32>,
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut players: Query<(&mut Hp, &mut Mp)>,
) {
    *cooldown -= time.delta_seconds();
    if 0.0 < *cooldown { 
        return; 
    } else {
        *cooldown = 0.1;
    }
    players.for_each_mut(|(mut hp, mut mp)| {
        if keyboard.pressed(KeyCode::Down) {
            hp.current = (hp.current - 1).clamp(0, hp.max);
        } 
        if keyboard.pressed(KeyCode::Up) {
            hp.current = (hp.current + 1).clamp(0, hp.max);
        }
        if keyboard.pressed(KeyCode::Left) {
            mp.current = (mp.current - 1).clamp(0, mp.max);
        }
        if keyboard.pressed(KeyCode::Right) {
            mp.current = (mp.current + 1).clamp(0, mp.max);
        }
    });
}

fn update_bars(
    mut query: Query<(&Hp, &Mp, &mut StatBars)>,
) {
    query.for_each_mut(|(hp, mp, mut bars)| {
        bars[0].value = hp.current as f32 / hp.max as f32;
        bars[1].value = mp.current as f32 / mp.max as f32;
    });
}

fn reverse(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut StatBars, With<PlayerCharacter>>,
) {
    query.for_each_mut(|mut bars| {
        if keyboard.just_pressed(KeyCode::N) {
            bars[0].rotation += PI;
        }
        if keyboard.just_pressed(KeyCode::M) {
            bars[1].rotation += PI;
        }
    });
}

fn rotate_individually(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut StatBars, With<PlayerCharacter>>,
) {
    query.for_each_mut(|mut bars| {
        if keyboard.pressed(KeyCode::J) {
            bars[0].rotation += time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::K) {
            bars[0].rotation -= time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::I) {
            bars[1].rotation += time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::O) {
            bars[1].rotation -= time.delta_seconds();
        }
    });
}

fn rotate_all(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut StatBars, With<PlayerCharacter>>,
) {
    query.for_each_mut(|mut bars| {
        if keyboard.pressed(KeyCode::V) {
            bars.rotation += time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::B) {
            bars.rotation -= time.delta_seconds();
        }
    });
}

fn death(
    mut commands: Commands,
    query: Query<(Entity, &Hp)>,
) {
    query.for_each(|(entity, hp)| {
        if hp.current <= 0 {
            commands.entity(entity).despawn();
        }
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
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })   
    .add_startup_system(spawn_player)
    .add_system(move_player)
    .add_system(death)
    .add_system(update_stats)
    .add_system(update_bars)
    .add_system(reverse)
    .add_system(rotate_all)
    .add_system(rotate_individually)
    .run();
}