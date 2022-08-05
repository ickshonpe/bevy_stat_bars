use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use copyless::VecHelper;
use crate::*;

pub (super) fn extract_stat_bars_to_sprites(
    stat_bar_query: Extract<Query<(Entity, &StatBar, Option<&GlobalTransform>)>>,
    stat_bars_query: Extract<Query<(Entity, &StatBars, Option<&GlobalTransform>)>>,
    z_depth: Extract<Res<StatBarZDepth>>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
) {
   
    let mut extract_stat_bar = |entity: Entity, bar: &StatBar, g_translation: Vec2, g_orientation: f32| {
        let local_rotation = Quat::from_rotation_z(bar.rotation); 
        let global_rotation = Quat::from_rotation_z(g_orientation);    
        let total_rotation = global_rotation.inverse() * local_rotation;
        let scale = Vec3::ONE;        
        let bar_position = g_translation.extend(z_depth.0) + global_rotation.inverse() * bar.translation.extend(0.0);
        let style = &bar.style;
        if let Some(border) = &style.border {            
            let left_size = border.thickness[0] * Vec2::X + bar.thickness * Vec2::Y;
            let left_position = - 0.5 * (bar.length + left_size.x) * Vec3::X; 
            let right_size = border.thickness[1] * Vec2::X + bar.thickness * Vec2::Y;
            let right_position = 0.5 * (bar.length + right_size.x) * Vec3::X; 
            let bottom_size = (border.thickness[0] + border.thickness[1] + bar.length) * Vec2::X + border.thickness[2] * Vec2::Y;
            let bottom_position = - 0.5 * (bar.thickness + bottom_size.y) * Vec3::Y;
            let top_size = (border.thickness[0] + border.thickness[1] + bar.length) * Vec2::X + border.thickness[3] * Vec2::Y;
            let top_position =  0.5 * (bar.thickness + top_size.y) * Vec3::Y;
            [
                (left_size, left_position),
                (right_size, right_position),
                (bottom_size, bottom_position),
                (top_size, top_position)
            ]
            .into_iter()
            .for_each(|(size, translation)| {
                extracted_sprites.sprites.alloc().init(
                    ExtractedSprite {
                        entity,
                        transform: Transform {
                            translation: bar_position + global_rotation.inverse() * local_rotation * translation, 
                            rotation: total_rotation,
                            scale,
                        }.into(),
                        color: border.color,
                        rect: None,
                        custom_size: Some(size),
                        image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                        flip_x: false,
                        flip_y: false,
                        anchor: Default::default()
                    }
                );
            });
        }
        if bar.value < 1.0 {
            let value = bar.value.max(0.0);
            let size = (1.0 - bar.value) * bar.length * Vec2::X + bar.thickness * Vec2::Y;
            let translation = 0.5 * (bar.length - (1.0 - value) * bar.length) * Vec3::X;// + bar_translation;
            extracted_sprites.sprites.alloc().init(
                ExtractedSprite {
                    entity,
                    transform: Transform {
                        translation: bar_position + total_rotation * translation,
                        rotation: total_rotation,
                        scale,
                    }.into(),
                    color: style.empty_color,
                    rect: None,
                    custom_size: Some(size),
                    image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                    flip_x: false,
                    flip_y: false,
                    anchor: Default::default()
                }
            );
        }
        if 0.0 < bar.value {
            let value = bar.value.min(1.0);
            let color = match style.bar_color {
                BarColor::Fixed(color) => color,
                BarColor::Lerp { min, max } => {
                    let min = Vec4::from(min);
                    let max = Vec4::from(max);
                    min.lerp(max, value).into()
                },
                BarColor::Cospolate { min, max } => {
                    let min = Vec4::from(min);
                    let max = Vec4::from(max);
                    let s = (1. - (value * PI).cos()) / 2.;
                    ((1. - s) * min + s * max).into()
                },
                BarColor::LerpHSV { min, max } => {
                    let min = Vec4::from(min.as_hsla_f32());
                    let max = Vec4::from(max.as_hsla_f32());
                    let out = min.lerp(max, value);
                    Color::hsla(
                        out[0], 
                        out[1], 
                        out[2], 
                        out[3]
                    )
                },  
                BarColor::CospolateHSV { min, max } => {
                    let min = Vec4::from(min.as_hsla_f32());
                    let max = Vec4::from(max.as_hsla_f32());
                    let s = (1. - (value * PI).cos()) / 2.;
                    let out = (1. - s) * min + s * max;
                    Color::hsla(
                        out[0], 
                        out[1], 
                        out[2], 
                        out[3]
                    )
                },
                BarColor::Function{ min, max, calculate_color } => calculate_color(min, max, value)
            };
            let size = value * bar.length * Vec2::X + bar.thickness * Vec2::Y;
            let translation = 
                0.5 * bar.length * (value - 1.0) * Vec3::X;
            extracted_sprites.sprites.alloc().init(
                ExtractedSprite {
                    entity,
                    transform: Transform {
                        translation: bar_position + total_rotation * translation,
                        rotation: total_rotation,
                        scale,
                    }.into(),
                    color,
                    rect: None,
                    custom_size: Some(size),
                    image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                    flip_x: false,
                    flip_y: false,
                    anchor: Default::default()
                }
            );
        }
    };
    for (id, bars, transform) in stat_bars_query.iter() {
        for bar in bars.iter() {
            let g_translation = transform.map(|tf| 
                tf.compute_transform()
                .translation.truncate())
                .unwrap_or_default() + bars.translation;
            extract_stat_bar(id, bar, g_translation, bars.rotation);
        }
    }
    for (id, bar, transform) in stat_bar_query.iter() {
        let g_translation = transform.map(|tf| 
            tf.compute_transform()
            .translation.truncate())
                .unwrap_or_default();
        extract_stat_bar(id, bar, g_translation, 0.0);
    }
}
