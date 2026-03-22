//! # font_id.rs
//!
//! # font_id.rs 文件
//!
//! ## Module Overview
//!
//! ## 模块概述
//!
//! This file defines the lightweight font identifier type shared across the bitmap text crate. It
//! exists so systems and assets can refer to registered fonts by stable names instead of passing
//! raw strings around everywhere.
//!
//! 这个文件定义了位图文本 crate 内部共享的轻量级字体标识类型。它的作用是让系统和资产可以用
//! 稳定名称引用已注册字体，而不是在各处直接传裸字符串。

use bevy::prelude::*;

/// Identifies a loaded font by name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub struct FontId(pub String);

impl FontId {
    pub fn from_name(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl<S: Into<String>> From<S> for FontId {
    fn from(s: S) -> Self {
        Self(s.into())
    }
}
