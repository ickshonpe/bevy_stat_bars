pub mod extract_sprites;

use bevy::prelude::*;
use bevy::reflect::FromReflect;

/// using the sprite renderer to draw the stat bars
/// all stat bars share the same depth
pub struct StatBarZDepth(pub f32);

impl Default for StatBarZDepth {
    fn default() -> Self {
        Self(999.0)
    }
}

#[derive(Copy, Clone, Reflect, Component, FromReflect)]
#[reflect(Component)]
pub enum BarColor {
    Fixed(Color),
    Lerp { min: Color, max: Color },
    Cospolate { min: Color, max: Color },
    LerpHSV { min: Color, max: Color },
    CospolateHSV { min: Color, max: Color },
    Function{ min: Color, max: Color, calculate_color: fn(Color, Color, f32) -> Color }, // + 'static + Send + Sync),
}

impl Default for BarColor {
    fn default() -> Self {
        Self::Fixed(Color::ORANGE)
    }
}

impl std::fmt::Debug for BarColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(arg0) => f.debug_tuple("Fixed").field(arg0).finish(),
            Self::Lerp { min, max } => f.debug_struct("Lerp").field("min", min).field("max", max).finish(),
            Self::Cospolate { min, max } => f.debug_struct("Cospolate").field("min", min).field("max", max).finish(),
            Self::LerpHSV { min, max } => f.debug_struct("LerpHSV").field("min", min).field("max", max).finish(),
            Self::CospolateHSV { min, max } => f.debug_struct("CospolateHSV").field("min", min).field("max", max).finish(),
            Self::Function { min, max, calculate_color } => 
                f.debug_struct("Function").field("min", min).field("max", max).field("calculate_color", calculate_color).finish(),
        }
    }
}


impl BarColor {
    pub fn set_min(&mut self, color: Color) {
        match self {
            BarColor::Lerp { min, .. } => *min = color,
            BarColor::Cospolate { min, .. } => *min = color,
            BarColor::LerpHSV { min, .. } => *min = color,
            BarColor::CospolateHSV { min, .. } => *min = color,
            BarColor::Function { min, .. } => *min = color,
            _ => {}
        }
    }

    pub fn set_max(&mut self, color: Color) {
        match self {
            BarColor::Lerp { max, .. } => *max = color,
            BarColor::Cospolate { max, .. } => *max = color,
            BarColor::LerpHSV { max, .. } => *max = color,
            BarColor::CospolateHSV { max, .. } => *max = color,
            BarColor::Function { max, .. } => *max = color, 
            _ => {}
        }
    }
}

impl From<Color> for BarColor {
    fn from(color: Color) -> Self {
        BarColor::Fixed(color)
    }
}



#[derive(Clone, Component, Debug, Reflect, FromReflect)]
#[reflect(Component)]
pub struct StatBarStyle {
    /// color of the full part of the bar
    pub bar_color: BarColor,
    /// color of the empty part of the bar
    pub empty_color: Color,
    /// None = no border
    pub border: Option<StatBarBorder>
}

impl Default for StatBarStyle {
    fn default() -> Self {
        Self { 
            bar_color: Color::ORANGE.into(), 
            empty_color: Color::rgb(0.2, 0.1, 0.0), 
            border: StatBarBorder::default().into(),
        }
    }
}

#[derive(Clone, Copy, Debug, Reflect, FromReflect)]
#[derive(Component)]
#[reflect(Component)]
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

#[derive(Clone, Component, Debug, Reflect, FromReflect)]
#[reflect(Component)]
pub struct StatBar {
    /// Length of the full part of the bar.\
    /// empty = 0.0, full = 1.0
    pub value: f32,
    pub length: f32,
    pub thickness: f32,
    /// colors and border etc
    pub style: StatBarStyle,
    /// displacement from sprite
    pub translation: Vec2,
    /// rotate stat bar CCW by `rotation` radians 
    pub rotation: f32,
}



impl Default for StatBar {
    fn default() -> Self {
        Self { 
            value: 0.5, 
            style: Default::default(),
            translation: Vec2::ZERO, 
            length: 64.,
            thickness: 8.,
            rotation: 0.0
        }
    }
}

/// Collection of stat bars.
/// Bevy entities can't have two components of the same type.
/// To support more than one stat bar on an entity, 
/// we store them in a vec inside a component
#[derive(Clone, Component, Default, Reflect, FromReflect)]
#[reflect(Component)]
pub struct StatBars {
    pub bars: Vec<StatBar>,
    /// Displacement applied to all StatBars in the collection
    pub translation: Vec2,
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

#[derive(Bundle)]
pub struct StatBarBundle {
    statbar: StatBar,
    #[bundle] 
    visibility_bundle: VisibilityBundle,
}

impl StatBarBundle {
    pub fn new(statbar: StatBar) -> Self {
        Self { statbar, visibility_bundle: VisibilityBundle::default() }
    }
}

#[derive(Bundle)]
pub struct StatBarsBundle {
    statbars: StatBars,
    #[bundle] 
    visibility_bundle: VisibilityBundle,
}


impl StatBarsBundle {
    pub fn new(statbars: StatBars) -> Self {
        Self { statbars, visibility_bundle: VisibilityBundle::default() }
    }
}
pub struct StatBarsPlugin;

impl Plugin for StatBarsPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StatBarZDepth>()
        .register_type::<StatBarStyle>()
        .register_type::<StatBars>()
        .register_type::<BarColor>()
        .register_type::<StatBarBorder>()
        ;
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