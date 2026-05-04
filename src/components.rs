//! # components.rs
//!
//! # components.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Defines the ECS data model for bitmap text rendering. It covers authored text blocks,
//! per-glyph layout records, styling and anchor choices, and the small effect components that are
//! later interpreted by the rendering systems.
//!
//! 定义了位图文本渲染所依赖的 ECS 数据模型。它包括作者侧的文本块、逐字形的排版记录、
//! 样式与锚点选择，以及后续渲染系统会解释的小型效果组件。

use std::collections::HashMap;

use bevy::color::Srgba;
use bevy::math::{Rect, Vec2};
use bevy::prelude::*;

use crate::font_id::FontId;

/// A text segment with optional per-segment styling.
#[derive(Debug, Clone, Reflect)]
pub struct TextSegment {
    pub text: String,
    pub style: SegmentStyle,
}

/// Per-segment styling override.
#[derive(Debug, Clone, Default, Reflect)]
pub struct SegmentStyle {
    pub color: Option<Srgba>,
}

/// A block of text to be rendered as individual glyph sprite entities.
///
/// When this component changes, the layout and glyph entity systems
/// automatically recalculate positions and synchronize child entities.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
#[require(TextBlockStyling, TextBlockLayout)]
pub struct TextBlock {
    pub segments: Vec<TextSegment>,
}

impl TextBlock {
    /// Create a text block with a single unstyled segment.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            segments: vec![TextSegment {
                text: text.into(),
                style: SegmentStyle::default(),
            }],
        }
    }

    /// Create a text block from pre-parsed segments.
    pub fn from_segments(segments: Vec<TextSegment>) -> Self {
        Self { segments }
    }

    /// Get the full text content (all segments concatenated).
    pub fn full_text(&self) -> String {
        self.segments.iter().map(|s| s.text.as_str()).collect()
    }

    /// Get single-segment text content, if there is exactly one segment.
    pub fn get_single(&self) -> Option<&str> {
        if self.segments.len() == 1 {
            Some(&self.segments[0].text)
        } else {
            None
        }
    }
}

/// Horizontal text alignment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TextAlign {
    /// Returns the alignment factor (0.0 = left, 0.5 = center, 1.0 = right).
    pub fn factor(&self) -> f32 {
        match self {
            TextAlign::Left => 0.0,
            TextAlign::Center => 0.5,
            TextAlign::Right => 1.0,
        }
    }
}

/// Anchor point for the text block bounding box.
///
/// Values typically range from `(-0.5, -0.5)` (top-left) to `(0.5, 0.5)` (bottom-right).
/// Default is `(0.5, -0.5)` (bottom-right), used by dialogue text.
/// where text grows downward from the top-left.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct TextAnchor(pub Vec2);

impl Default for TextAnchor {
    fn default() -> Self {
        // Bottom-right anchor, matching bevy_rich_text3d convention
        Self(Vec2::new(0.5, -0.5))
    }
}

impl TextAnchor {
    pub const TOP_LEFT: Self = Self(Vec2::new(-0.5, 0.5));
    pub const TOP_CENTER: Self = Self(Vec2::new(0.0, 0.5));
    pub const TOP_RIGHT: Self = Self(Vec2::new(0.5, 0.5));
    pub const CENTER_LEFT: Self = Self(Vec2::new(-0.5, 0.0));
    pub const CENTER: Self = Self(Vec2::new(0.0, 0.0));
    pub const CENTER_RIGHT: Self = Self(Vec2::new(0.5, 0.0));
    pub const BOTTOM_LEFT: Self = Self(Vec2::new(-0.5, -0.5));
    pub const BOTTOM_CENTER: Self = Self(Vec2::new(0.0, -0.5));
    pub const BOTTOM_RIGHT: Self = Self(Vec2::new(0.5, -0.5));
}

/// Styling configuration for a text block.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TextBlockStyling {
    /// Font identifier (must be registered in `DynamicGlyphCache`).
    pub font: FontId,
    /// Rasterization size in pixels (e.g. 32).
    pub size_px: u32,
    /// World-space scale applied to glyph sprites.
    pub world_scale: f32,
    /// Default text color.
    pub color: Srgba,
    /// Horizontal alignment.
    pub align: TextAlign,
    /// Anchor point for the text bounding box.
    pub anchor: TextAnchor,
    /// Line height multiplier (1.0 = tight, 1.375 = spaced).
    pub line_height: f32,
    /// Extra horizontal spacing between characters, in pixels.
    pub char_spacing: f32,
    /// Extra horizontal spacing after word-separator characters (space, etc.), in pixels.
    pub word_spacing: f32,
    /// Maximum width for auto line-wrapping (None = no wrapping).
    pub max_width: Option<f32>,
}

