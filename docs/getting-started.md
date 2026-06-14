# Getting Started

This guide will help you install SwiftTeX and start rendering math in your vanilla JavaScript/HTML project. 

*(If you are using a frontend framework, check out the Framework Guides in the sidebar!)*

## Installation

You can install SwiftTeX via your preferred package manager:

```bash
npm install swifttex-wasm
```

## Basic Usage

To use SwiftTeX, you must first initialize the WebAssembly module, and then you can call the `render` function.

### 1. Import the CSS

SwiftTeX relies on CSS to provide the correct fonts (like `KaTeX_Main` and `KaTeX_Math`) and spacing. Import it into your HTML or via your JS bundler:

```html
<link rel="stylesheet" href="node_modules/swifttex-wasm/style.css">
```
*Or in JS:*
```javascript
import 'swifttex-wasm/style.css';
```

### 2. Render Math

```javascript
import init, { render } from "swifttex-wasm";

// Initialize the WASM module. This only needs to be done once!
await init();

// The math string to render (use String.raw to avoid escaping backslashes)
const latexString = String.raw`\int_0^\infty x^2 dx`;

// Render it!
const result = render(latexString, {
    display_mode: true,
    output: 'svg' // or 'mathml', or 'both'
});

// Inject it into your page
if (!result.error) {
    document.getElementById('math-container').innerHTML = result.svg;
} else {
    console.error("Render failed:", result.error);
}
```

## Configuration Options

The `render` function accepts a second argument, an `options` object:

- `output`: `"svg" | "mathml" | "both"` (Default: `"svg"`)
- `display_mode`: `boolean`. If true, renders block-style math (centered, limits above/below). (Default: `false`)
- `font_size`: `number`. Base font size for layout scaling. (Default: `16.0`)
- `math_style`: `"display" | "text" | "script" | "scriptscript"`. Explicitly forces a layout style.
- `inline_fonts`: `boolean`. If true, injects the font-faces directly into the SVG output.
