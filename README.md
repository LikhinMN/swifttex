# SwiftTeX

Fast, accurate LaTeX math rendering — written in Rust, compiled to WebAssembly.

**"Beautiful Mathematics. Native Speed."**

## Features

- SVG and MathML output
- KaTeX-quality typesetting via KaTeX webfonts
- Math style cascade (Display/Text/Script/ScriptScript)
- Matrices, delimiters, big operators
- Extensible via plugin API
- Screen reader accessible
- < 300KB WASM bundle (compressed)

## Install

```bash
npm install swifttex-wasm
```

## Quick Start

```js
import init, { render } from "swifttex-wasm";
await init();

// SVG output
const { svg, width, height } = render(String.raw`\frac{x^2}{y}`);
document.getElementById("math").innerHTML = svg;

// MathML output
const { mathml } = render(String.raw`\frac{x^2}{y}`, { output: "mathml" });

// Both
const result = render(String.raw`\frac{x^2}{y}`, {
  output: "both",
  display_mode: true,
  font_size: 18,
});
```

## Plugin API

```js
import { register_symbol } from "swifttex-wasm";
register_symbol("hbar", "ℏ");

const { svg } = render(String.raw`\hbar`);
```

## React Component

```jsx
import { SwiftTeX } from "swifttex/react";
<SwiftTeX tex="\frac{x^2}{y}" displayMode />
```

## Performance

| Expression                        | SwiftTeX | KaTeX  |
|-----------------------------------|----------|--------|
| x^2                               | <0.1ms   | ~0.5ms |
| \frac{x^2}{\sqrt{y}}              | <0.5ms   | ~1ms   |
| \sum_{i=0}^{n} \frac{x_i}{\sigma} | <1ms     | ~2ms   |

## Crates

| Crate                    | Purpose                        |
|--------------------------|--------------------------------|
| swifttex-lexer           | Tokenizer                      |
| swifttex-parser          | AST parser                     |
| swifttex-layout          | TeX box model layout engine    |
| swifttex-renderer-svg    | SVG renderer                   |
| swifttex-renderer-mathml | MathML renderer                |
| swifttex-plugin-api      | Plugin trait and registry      |
| swifttex-wasm            | WASM bindings + npm package    |

## License
MIT
