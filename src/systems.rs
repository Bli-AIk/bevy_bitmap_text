//! # systems.rs
//!
//! # systems.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! Contains the ECS systems that keep bitmap text entities synchronized with their data.
//! It rasterizes required glyphs, recomputes layout when text changes, spawns and updates glyph
//! child entities, and applies reveal state to the final rendered characters.
//!
//! 包含让位图文本实体与其数据保持同步的 ECS 系统。它会栅格化所需字形，在文本变化时
//! 重新计算布局，生成并更新逐字形子实体，并把 reveal 状态应用到最终渲染出的字符上。

use bevy::prelude::*;

use crate::cache::{DynamicGlyphCache, GlyphKey};
use crate::components::*;
use crate::layout;

/// System set for bitmap text systems (runs in PostUpdate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct BitmapTextSet;

/// Rasterize any new glyphs needed by changed text blocks.
pub fn rasterize_glyphs_system(
    mut cache: ResMut<DynamicGlyphCache>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(&TextBlock, &TextBlockStyling), Changed<TextBlock>>,
) {
    for (block, styling) in query.iter() {
        rasterize_block_glyphs(block, styling, &mut cache, &mut images);
    }
}

fn rasterize_block_glyphs(
    block: &TextBlock,
    styling: &TextBlockStyling,
    cache: &mut DynamicGlyphCache,
    images: &mut Assets<Image>,
) {
    for segment in &block.segments {
        for ch in segment.text.chars() {
            if ch == '\n' || ch == '\r' {
                continue;
            }
            let key = GlyphKey {
                font_id: styling.font.clone(),
                character: ch,
                size_px: styling.size_px,
            };
            cache.get_or_insert(&key, images);
        }
    }
}

/// Compute layout for changed text blocks.
pub fn layout_text_system(
    cache: Res<DynamicGlyphCache>,
    font_overrides: Option<Res<FontLayoutOverrides>>,
    mut query: Query<
        (&TextBlock, &TextBlockStyling, &mut TextBlockLayout),
        Or<(Changed<TextBlock>, Changed<TextBlockStyling>)>,
    >,
) {
    for (block, styling, mut text_layout) in query.iter_mut() {
        let font_override = font_overrides
            .as_deref()
            .and_then(|overrides| overrides.get(&styling.font));
        *text_layout =
            layout::compute_layout_with_font_override(block, styling, &cache, font_override);
    }
}

/// Synchronize glyph child entities to match the computed layout.
pub fn sync_glyph_entities_system(
    mut commands: Commands,
    cache: Res<DynamicGlyphCache>,
    mut query: Query<
        (
            Entity,
            &TextBlockStyling,
            &mut TextBlockLayout,
            Option<&GlyphReveal>,
        ),
        Changed<TextBlockLayout>,
    >,
    existing_glyphs: Query<Entity, With<GlyphEntity>>,
    children_query: Query<&Children>,
) {
    for (text_entity, styling, mut text_layout, reveal) in query.iter_mut() {
        // Despawn all existing glyph children.
        if let Ok(children) = children_query.get(text_entity) {
            for child in children.iter().filter(|c| existing_glyphs.get(*c).is_ok()) {
                commands.entity(child).despawn();
            }
        }

        let glyph_entities = spawn_layout_glyphs(
            &mut commands,
            text_entity,
            styling,
            &text_layout,
            reveal,
            &cache,
        );

        for (glyph, entity) in text_layout.glyphs.iter_mut().zip(glyph_entities.iter()) {
            glyph.entity = *entity;
        }
    }
}

fn spawn_layout_glyphs(
    commands: &mut Commands,
    text_entity: Entity,
    styling: &TextBlockStyling,
    text_layout: &TextBlockLayout,
    reveal: Option<&GlyphReveal>,
    cache: &DynamicGlyphCache,
) -> Vec<Option<Entity>> {
    let scale_factor = styling.world_scale / styling.size_px as f32;
    let mut glyph_entities = Vec::with_capacity(text_layout.glyphs.len());

    for glyph in text_layout.glyphs.iter() {
        let entity = spawn_single_glyph(
            commands,
            text_entity,
            glyph,
            styling,
            reveal,
            cache,
            scale_factor,
        );
        glyph_entities.push(entity);
    }

    glyph_entities
}

fn spawn_single_glyph(
    commands: &mut Commands,
    text_entity: Entity,
    glyph: &LayoutGlyph,
    styling: &TextBlockStyling,
    reveal: Option<&GlyphReveal>,
    cache: &DynamicGlyphCache,
    scale_factor: f32,
) -> Option<Entity> {
    if glyph.character == ' ' || glyph.size == Vec2::ZERO {
        return None;
    }

    let key = GlyphKey {
        font_id: styling.font.clone(),
        character: glyph.character,
        size_px: styling.size_px,
    };

    let uv_rect = cache
        .glyph_map_get(&key)
        .map(|info| info.pixel_rect)
        .unwrap_or_default();

    if uv_rect.width() == 0.0 || uv_rect.height() == 0.0 {
        return None;
    }

    let pos = glyph.position * scale_factor;
    let sprite_size = glyph.size * scale_factor;

    // Layout gives top-left corner (Y-up); Sprite renders from center.
    let center = Vec2::new(pos.x + sprite_size.x / 2.0, pos.y - sprite_size.y / 2.0);

    let vis = match reveal {
        Some(r) if glyph.char_index >= r.visible_count => Visibility::Hidden,
        _ => Visibility::Inherited,
    };

    // Small Z offset to ensure glyphs render in front of parent/sibling SDF shapes.
    let glyph_z = 0.001;

    let glyph_entity = commands
        .spawn((
            GlyphEntity {
                char_index: glyph.char_index,
                character: glyph.character,
            },
            GlyphBaseOffset(center),
            Sprite {
                image: cache.atlas_image.clone(),
                custom_size: Some(sprite_size),
                rect: Some(uv_rect),
                color: glyph.color.into(),
                ..Default::default()
            },
            Transform::from_translation(center.extend(glyph_z)),
            vis,
        ))
        .id();

    commands.entity(text_entity).add_child(glyph_entity);
    Some(glyph_entity)
}

/// Update glyph visibility when `GlyphReveal` changes.
pub fn glyph_reveal_system(
    reveal_query: Query<(&GlyphReveal, &Children), Changed<GlyphReveal>>,
    mut glyph_query: Query<(&GlyphEntity, &mut Visibility)>,
) {
    for (reveal, children) in reveal_query.iter() {
        apply_reveal_to_children(reveal, children, &mut glyph_query);
    }
}

fn apply_reveal_to_children(
    reveal: &GlyphReveal,
    children: &Children,
    glyph_query: &mut Query<(&GlyphEntity, &mut Visibility)>,
) {
    for child in children.iter() {
        let Ok((glyph, mut vis)) = glyph_query.get_mut(child) else {
            continue;
        };
        *vis = if glyph.char_index < reveal.visible_count {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

/// Animate glyphs with `WaveEffect` using a sine-wave vertical offset.
pub fn text_wave_system(
    time: Res<Time>,
    mut query: Query<(
        &GlyphEntity,
        &mut WaveEffect,
        &GlyphBaseOffset,
        &mut Transform,
    )>,
) {
    for (glyph, mut wave, base, mut transform) in query.iter_mut() {
        wave.elapsed += time.delta_secs();
        let phase = glyph.char_index as f32 * 0.5;
        let offset = (wave.elapsed * wave.frequency + phase).sin() * wave.amplitude;
        transform.translation.x = base.0.x;
        transform.translation.y = base.0.y + offset;
    }
}
