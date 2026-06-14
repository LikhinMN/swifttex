# Web Components (Vanilla)

If you aren't using a specific frontend framework, or prefer framework-agnostic native elements, SwiftTeX ships a native **Web Component** (`<swift-tex>`).

## Setup

You need to register the custom element in your JavaScript bundle once:

```javascript
import { defineWebComponent } from "swifttex-wasm/web-component";
import "swifttex-wasm/style.css";

// This registers the <swift-tex> element with the browser
defineWebComponent();
```

## Usage in HTML

Once registered, you can use the `<swift-tex>` tag anywhere in your raw HTML!

```html
<p>
  The Pythagorean theorem is <swift-tex tex="a^2 + b^2 = c^2"></swift-tex>.
</p>

<p>Here is Euler's identity in block mode:</p>
<swift-tex tex="e^{i\pi} + 1 = 0" display-mode="true"></swift-tex>
```

## Attributes

The custom element observes the following attributes. Modifying them via JavaScript (e.g., `element.setAttribute(...)`) will automatically trigger a highly optimized re-render.

| Attribute | Maps To | Description |
|-----------|---------|-------------|
| `tex` | The equation string | The LaTeX string. (Alternatively, you can just place the text inside the element: `<swift-tex>x^2</swift-tex>`) |
| `display-mode` | `display_mode` | Set to `"true"` for block display mode. |
| `output` | `output` | `"svg"`, `"mathml"`, or `"both"`. |
| `font-size` | `font_size` | Number. Sets the layout scaling factor. |