impl Default for TextBlockStyling {
    fn default() -> Self {
        Self {
            font: FontId::from_name("default"),
            size_px: 32,
            world_scale: 1.0,
            color: Srgba::WHITE,
            align: TextAlign::default(),
            anchor: TextAnchor::default(),
            line_height: 1.375,
            char_spacing: 0.0,
            word_spacing: 0.0,
            max_width: None,
        }
    }
}

/// Per-font layout override applied before glyphs are spawned.
///
/// 生成字形实体前应用的字体级排版覆盖。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontLayoutOverride {
    /// Glyph offset as a fraction of `TextBlockStyling::world_scale`.
    ///
    /// 字形偏移量，单位为 `TextBlockStyling::world_scale` 的比例。
    pub offset_factor: Vec2,
}

impl Default for FontLayoutOverride {
    fn default() -> Self {
        Self {
            offset_factor: Vec2::ZERO,
        }
    }
}

/// Global per-font layout override table.
///
/// 全局字体级排版覆盖表。
#[derive(Resource, Debug, Clone, Default)]
pub struct FontLayoutOverrides {
    offsets: HashMap<FontId, FontLayoutOverride>,
}

impl FontLayoutOverrides {
    /// Insert or replace the override for a font.
    ///
    /// 插入或替换某个字体的排版覆盖。
    pub fn insert(&mut self, font: FontId, layout: FontLayoutOverride) {
        self.offsets.insert(font, layout);
    }

    /// Return the override for a font, if configured.
    ///
    /// 返回某个字体的排版覆盖；未配置时返回空。
    pub fn get(&self, font: &FontId) -> Option<&FontLayoutOverride> {
        self.offsets.get(font)
    }
}

/// Computed layout for a text block (populated by the layout system).
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct TextBlockLayout {
    /// Per-glyph layout information.
    pub glyphs: Vec<LayoutGlyph>,
    /// Total dimension of the text block (in world units BEFORE world_scale).
    pub dimension: Vec2,
}

/// Layout information for a single glyph.
#[derive(Debug, Clone, Reflect)]
pub struct LayoutGlyph {
    /// Index in the original text string (char index, not byte index).
    pub char_index: usize,
    /// The character.
    pub character: char,
    /// Position relative to the text block origin (in pixel space, before world_scale).
    pub position: Vec2,
    /// Rendering size of the glyph (in pixels).
    pub size: Vec2,
    /// UV rectangle in the atlas (in pixels).
    pub uv_rect: Rect,
    /// Color for this glyph.
    pub color: Srgba,
    /// Entity ID of the spawned glyph (populated by sync system).
    pub entity: Option<Entity>,
}

/// Marker component for a glyph child entity.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GlyphEntity {
    /// Character index in the parent TextBlock.
    pub char_index: usize,
    /// The rendered character.
    pub character: char,
}

/// Stores the base layout offset for a glyph entity.
/// Animation systems can modify `Transform` while this records the "home" position.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct GlyphBaseOffset(pub Vec2);

/// Controls progressive glyph reveal (typewriter effect).
///
/// When present on a `TextBlock` entity, only child glyphs with
/// `char_index < visible_count` are visible. Glyphs at or beyond
/// `visible_count` have `Visibility::Hidden`.
///
/// Absent by default — all glyphs are visible.
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct GlyphReveal {
    pub visible_count: usize,
}

/// Per-glyph shake effect — random jitter on each axis per frame.
///
/// While present on a `GlyphEntity`, the glyph's transform receives a
/// uniformly-distributed pseudo-random offset each frame.
///
/// 逐字形抖动效果 — 每帧每个轴的随机抖动。
///
/// 当存在于 `GlyphEntity` 上时，字形每帧获得均匀分布的伪随机偏移。
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ShakeEffect {
    /// Maximum pixel offset in each axis.
    pub intensity: f32,
}

/// Per-glyph twitch effect — occasional fixed offset.
///
/// While present on a `GlyphEntity`, the glyph's transform receives this fixed
/// offset relative to its base layout position.
///
/// 逐字形闪动效果 — 偶发固定偏移。
///
/// 当存在于 `GlyphEntity` 上时，字形相对其基础布局位置获得这个固定偏移。
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TwitchEffect {
    /// Pixel offset applied to the glyph.
    pub offset: Vec2,
}

/// Per-glyph wave effect.
///
/// While present on a `GlyphEntity`, the glyph oscillates vertically
/// in a sine-wave pattern seeded by its `char_index`.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct WaveEffect {
    /// Peak amplitude in pixels.
    pub amplitude: f32,
    /// Oscillation speed (radians per second).
    pub frequency: f32,
    /// Accumulated time (advanced by the wave system).
    pub elapsed: f32,
}
