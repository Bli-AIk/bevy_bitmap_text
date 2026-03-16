//! # Color Text Demo
//!
//! Demonstrates per-segment text coloring: red, purple, and scrolling rainbow gradient.
//!
//! Font: Source Han Sans CN Light (思源黑体 CN Light)
//! Copyright 2014-2021 Adobe — SIL Open Font License 1.1

use bevy::color::Srgba;
use bevy::prelude::*;
use bevy_bitmap_text::*;

const FONT_NAME: &str = "SourceHanSansCN-Light";
const SIZE: u32 = 40;
const RAINBOW_SPEED: f32 = 0.3;

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
        .add_systems(Update, scroll_rainbow_system)
        .run();
}

#[derive(Component)]
struct RainbowText;

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

    // --- Scrolling rainbow text (per-character, animated) ---
    commands.spawn((
        RainbowText,
        TextBlock::new("Rainbow 彩虹渐变文字!"),
        base_styling,
        Transform::from_xyz(0.0, -50.0, 0.0),
    ));
}

fn scroll_rainbow_system(
    time: Res<Time>,
    rainbow_query: Query<(&TextBlockLayout, &TextBlockStyling, &Children), With<RainbowText>>,
    mut glyph_query: Query<(&GlyphBaseOffset, &mut Sprite), With<GlyphEntity>>,
) {
    for (layout, styling, children) in rainbow_query.iter() {
        let scale = styling.world_scale / styling.size_px as f32;
        let block_width = layout.dimension.x * scale;
        if block_width <= 0.0 {
            continue;
        }

        for child in children.iter() {
            let Ok((base, mut sprite)) = glyph_query.get_mut(child) else {
                continue;
            };
            let normalized_x = (base.0.x / block_width).rem_euclid(1.0);
            let hue = (normalized_x + time.elapsed_secs() * RAINBOW_SPEED).rem_euclid(1.0) * 360.0;
            sprite.color = Srgba::from(Hsla::new(hue, 1.0, 0.6, 1.0)).into();
        }
    }
}
