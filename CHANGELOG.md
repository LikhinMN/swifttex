# Changelog

## [0.3.0] — V3

### Added
- MathML export via swifttex-renderer-mathml crate
- Plugin system via swifttex-plugin-api crate
- Screen reader support: aria-label generation from AST
- Dual SVG+MathML output via render_accessible()
- Math accents: \hat \bar \vec \dot \ddot \tilde
- Text operators: \sin \cos \tan \log \ln \lim etc.
- Spacing commands: \, \: \; \! \quad \qquad
- Extended symbol table: \infty \partial \nabla \pm \times \div \leq \geq \neq \approx \in \notin \to \forall \exists
- OutputFormat enum in WASM API: "svg" | "mathml" | "both"
- register_symbol() and list_plugins() in WASM API
- Example PhysicsPlugin: \hbar \bra \ket \braket

## [0.2.0] — V2

### Added
- KaTeX webfont rendering replacing geometric approximations
- Real KaTeX font metrics via phf static tables
- Math style cascade: Display/Text/Script/ScriptScript
- Matrices: \begin{pmatrix} \begin{bmatrix} \begin{vmatrix} \begin{cases}
- Delimiter sizing: \left \right \big \Big \bigg \Bigg
- Big operators with limits: \sum \int \prod \bigcup \bigcap
- style.css package with KaTeX font import
- inline_fonts option in render API

## [0.1.0] — V1

### Added
- Lexer, parser, layout engine, SVG renderer
- WASM bindings with TypeScript types
- Core LaTeX: fractions, superscripts, subscripts, square roots
- Greek symbols, basic operators
- Cargo workspace with 5 crates
- GitHub Actions CI
