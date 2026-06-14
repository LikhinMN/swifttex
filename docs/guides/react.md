# React Guide

Using SwiftTeX in React is incredibly straightforward. We ship a `<SwiftTeX />` component out-of-the-box that manages the WebAssembly rendering lifecycle using optimized `useMemo` hooks.

## Import

```jsx
import { SwiftTeX } from 'swifttex-wasm/react';
import 'swifttex-wasm/style.css'; // Don't forget the styles!
```

## Usage

Pass your mathematical string into the `tex` prop. 

> **Tip:** Use `String.raw` or double-escape your backslashes (e.g. `"\\frac{1}{2}"`) so that JavaScript doesn't swallow the backslashes!

```jsx
function MathComponent() {
  return (
    <div>
      <p>
        The quadratic formula is: 
        <SwiftTeX tex="x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}" />
      </p>

      <p>And here is a block equation:</p>
      <SwiftTeX 
        tex="\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}" 
        display_mode={true} 
      />
    </div>
  );
}
```

## Props

The `<SwiftTeX />` component accepts all of the core `RenderOptions`, plus standard React attributes:

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `tex` | `string` | **Required** | The LaTeX string to render. |
| `display_mode` | `boolean` | `false` | Renders in block display mode. |
| `output` | `"svg" \| "mathml" \| "both"` | `"svg"` | Output format. |
| `className` | `string` | `""` | Additional CSS classes for the container. |
| `style` | `React.CSSProperties` | `{}` | Inline styles applied to the container. |
| `as` | `string` | `"span"` or `"div"` | The wrapper HTML tag. Defaults to `div` if `display_mode` is true, otherwise `span`. |
