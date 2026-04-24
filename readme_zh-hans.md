# bevy_bitmap_text

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-APACHE)
<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

**bevy_bitmap_text** — 基于 Bevy 的 Glyph-as-Entity 动态图集文本渲染。

| English                | 简体中文 |
|------------------------|------|
| [English](./readme.md) | 简体中文 |

## 简介

在游戏引擎中，标准的文本渲染通常是一个不可分割的整体：整句话会被烘焙成一个单独的网格（Mesh）。但如果你只希望句子中某*一个*
特定的字母剧烈抖动、上下跳跃，或者动态地改变颜色呢？尤其是对于像素游戏而言，想要在高度抽象的现代文本系统中实现那种充满“复古感”的逐字表现力，往往会让人感到异常沮丧。

所以我想，要不干脆化繁为简，回到最淳朴的位图思路吧。于是做了 `bevy_bitmap_text` 这个
crate。它是一个面向 [Bevy](https://bevyengine.org/) 的文本渲染后端，**将每一个字符作为独立的 ECS 实体（Entity）生成**
，并附带一个标准的 `Sprite` 组件。

通过将字形视为独立的位图精灵（Sprite），它把控制权重新交回给了 Bevy 强大的 ECS 系统。想让一个单词像旗帜一样波动？只需给那些字符实体贴上一个
`WaveEffect` 组件即可。插件会在幕后自动为你处理繁重的脏活累活：实时字体光栅化、图集动态装箱、排版布局，以及保持所有实体状态同步。

就是这样——一个简单的文本替代方案摆在这里咯。喜欢的话就拿去用吧。

## 功能特性

* 🧩 **Glyph-as-Entity 架构** — 每个字符是独立的 `Sprite` 实体，通过标准 ECS 实现逐字动画和样式控制
* 🔠 **动态字形图集** — 使用 [fontdue](https://github.com/mooman219/fontdue)
  按需光栅化，通过 [etagere](https://crates.io/crates/etagere) 进行矩形装箱
* ⌨️ **打字机效果** — 内置 `GlyphReveal` 组件，实现逐字显示
* ✨ **逐字特效** — `ShakeEffect` 和 `WaveEffect` 组件，提供抖动和正弦波动画
* 🎨 **颜色标签** — 内联 `{#RRGGBB:文本}` 标记，实现分段着色
* 🖌️ **文本样式** — 可配置字体、大小、颜色、对齐、锚点、行高、字间距和词间距
* ↩️ **自动换行** — 可选的 `max_width` 实现自动换行

## 使用方法

1. **添加依赖** 到 `Cargo.toml`：

   ```toml
   [dependencies]
   bevy_bitmap_text = { path = "crates/bevy_bitmap_text" }
   ```

2. **注册插件**：

   ```rust
   use bevy::prelude::*;
   use bevy_bitmap_text::{BitmapTextPlugin, FontDirectories};

   fn main() {
       App::new()
           .add_plugins(DefaultPlugins)
           .insert_resource(FontDirectories {
               directories: vec!["assets/fonts".to_string()],
           })
           .add_plugins(BitmapTextPlugin::default())
           .add_systems(Startup, setup)
           .run();
   }
   ```

3. **生成文本块**：

   ```rust
   use bevy_bitmap_text::*;

   fn setup(mut commands: Commands) {
       commands.spawn(Camera2d);
       commands.spawn((
           TextBlock::new("你好，世界！"),
           TextBlockStyling {
               font: FontId::from_name("my_font"),
               size_px: 32,
               world_scale: 32.0,
               ..default()
           },
       ));
   }
   ```

4. **打字机效果**：

   ```rust
   commands.spawn((
       TextBlock::new("逐字显示的文本..."),
       TextBlockStyling { /* ... */ },
       GlyphReveal { visible_count: 0 },
   ));
   // 每帧递增 `visible_count` 以逐字显示。
   ```

5. **颜色标签**：

   ```rust
   use bevy_bitmap_text::parse_text_to_segments;
   let segments = parse_text_to_segments("{#FF0000:红色} 和 {#00FF00:绿色}");
   commands.spawn((
       TextBlock::from_segments(segments),
       TextBlockStyling { /* ... */ },
   ));
   ```

## 构建方法

### 前置要求

* Rust 1.85 或更高版本（edition 2024）
* Bevy 0.18

### 构建步骤

1. **构建 crate**：

   ```bash
   cargo build -p bevy_bitmap_text
   ```

2. **运行测试**：

   ```bash
   cargo test -p bevy_bitmap_text
   ```

3. **运行演示**：

   ```bash
   cargo run -p bevy_bitmap_text --example demo
   ```

## 依赖

本项目使用以下 crate：

| Crate                                       | 版本   | 说明                  |
|---------------------------------------------|------|---------------------|
| [bevy](https://crates.io/crates/bevy)       | 0.18 | 游戏引擎框架（资源、颜色、渲染、精灵） |
| [fontdue](https://crates.io/crates/fontdue) | 0.7  | 轻量级字体光栅化            |
| [etagere](https://crates.io/crates/etagere) | 0.2  | 矩形图集装箱              |
| [log](https://crates.io/crates/log)         | 0.4  | 日志门面                |

## 贡献

欢迎贡献！
无论是修复 bug、添加功能还是改进文档：

* 提交 **Issue** 或 **Pull Request**。
* 分享想法，讨论设计或架构。

## 许可证

本项目采用以下任一许可证：

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

由你选择。
