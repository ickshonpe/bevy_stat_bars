use crate::*;
use bevy::math::vec2;
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::render::Extract;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use copyless::VecHelper;

/// The z depth the stat bar sprites are drawn with.
const DEFAULT_Z_DEPTH: f32 = 999.0;

pub(crate) fn extract_stat_bars<V>(
    extraction: Extract<(
        Option<Res<StatbarDepth>>,
        Query<(
            Entity,
            &Statbar<V>,
            Option<&StatbarBorder<V>>,
            &GlobalTransform,
            &ComputedVisibility,
        )>,
    )>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
) {
    let (depth, query) = &*extraction;
    for (id, bar, border, global_transform, computed_visibility) in query.iter() {
        if bar.hide || !computed_visibility.is_visible() {
            continue;
        }
        let (major_axis, minor_axis) = if bar.vertical {
            (Vec2::Y, Vec2::X)
        } else {
            (Vec2::X, Vec2::Y)
        };

        let value = bar.value;
        let length = bar.length;
        let thickness = bar.thickness;
        let mut transform = *global_transform;
        transform.translation_mut().z = depth
            .as_ref()
            .map(|depth| depth.0)
            .unwrap_or(DEFAULT_Z_DEPTH);
        transform.translation_mut().x += bar.displacement.x;
        transform.translation_mut().y += bar.displacement.y;
        let size = length * major_axis + thickness * minor_axis;
        if let Some(border) = border {
            let border_size = vec2(
                size.x + border.left + border.right,
                size.y + border.bottom + border.top,
            );
            extracted_sprites.sprites.alloc().init(ExtractedSprite {
                entity: id,
                transform,
                color: border.color,
                rect: None,
                custom_size: Some(border_size),
                image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                flip_x: false,
                flip_y: false,
                anchor: Default::default(),
            });
        }

        // draw bar back
        if value < 1.0 {
            extracted_sprites.sprites.alloc().init(ExtractedSprite {
                entity: id,
                transform,
                color: bar.empty_color,
                rect: None,
                custom_size: Some(size),
                image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                flip_x: false,
                flip_y: false,
                anchor: Default::default(),
            });
        }

        // draw bar
        if 0.0 < value {
            let value = value.clamp(0., 1.);
            let bar_size = value * length * major_axis + thickness * minor_axis;
            let direction = if bar.reverse { -1. } else { 1. };
            *transform.translation_mut() +=
                Vec3A::from(direction * 0.5 * length * (value - 1.) * major_axis.extend(0.));
            extracted_sprites.sprites.alloc().init(ExtractedSprite {
                entity: id,
                transform,
                color: bar.color,
                rect: None,
                custom_size: Some(bar_size),
                image_handle_id: DEFAULT_IMAGE_HANDLE.into(),
                flip_x: false,
                flip_y: false,
                anchor: Default::default(),
            });
        }
    }
}
