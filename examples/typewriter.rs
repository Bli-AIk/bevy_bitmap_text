//! # Typewriter Animation Demo
//!
//! Characters appear one by one with fade-in + rise-up animation.
//! - Fade: alpha 0 → 1 over 0.5s
//! - Rise: offset Y -20px → 0 over 0.5s
//! - Delay between characters: 0.2s
//! - Full cycle: 5s (auto-reset)
//!
//! Font: Source Han Sans CN Light (思源黑体 CN Light)
//! Copyright 2014-2021 Adobe — SIL Open Font License 1.1

use bevy::prelude::*;
use bevy_bitmap_text::*;

const FONT_NAME: &str = "SourceHanSansCN-Light";
const SIZE: u32 = 36;
const CHAR_DELAY: f32 = 0.2;
const ANIM_DURATION: f32 = 0.5;
const CYCLE_PERIOD: f32 = 5.0;
const RISE_DISTANCE: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_bitmap_text — typewriter animation".into(),
                resolution: (960, 640).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BitmapTextPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, typewriter_animation_system)
        .run();
}

/// Tracks the typewriter animation state.
#[derive(Component)]
struct TypewriterAnim {
    elapsed: f32,
}

fn setup(mut commands: Commands, mut cache: ResMut<DynamicGlyphCache>) {
    commands.spawn(Camera2d);

    let font_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/assets/SourceHanSansCN-Light.otf");
    let font_data = std::fs::read(&font_path).expect("Failed to read font file");
    cache
        .add_font(FontId::from_name(FONT_NAME), &font_data)
        .expect("Failed to load font");

    commands.spawn((
        TypewriterAnim { elapsed: 0.0 },
        TextBlock::new("Hello! 你好世界！逐字出现动画"),
        TextBlockStyling {
            font: FontId::from_name(FONT_NAME),
            size_px: SIZE,
            world_scale: SIZE as f32,
            color: bevy::color::Srgba::WHITE,
            align: TextAlign::Left,
            anchor: TextAnchor::CENTER,
            line_height: 1.4,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn typewriter_animation_system(
    time: Res<Time>,
    mut anim_query: Query<(&mut TypewriterAnim, &Children)>,
    mut glyph_query: Query<(
        &GlyphEntity,
        &GlyphBaseOffset,
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
    )>,
) {
    for (mut anim, children) in anim_query.iter_mut() {
        anim.elapsed += time.delta_secs();

        if anim.elapsed >= CYCLE_PERIOD {
            anim.elapsed -= CYCLE_PERIOD;
        }

        let elapsed = anim.elapsed;

        for child in children.iter() {
            let Ok((glyph, base, mut transform, mut sprite, mut vis)) =
                glyph_query.get_mut(child)
            else {
                continue;
            };

            let char_start = glyph.char_index as f32 * CHAR_DELAY;
            let progress = ((elapsed - char_start) / ANIM_DURATION).clamp(0.0, 1.0);

            if elapsed < char_start {
                *vis = Visibility::Hidden;
                continue;
            }

            *vis = Visibility::Inherited;

            let ease = ease_out_cubic(progress);

            sprite.color = sprite.color.with_alpha(ease);

            let y_offset = (1.0 - ease) * -RISE_DISTANCE;
            transform.translation.x = base.0.x;
            transform.translation.y = base.0.y + y_offset;
        }
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
