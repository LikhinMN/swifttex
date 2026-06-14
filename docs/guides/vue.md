# Vue 3 Guide

SwiftTeX provides a native Vue 3 component built with the Composition API. It reacts seamlessly to reactive state changes.

## Import

```vue
<script setup>
import SwiftTeX from 'swifttex-wasm/vue';
import 'swifttex-wasm/style.css';
</script>
```

## Usage

```vue
<template>
  <div>
    <p>
      The area of a circle is 
      <SwiftTeX tex="A = \pi r^2" />
    </p>

    <p>And here is a dynamic equation:</p>
    <SwiftTeX :tex="dynamicEquation" displayMode />

    <button @click="changeEquation">Change</button>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import SwiftTeX from 'swifttex-wasm/vue';
import 'swifttex-wasm/style.css';

const dynamicEquation = ref("\\sum_{i=1}^{n} i = \\frac{n(n+1)}{2}");

function changeEquation() {
  dynamicEquation.value = "\\int_0^1 x^2 dx = \\frac{1}{3}";
}
</script>
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `tex` | `String` | **Required** | The LaTeX string. |
| `displayMode` | `Boolean` | `false` | Renders in block display mode. |
| `output` | `String` | `"svg"` | `"svg"`, `"mathml"`, or `"both"`. |
| `className` | `String` | `""` | Additional CSS classes. |
