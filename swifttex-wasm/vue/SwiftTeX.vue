<template>
  <component
    :is="displayMode ? 'div' : 'span'"
    :class="['swifttex-container', className]"
    :style="mergedStyle"
    v-html="innerHTML"
  />
</template>

<script setup>
import { computed } from 'vue';
import { render } from '../pkg/swifttex.js';

const props = defineProps({
  tex: {
    type: String,
    required: true
  },
  className: {
    type: String,
    default: ''
  },
  style: {
    type: Object,
    default: () => ({})
  },
  displayMode: Boolean,
  fontSize: Number,
  output: String,
  mathStyle: String,
  inlineFonts: Boolean
});

const result = computed(() => {
  const options = {
    display_mode: props.displayMode,
    font_size: props.fontSize,
    output: props.output,
    math_style: props.mathStyle,
    inline_fonts: props.inlineFonts
  };
  
  // Remove undefined fields
  Object.keys(options).forEach(key => options[key] === undefined && delete options[key]);

  try {
    return render(props.tex, options);
  } catch (e) {
    return { error: String(e) };
  }
});

const innerHTML = computed(() => {
  if (result.value.error) {
    return `<span style="color: red;">${result.value.error}</span>`;
  }
  
  if (result.value.svg && result.value.mathml) {
    return result.value.mathml + result.value.svg;
  } else if (result.value.svg) {
    return result.value.svg;
  } else if (result.value.mathml) {
    return result.value.mathml;
  }
  return '';
});

const mergedStyle = computed(() => {
  if (result.value.error) {
    return { ...props.style, color: 'red' };
  }
  return {
    ...props.style,
    display: props.displayMode ? 'block' : 'inline-block'
  };
});
</script>
