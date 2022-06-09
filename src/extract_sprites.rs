use bevy::prelude::*;
use bevy::render::RenderWorld;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use copyless::VecHelper;
use crate::*;

pub (super) fn extract_stat_bars_to_sprites(
    mut render_world: ResMut<RenderWorld>,
    stat_bar_query: Query<(&StatBar, Option<&GlobalTransform>)>,
    stat_bars_query: Query<(&StatBars, Option<&GlobalTransform>)>,
    z_depth: Res<StatBarZDepth>,
) {
    let mut extracted_sprites = render_world.get_resource_mut::<ExtractedSprites>().unwrap();
    let mut extract_stat_bar = |bar: &StatBar, g_translation: Vec2, g_orientation: f32| {
        let local_rotation = Quat::from_rotation_z(bar.rotation); 
        let global_rotation = Quat::from_rotation_z(g_orientation);    
        let total_rotation = global_rotation.inverse() * local_rotation;
        let scale = Vec3::ONE;        
        let bar_position = g_translation.extend(z_depth.0) + global_rotation.inverse() * bar.displacement.extend(0.0);
        let style = &bar.style;
        if let Some(border) = &style.border {            
            let left_size = border.thickness[0] * Vec2::X + bar.size.y * Vec2::Y;
            let left_position = - 0.5 * (bar.size.x + left_size.x) * Vec3::X; 
            let right_size = border.thickness[1] * Vec2::X + bar.size.y * Vec2::Y;
            let right_position = 0.5 * (bar.size.x + right_size.x) * Vec3::X; 
            let bottom_size = (border.thickness[0] + border.thickness[1] + bar.size.x) * Vec2::X + border.thickness[2] * Vec2::Y;
            let bottom_position = - 0.5 * (bar.size.y + bottom_size.y) * Vec3::Y;
            let top_size = (border.thickness[0] + border.thickness[1] + bar.size.x) * Vec2::X + border.thickness[3] * Vec2::Y;
            let top_position =  0.5 * (bar.size.y + top_size.y) * Vec3::Y;
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
                        transform: GlobalTransform {
                            translation: bar_position + global_rotation.inverse() * local_rotation * translation, 
                            rotation: total_rotation,
                            scale,
                        },
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
            let size = (1.0 - bar.value) * bar.size.x * Vec2::X + bar.size.y * Vec2::Y;
            let translation = 0.5 * (bar.size.x - (1.0 - value) * bar.size.x) * Vec3::X;// + bar_translation;
            extracted_sprites.sprites.alloc().init(
                ExtractedSprite {
                    transform: GlobalTransform {
                        translation: bar_position + total_rotation * translation,
                        rotation: total_rotation,
                        scale,
                    },
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
            let size = value * bar.size.x * Vec2::X + bar.size.y * Vec2::Y;
            let translation = 
                0.5 * bar.size.x * (value - 1.0) * Vec3::X;
            extracted_sprites.sprites.alloc().init(
                ExtractedSprite {
                    transform: GlobalTransform {
                        translation: bar_position + total_rotation * translation,
                        rotation: total_rotation,
                        scale,
                    },
                    color: style.full_color,
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
    for (bars, transform) in stat_bars_query.iter() {
        for bar in bars.iter() {
            let g_translation = transform.map(|tf| tf.translation.truncate())
                .unwrap_or_default() + bars.displacement;
            extract_stat_bar(bar, g_translation, bars.rotation);
        }
    }
    for (bar, transform) in stat_bar_query.iter() {
        let g_translation = transform.map(|tf| tf.translation.truncate())
                .unwrap_or_default();
        extract_stat_bar(bar, g_translation, 0.0);
    }
}
