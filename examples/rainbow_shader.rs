//! # Shader Rainbow Demo
//!
//! True per-pixel smooth rainbow gradient using a custom `Material2d` shader.
//! Each glyph gets a seamless gradient based on its world-space X position.
//!
//! Font: Source Han Sans CN Light (思源黑体 CN Light)
//! Copyright 2014-2021 Adobe — SIL Open Font License 1.1

use bevy::asset::uuid_handle;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d};
use bevy_bitmap_text::*;

const FONT_NAME: &str = "SourceHanSansCN-Light";
const SIZE: u32 = 48;

const RAINBOW_SHADER_HANDLE: Handle<Shader> = uuid_handle!("f00ba4ca-feba-be12-3456-780000000001");

const RAINBOW_WGSL: &str = r#"
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct RainbowParams {
    time: f32,
    speed: f32,
    period: f32,
    lightness: f32,
}

@group(2) @binding(0) var glyph_texture: texture_2d<f32>;
@group(2) @binding(1) var glyph_sampler: sampler;
@group(2) @binding(2) var<uniform> params: RainbowParams;

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    let c = (1.0 - abs(2.0 * l - 1.0)) * s;
    let h6 = h * 6.0;
    let x = c * (1.0 - abs(h6 % 2.0 - 1.0));
    var rgb: vec3<f32>;
    if h6 < 1.0 { rgb = vec3(c, x, 0.0); }
    else if h6 < 2.0 { rgb = vec3(x, c, 0.0); }
    else if h6 < 3.0 { rgb = vec3(0.0, c, x); }
    else if h6 < 4.0 { rgb = vec3(0.0, x, c); }
    else if h6 < 5.0 { rgb = vec3(x, 0.0, c); }
    else { rgb = vec3(c, 0.0, x); }
    let m = l - c * 0.5;
    return rgb + m;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(glyph_texture, glyph_sampler, in.uv);
    if tex.a < 0.01 { discard; }

    let hue = fract(in.world_position.x / params.period + params.time * params.speed);
    let rgb = hsl_to_rgb(hue, 1.0, params.lightness);

    return vec4(rgb * tex.a, tex.a);
}
"#;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct RainbowGlyphMaterial {
    #[texture(0)]
    #[sampler(1)]
    atlas_texture: Handle<Image>,
    #[uniform(2)]
    params: RainbowParams,
}

#[derive(Clone, Copy, ShaderType)]
struct RainbowParams {
    time: f32,
    speed: f32,
    period: f32,
    lightness: f32,
}

impl Material2d for RainbowGlyphMaterial {
    fn fragment_shader() -> ShaderRef {
        RAINBOW_SHADER_HANDLE.into()
    }
}

/// Marker for the shader-rainbow text block.
#[derive(Component)]
struct ShaderRainbow;

/// Marks a glyph that has been converted from Sprite to Mesh2d.
#[derive(Component)]
struct ConvertedToMesh;

const WAVE_AMPLITUDE: f32 = 8.0;
const WAVE_FREQUENCY: f32 = 4.0;
const WAVE_PHASE_STEP: f32 = 0.5;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "bevy_bitmap_text — shader rainbow".into(),
            resolution: (960, 640).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(BitmapTextPlugin::default())
    .add_plugins(Material2dPlugin::<RainbowGlyphMaterial>::default());

    // Register the inline shader.
    {
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
        let _ = shaders.insert(
            RAINBOW_SHADER_HANDLE.id(),
            Shader::from_wgsl(RAINBOW_WGSL, "rainbow_glyph.wgsl"),
        );
    }

    app.add_systems(Startup, setup)
        .add_systems(PostUpdate, convert_glyphs_to_mesh.after(BitmapTextSet))
        .add_systems(Update, (update_rainbow_time, wave_animation_system))
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

    commands.spawn((
        ShaderRainbow,
        TextBlock::new("Shader Rainbow 着色器彩虹渐变!"),
        TextBlockStyling {
            font: FontId::from_name(FONT_NAME),
            size_px: SIZE,
            world_scale: SIZE as f32,
            align: TextAlign::Left,
            anchor: TextAnchor::CENTER,
            line_height: 1.4,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

/// Replace `Sprite` with `Mesh2d` + `MeshMaterial2d<RainbowGlyphMaterial>`
/// for each glyph under a `ShaderRainbow` text block.
fn convert_glyphs_to_mesh(
    mut commands: Commands,
    cache: Res<DynamicGlyphCache>,
    images: Res<Assets<Image>>,
    rainbow_query: Query<&Children, With<ShaderRainbow>>,
    glyph_query: Query<(Entity, &Sprite), (With<GlyphEntity>, Without<ConvertedToMesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RainbowGlyphMaterial>>,
) {
    let atlas_handle = &cache.atlas_image;
    let Some(atlas_img) = images.get(atlas_handle) else {
        return;
    };
    let atlas_size = Vec2::new(atlas_img.width() as f32, atlas_img.height() as f32);

    for children in rainbow_query.iter() {
        for child in children.iter() {
            let Ok((entity, sprite)) = glyph_query.get(child) else {
                continue;
            };

            let Some(custom_size) = sprite.custom_size else {
                continue;
            };
            let Some(pixel_rect) = sprite.rect else {
                continue;
            };

            let mesh_handle = build_glyph_mesh(&mut meshes, custom_size, pixel_rect, atlas_size);
            let material_handle = materials.add(RainbowGlyphMaterial {
                atlas_texture: atlas_handle.clone(),
                params: RainbowParams {
                    time: 0.0,
                    speed: 0.2,
                    period: 500.0,
                    lightness: 0.6,
                },
            });

            commands.entity(entity).remove::<Sprite>().insert((
                Mesh2d(mesh_handle),
                MeshMaterial2d(material_handle),
                ConvertedToMesh,
            ));
        }
    }
}

fn build_glyph_mesh(
    meshes: &mut Assets<Mesh>,
    size: Vec2,
    pixel_rect: Rect,
    atlas_size: Vec2,
) -> Handle<Mesh> {
    let uv_min = pixel_rect.min / atlas_size;
    let uv_max = pixel_rect.max / atlas_size;

    // Rectangle vertex order: top-right, top-left, bottom-left, bottom-right.
    let mut mesh = Rectangle::new(size.x, size.y).mesh().build();
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [uv_max.x, uv_min.y], // top-right
            [uv_min.x, uv_min.y], // top-left
            [uv_min.x, uv_max.y], // bottom-left
            [uv_max.x, uv_max.y], // bottom-right
        ],
    );
    meshes.add(mesh)
}

fn update_rainbow_time(time: Res<Time>, mut materials: ResMut<Assets<RainbowGlyphMaterial>>) {
    for (_, material) in materials.iter_mut() {
        material.params.time = time.elapsed_secs();
    }
}

fn wave_animation_system(
    time: Res<Time>,
    rainbow_query: Query<&Children, With<ShaderRainbow>>,
    mut glyph_query: Query<(&GlyphEntity, &GlyphBaseOffset, &mut Transform)>,
) {
    let t = time.elapsed_secs();
    for children in rainbow_query.iter() {
        for child in children.iter() {
            let Ok((glyph, base, mut transform)) = glyph_query.get_mut(child) else {
                continue;
            };
            let phase = glyph.char_index as f32 * WAVE_PHASE_STEP;
            let offset = (t * WAVE_FREQUENCY + phase).sin() * WAVE_AMPLITUDE;
            transform.translation.x = base.0.x;
            transform.translation.y = base.0.y + offset;
        }
    }
}
