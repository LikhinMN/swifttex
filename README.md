<div align="center">
  <h1>🚀 SwiftTeX</h1>
  <p><strong>Fast, accurate LaTeX math rendering engine written in Rust, compiled to WebAssembly.</strong></p>

  <p>
    <a href="https://www.npmjs.com/package/swifttex-wasm"><img src="https://img.shields.io/npm/v/swifttex-wasm.svg?style=flat-square" alt="NPM Version" /></a>
    <img src="https://img.shields.io/badge/bundle-size-success" alt="Bundle Size" />
    <img src="https://img.shields.io/npm/l/swifttex-wasm.svg?style=flat-square" alt="License" />
  </p>
</div>

---

**SwiftTeX** is a highly optimized LaTeX math rendering engine for the web. By leveraging Rust and WebAssembly, it achieves incredible rendering speeds—outperforming JavaScript-based libraries like KaTeX while maintaining the same beautiful, pixel-perfect output.

It provides out-of-the-box framework components for **React**, **Vue**, **Svelte**, and native **Web Components**, so you can easily drop fast math rendering into any modern stack.

## ✨ Why SwiftTeX?

- **Blazing Fast**: Up to 5x faster than KaTeX for complex equations thanks to WebAssembly.
- **Universal Framework Support**: Ready-to-use bindings for React, Vue, Svelte, and Vanilla JS.
- **Beautiful Output**: Uses the industry-standard KaTeX font metrics and CSS for perfect typesetting.
- **Dual Output Modes**: Renders to **SVG** (for standalone visual perfection) and **MathML** (for accessibility).
- **Accessible**: Screen-reader friendly by default with generated ARIA labels and MathML.
- **Extensible**: Powerful plugin API to define custom symbols and macros.

---

## 📦 Installation

Install SwiftTeX and its peer dependencies via your favorite package manager:

```bash
# npm
npm install swifttex-wasm

# yarn
yarn add swifttex-wasm

# pnpm
pnpm add swifttex-wasm
```

### Don't forget the CSS!
To ensure your math renders beautifully, you must include the SwiftTeX stylesheet in your application (this applies the KaTeX webfonts and spacing rules).

```javascript
// In your app's entry point (e.g., index.js, main.js, _app.jsx)
import 'swifttex-wasm/style.css';
```

---

## 💻 Framework Integrations

We provide pre-optimized wrappers that handle the WebAssembly lifecycle and DOM injection for you.

### ⚛️ React

```jsx
import { SwiftTeX } from "swifttex-wasm/react";
import "swifttex-wasm/style.css";

function App() {
  return (
    <main>
      <h2>Inline Math: <SwiftTeX tex="E = mc^2" /></h2>
      <h2>Block Math:</h2>
      <SwiftTeX 
        tex="\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}" 
        display_mode={true} 
      />
    </main>
  );
}
```

### 💚 Vue 3

```vue
<template>
  <main>
    <h2>Inline Math: <SwiftTeX tex="E = mc^2" /></h2>
    <h2>Block Math:</h2>
    <SwiftTeX 
      tex="\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}" 
      displayMode 
    />
  </main>
</template>

<script setup>
import SwiftTeX from "swifttex-wasm/vue";
import "swifttex-wasm/style.css";
</script>
```

### 🧡 Svelte

```svelte
<script>
  import SwiftTeX from "swifttex-wasm/svelte";
  import "swifttex-wasm/style.css";
</script>

<main>
  <h2>Inline Math: <SwiftTeX tex="E = mc^2" /></h2>
  <h2>Block Math:</h2>
  <SwiftTeX 
    tex="\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}" 
    displayMode={true} 
  />
</main>
```

### 🌐 Web Components (Vanilla HTML/JS)

If you aren't using a framework, or want a framework-agnostic solution, register our custom `<swift-tex>` element:

```html
<head>
  <link rel="stylesheet" href="node_modules/swifttex-wasm/style.css">
  <script type="module">
    import { defineWebComponent } from "swifttex-wasm/web-component";
    defineWebComponent(); // Registers <swift-tex>
  </script>
</head>
<body>
  <h2>Inline Math: <swift-tex tex="E = mc^2"></swift-tex></h2>
  
  <h2>Block Math:</h2>
  <swift-tex tex="\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}" display-mode="true"></swift-tex>
</body>
```

---

## 🛠 Core API (Vanilla JS)

If you prefer to render the math strings manually, you can use the core `render` function.

```javascript
import init, { render } from "swifttex-wasm";

// Initialize the WebAssembly module (only needed once)
await init();

// 1. Render to SVG (Default)
const svgResult = render(String.raw`\frac{x^2}{y}`);
document.getElementById("math-container").innerHTML = svgResult.svg;

// 2. Render to MathML (Accessibility Focus)
const mathmlResult = render(String.raw`x \leq y`, { output: "mathml" });
document.getElementById("accessible-math").innerHTML = mathmlResult.mathml;

// 3. Render Both with Advanced Options
const result = render(String.raw`\sum_{i=0}^{n} i^2`, {
  output: "both",
  display_mode: true,
  font_size: 18.0,
});
```

### `render(tex: string, options?: RenderOptions)`

#### `RenderOptions`
| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `output` | `"svg" \| "mathml" \| "both"` | `"svg"` | The format of the output markup. |
| `display_mode` | `boolean` | `false` | If `true`, renders the math in display mode (centered, larger limits). If `false`, renders in inline mode. |
| `font_size` | `number` | `16.0` | The base font size for scaling the SVGs and layout metrics. |
| `math_style` | `"display" \| "text" \| "script" \| "scriptscript"` | `"text"` | Explicitly force a specific TeX math style. |
| `inline_fonts` | `boolean` | `false` | If `true`, SVG output will contain inline `<style>` tags rather than relying on the external CSS file. |

#### Returns
The function returns an object containing:
- `svg?: string` - The generated SVG markup.
- `mathml?: string` - The generated MathML markup.
- `width: number` - The calculated layout width.
- `height: number` - The calculated layout height.

---

## 🔌 Plugin API

SwiftTeX is highly extensible. You can register custom commands and symbols into the global registry at runtime:

```javascript
import { register_symbol, render } from "swifttex-wasm";

// Add a custom symbol
register_symbol("hbar", "ℏ");

// You can now use \hbar in your equations!
const { svg } = render(String.raw`\hbar \omega`);
```

---

## ⚡ Performance

SwiftTeX operates directly on memory-safe Rust compiled to WASM. Below are benchmark comparisons for common expressions rendered to SVG:

| Expression                        | SwiftTeX | KaTeX  |
|-----------------------------------|----------|--------|
| `x^2`                             | **<0.1ms**   | ~0.5ms |
| `\frac{x^2}{\sqrt{y}}`            | **<0.5ms**   | ~1ms   |
| `\sum_{i=0}^{n} \frac{x_i}{\sigma}` | **<1ms**     | ~2ms   |

---

## 📄 License
MIT © [LikhinMN](https://github.com/LikhinMN)
