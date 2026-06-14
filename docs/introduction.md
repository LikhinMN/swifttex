# Introduction

Welcome to **SwiftTeX**! 

SwiftTeX is a modern, high-performance LaTeX math rendering engine. It was built from the ground up in **Rust** and compiled to **WebAssembly (WASM)** to deliver unmatched speed while maintaining pixel-perfect aesthetic typesetting comparable to KaTeX.

## Core Philosophy

Mathematical rendering on the web has traditionally relied on heavy JavaScript libraries. While powerful, they can become a performance bottleneck when rendering hundreds of complex equations on a single page.

SwiftTeX solves this by moving the heavy lifting (Lexing, Parsing, and Layout calculations) to WebAssembly. The result is:
- **Instantaneous Rendering**: Parse and layout equations in fractions of a millisecond.
- **Zero Layout Thrashing**: Generates pure `<svg>` or `<math>` strings that can be safely injected into the DOM without causing expensive browser reflows.
- **Universal Portability**: By wrapping the core WASM logic, we distribute highly optimized, native-feeling components for React, Vue, Svelte, and vanilla HTML.

## Output Formats

SwiftTeX supports dual output modes:

1. **SVG (Default)**
   Generates a pure, scalable vector graphic using KaTeX font metrics. This ensures that the math looks exactly the same on every browser and device, regardless of local font availability.
2. **MathML**
   Generates native `<math>` tags according to the MathML 3.0 specification. This is critical for screen-reader accessibility and copying/pasting math.

You can configure SwiftTeX to output both simultaneously, creating a visually perfect yet fully accessible math element!
