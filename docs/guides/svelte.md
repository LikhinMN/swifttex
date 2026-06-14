# Svelte Guide

SwiftTeX provides a high-performance Svelte component that utilizes Svelte's reactive `$:` statements to compile WASM math bindings only when props change.

## Import

```svelte
<script>
  import SwiftTeX from 'swifttex-wasm/svelte';
  import 'swifttex-wasm/style.css';
</script>
```

## Usage

```svelte
<script>
  import SwiftTeX from 'swifttex-wasm/svelte';
  import 'swifttex-wasm/style.css';

  let equation = "\\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}";
</script>

<main>
  <p>
    Inline mode: <SwiftTeX tex="E = mc^2" />
  </p>

  <p>Display mode:</p>
  <SwiftTeX tex={equation} displayMode={true} />
</main>
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `tex` | `string` | `""` | The LaTeX string. |
| `displayMode` | `boolean` | `false` | Renders in block display mode. |
| `output` | `string` | `"svg"` | `"svg"`, `"mathml"`, or `"both"`. |
| `className` | `string` | `""` | Additional CSS classes. |
