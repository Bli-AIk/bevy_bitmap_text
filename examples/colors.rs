//! # Color Text Demo
//!
//! Demonstrates per-segment text coloring: red, purple, and rainbow gradient.
//!
//! Font: Source Han Sans CN Light (思源黑体 CN Light)
//! Copyright 2014-2021 Adobe — SIL Open Font License 1.1

use bevy::color::Srgba;
use bevy::prelude::*;
use bevy_bitmap_text::*;

const FONT_NAME: &str = "SourceHanSansCN-Light";
const SIZE: u32 = 40;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_bitmap_text — color demo".into(),
                resolution: (960, 640).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BitmapTextPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut cache: ResMut<DynamicGlyphCache>) {
    commands.spawn(Camera2d);

    let font_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples/assets/SourceHanSansCN-Light.otf");
    let font_data = std::fs::read(&font_path).expect("Failed to read font file");
    cache
        .add_font(FontId::from_name(FONT_NAME), &font_data)
        .expect("Failed to load font");

    let base_styling = TextBlockStyling {
        font: FontId::from_name(FONT_NAME),
        size_px: SIZE,
        world_scale: SIZE as f32,
        align: TextAlign::Left,
        anchor: TextAnchor::CENTER,
        line_height: 1.4,
        ..default()
    };

    // --- Red text ---
    commands.spawn((
        TextBlock::new("This text is RED 红色文字"),
        TextBlockStyling {
            color: Srgba::new(1.0, 0.2, 0.2, 1.0),
            ..base_styling.clone()
        },
        Transform::from_xyz(0.0, 150.0, 0.0),
    ));

    // --- Purple text ---
    commands.spawn((
        TextBlock::new("This text is PURPLE 紫色文字"),
        TextBlockStyling {
            color: Srgba::new(0.7, 0.2, 1.0, 1.0),
            ..base_styling.clone()
        },
        Transform::from_xyz(0.0, 50.0, 0.0),
    ));

    // --- Rainbow gradient text (per-character segment) ---
    let rainbow_text = "Rainbow 彩虹渐变文字!";
    let segments = build_rainbow_segments(rainbow_text);
    commands.spawn((
        TextBlock::from_segments(segments),
        base_styling,
        Transform::from_xyz(0.0, -50.0, 0.0),
    ));
}

fn build_rainbow_segments(text: &str) -> Vec<TextSegment> {
    let chars: Vec<char> = text.chars().collect();
    let count = chars.len().max(1) as f32;

    chars
        .into_iter()
        .enumerate()
        .map(|(i, ch)| {
            let hue = (i as f32 / count) * 360.0;
            let color = Hsla::new(hue, 1.0, 0.6, 1.0);

            TextSegment {
                text: ch.to_string(),
                style: SegmentStyle {
                    color: Some(Srgba::from(color)),
                },
            }
        })
        .collect()
}
