use bevy::prelude::*;
use bevy_stat_bars::*;

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.0)));
    let border = StatBarBorder {
        color: Color::DARK_GRAY,
        thickness: [2.0; 4],
    }.into();
    commands
        .spawn_bundle(Camera2dBundle::default())
        .commands()
        .spawn_bundle(StatBarsBundle::new(StatBars {
            bars: vec![
                StatBar { 
                    value: 0.5, 
                    length: 300.0, 
                    thickness: 30.0, 
                    style: StatBarStyle {
                        bar_color: BarColor::Fixed(Color::WHITE),
                        empty_color: Color::BLACK,
                        border,
                    },                     
                    ..Default::default()
                },
                StatBar { 
                    value: 0.5, 
                    length: 300.0, 
                    thickness: 30.0, 
                    style: StatBarStyle {
                        bar_color: BarColor::Lerp { min: Color::RED, max: Color::WHITE },
                        empty_color: Color::BLACK,
                        border,
                    }, 
                    translation: 40. * Vec2::Y, 
                    ..Default::default()
                },
                StatBar { 
                    value: 0.5, 
                    length: 300.0, 
                    thickness: 30.0, 
                    style: StatBarStyle {
                        bar_color: BarColor::Cospolate { min: Color::RED, max: Color::WHITE },
                        empty_color: Color::BLACK,
                        border,
                    }, 
                    translation: 80. * Vec2::Y, 
                    ..Default::default()
                },
                StatBar { 
                    value: 0.5, 
                    length: 300.0, 
                    thickness: 30.0, 
                    style: StatBarStyle {
                        bar_color: BarColor::LerpHSV { min: Color::RED, max: Color::WHITE },
                        empty_color: Color::BLACK,
                        border,
                    }, 
                    translation: 120. * Vec2::Y, 
                    ..Default::default()
                },
                StatBar { 
                    value: 0.5, 
                    length: 300., 
                    thickness: 30., 
                    style: StatBarStyle {
                        bar_color: BarColor::CospolateHSV { min: Color::RED, max: Color::WHITE },
                        empty_color: Color::BLACK,
                        border,
                    }, 
                    translation: 160. * Vec2::Y, 
                    ..Default::default()
                },
                StatBar { 
                    value: 0.5, 
                    length: 300., 
                    thickness: 30., 
                    style: StatBarStyle {
                        bar_color: BarColor::Function{ min: Color::RED, max: Color::WHITE, calculate_color: |min, max, value| { if value < 0.25 {
                                min
                           } else if value < 0.5 {
                                Color::YELLOW
                           } else {
                                max
                           }
                        }},
                        empty_color: Color::BLACK,
                        border,
                    }, 
                    translation: 200. * Vec2::Y, 
                    ..Default::default()
                },
            ],
            translation: -100. * Vec2::Y,
            ..Default::default()            
        }));
}

fn update_colors(
    keyboard: Res<Input<KeyCode>>,
    mut bars_query: Query<&mut StatBars>,
) {
    let bindings = [
        (Color::WHITE, KeyCode::Q, KeyCode::A),
        (Color::RED, KeyCode::W, KeyCode::S),
        (Color::GREEN, KeyCode::E, KeyCode::D),
        (Color::BLUE, KeyCode::R, KeyCode::F),
        (Color::YELLOW, KeyCode::T, KeyCode::G),
        (Color::BLACK, KeyCode::Y, KeyCode::H),
        (Color::CYAN, KeyCode::U, KeyCode::J),
        (Color::PURPLE, KeyCode::I, KeyCode::K),
        (Color::ORANGE, KeyCode::O, KeyCode::L),
    ];
    bars_query.for_each_mut(|mut bars| {
        bars.bars.iter_mut().for_each(|bar| {
            for (color, min, max) in bindings {
                if keyboard.pressed(min) {
                    bar.style.bar_color.set_min(color);
                }
                if keyboard.pressed(max) {
                    bar.style.bar_color.set_max(color);
                }
            }
        });
    });
}

fn update_bars(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut bars_query: Query<&mut StatBars>,
) { 
    let delta = 0.5 * time.delta_seconds();
    bars_query.for_each_mut(|mut bars| {
        bars.bars.iter_mut().for_each(|mut bar| {
            if keyboard.pressed(KeyCode::Left) {
                bar.value -= delta;
            }
            if keyboard.pressed(KeyCode::Right) {
                bar.value += delta;
            }
            bar.value = bar.value.clamp(0.0, 1.0);
        });
    });
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(StatBarsPlugin)
    .add_startup_system(setup)
    .add_system(update_bars)
    .add_system(update_colors)
    .run();
}