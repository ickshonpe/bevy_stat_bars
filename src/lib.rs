pub mod extract_sprites;

use bevy::math::vec2;
use bevy::prelude::*;

/// using the sprite renderer to draw the stat bars
/// all stat bars share the same depth
pub struct StatBarZDepth(pub f32);

impl Default for StatBarZDepth {
    fn default() -> Self {
        Self(999.0)
    }
}

#[derive(Clone, Component)]
pub struct StatBarStyle {
    /// color of the full part of the bar
    pub full_color: Color,
    /// color of the empty part of the bar
    pub empty_color: Color,
    /// None = no border
    pub border: Option<StatBarBorder>
}

impl Default for StatBarStyle {
    fn default() -> Self {
        Self { 
            full_color: Color::ORANGE, 
            empty_color: Color::rgb(0.2, 0.1, 0.0), 
            border: StatBarBorder::default().into(),
        }
    }
}

pub struct StatBarOrientation(pub f32);

#[derive(Clone, Component)]
pub struct StatBarBorder {
    /// color of the border
    pub color: Color,
    /// Thickness of the border on edges
    /// `[left, right, bottom, top]` respectively.
    pub thickness: [f32; 4],
}

impl Default for StatBarBorder {
    fn default() -> Self {
        Self { 
            color: Color::ANTIQUE_WHITE, 
            thickness: [2.0; 4]
        }
    }
}

impl StatBarBorder {
    pub fn new(color: Color, thickness: f32) -> Self {
        Self {
            color,
            thickness: [thickness; 4]
        }
    }
}

#[derive(Clone, Component)]
pub struct StatBar {
    /// Length of the full part of the bar.\
    /// empty = 0.0, full = 1.0
    pub value: f32,
    /// colors and border etc
    pub style: StatBarStyle,
    /// displacement from sprite
    pub displacement: Vec2,
    /// size of stat bar, doesn't include border
    pub size: Vec2,
    /// rotate stat bar CCW by `rotation` radians 
    pub rotation: f32,
}

impl Default for StatBar {
    fn default() -> Self {
        Self { 
            value: 0.5, 
            style: Default::default(),
            displacement: Vec2::ZERO, 
            size: vec2(64., 8.),
            rotation: 0.0
        }
    }
}

/// Collection of stat bars.
/// Bevy entities can't have two components of the same type.
/// To support more than one stat bar on an entity, 
/// we store them in a vec inside a component
#[derive(Clone, Component, Default)]
pub struct StatBars {
    pub bars: Vec<StatBar>,
    /// Displacement applied to all StatBars in the collection
    pub displacement: Vec2,
    /// all StatBars in collection rotated CCW by `rotation` radians
    pub rotation: f32,
}

impl StatBars {
    pub fn iter(&self) -> impl Iterator<Item=&StatBar> {
        self.bars.iter()
    }
}

impl std::ops::Index<usize> for StatBars {
    type Output=StatBar;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bars[index]
    }
}

impl std::ops::IndexMut<usize> for StatBars {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bars[index]
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum StatBarSystem {
    ExtractStatBars,
}

pub struct StatBarsPlugin;

impl Plugin for StatBarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StatBarZDepth>();
        if let Ok(render_app) = app.get_sub_app_mut(bevy::render::RenderApp) {
            render_app
            .add_system_to_stage(
                bevy::render::RenderStage::Extract,
                extract_sprites::extract_stat_bars_to_sprites
                .label(StatBarSystem::ExtractStatBars)
                .after(bevy::sprite::SpriteSystem::ExtractSprites)
            );
        }
    }
}