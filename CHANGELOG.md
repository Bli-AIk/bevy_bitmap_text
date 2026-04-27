# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/Bli-AIk/bevy_bitmap_text/compare/v0.1.1...v0.2.0) - 2026-04-27

### Added

- *(bevy_bitmap_text)* add Reflect trait to components for reflection support
- *(view_text_reconstruction)* add best score restoration and update HUD configurations

### Documentation

- *(bevy_bitmap_text)* rewrite readme and zh-hans readme

### Miscellaneous Tasks

- *(deps)* update bevy dependencies and suppress clippy lint
- *(deps)* explicitly disable bevy default features in crates
- *(deps)* update fontdue requirement from 0.7 to 0.9 ([#3](https://github.com/Bli-AIk/bevy_bitmap_text/pull/3))
- *(deps)* update etagere requirement from 0.2 to 0.3 ([#5](https://github.com/Bli-AIk/bevy_bitmap_text/pull/5))
- *(lint)* enforce dual tokei thresholds ([#4](https://github.com/Bli-AIk/bevy_bitmap_text/pull/4))

### Refactor

- *(core)* add font layout override support
- *(bevy_bitmap_text)* remove stale readme table formatting
- standardize terminology from "资产" to "资源"
- *(bevy_bitmap_text)* restructure system scheduling and font loading

## [0.1.1](https://github.com/Bli-AIk/bevy_bitmap_text/compare/v0.1.0...v0.1.1) - 2026-03-25

### Fixed

- support fontdue 0.9 font settings

### Miscellaneous Tasks

- *(deps)* update fontdue requirement from 0.7 to 0.9
- *(lint)* enforce dual tokei thresholds ([#4](https://github.com/Bli-AIk/bevy_bitmap_text/pull/4))
