# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-06-14

### Added
- **Framework Support**: Shipped pre-compiled wrapper components for the following modern frameworks out of the box:
  - React Component (`<SwiftTeX />`) using `useMemo` hooks.
  - Vue 3 Component (`<SwiftTeX />`) using the Composition API.
  - Svelte Component (`<SwiftTeX />`) using reactive blocks.
  - Framework-agnostic native Web Component (`<swift-tex>`).
- **Font Styles & Text Mode**: Added support for standard LaTeX style formatting:
  - `\mathbf`, `\mathcal`, `\mathbb`, `\mathrm`, `\mathit`, `\text`.
  - Upgraded both SVG and MathML output generation to render these appropriately.
- **Optional Arguments**: Added layout and parsing logic for commands with optional arguments. Specifically, `\sqrt[n]{x}` is now fully supported.
- **VitePress Documentation**: Completely rebuilt the static documentation into a beautiful, structured VitePress site with dedicated framework guides and API references.

### Changed
- Refactored `swifttex-plugin-api` AST:
  - Added `TextStyle` enum and `Node::Style` to represent text styling.
  - Updated `Node::SquareRoot` to include an `index: Option<Box<Node>>`.
- Modified layout engine to safely cascade styles into nested groups.

### Fixed
- Fixed MathML accessibility trees to use standard `<mroot>` tags.
- Fixed `generate_aria_label` inside SVG output to handle multi-layered AST components accurately.
