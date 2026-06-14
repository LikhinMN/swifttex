# API Reference

If you are bypassing the framework wrappers and interacting directly with the `swifttex-wasm` module, this is the core API.

## `init()`
Initializes the WebAssembly module. This **must** be awaited before calling `render` or `register_symbol`.

```javascript
import init from 'swifttex-wasm';

await init(); // Fetches and instantiates the .wasm binary
```

## `render(tex, options?)`

The core rendering function. Parses the LaTeX string and performs box-layout calculations, returning generated markup.

**Arguments:**
1. `tex` *(string)*: The LaTeX mathematical string.
2. `options` *(object, optional)*: Configuration options.

**Returns:** `RenderResult | RenderError`

```javascript
const result = render("\\frac{1}{2}", { display_mode: true });
```

### `RenderOptions`

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `output` | `"svg" \| "mathml" \| "both"` | `"svg"` | The format of the output markup. |
| `display_mode` | `boolean` | `false` | Renders the math in display style (centered, larger limits) instead of inline. |
| `font_size` | `number` | `16.0` | Base font size used to calculate `em` relative layout metrics. |
| `math_style` | `"display" \| "text" \| "script" \| "scriptscript"` | `"text"` | Forces a specific mathematical style cascade. |
| `inline_fonts` | `boolean` | `false` | If true, `<style>` definitions for KaTeX fonts are injected directly into the SVG. Useful for downloading standalone SVG files. |

### `RenderResult`

| Field | Type | Description |
|-------|------|-------------|
| `svg` | `string?` | The raw `<svg>` HTML string. Only present if `output` was `"svg"` or `"both"`. |
| `mathml` | `string?` | The raw `<math>` HTML string. Only present if `output` was `"mathml"` or `"both"`. |
| `width` | `number` | The calculated layout width in pixels (assuming base `font_size`). |
| `height` | `number` | The calculated layout height in pixels. |

### `RenderError`

| Field | Type | Description |
|-------|------|-------------|
| `error` | `string` | The parsing or layout error message. |

---

## `register_symbol(command, unicode_char)`

Allows you to dynamically register custom symbol macros into the global SwiftTeX parser registry.

```javascript
import { register_symbol, render } from 'swifttex-wasm';

// Registers \myStar to render the '★' character
register_symbol("myStar", "★");

const result = render("\\myStar");
```
