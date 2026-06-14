<script>
    import { render } from '../pkg/swifttex.js';

    export let tex = '';
    export let className = '';
    export let style = '';
    export let displayMode = false;
    export let fontSize = undefined;
    export let output = undefined;
    export let mathStyle = undefined;
    export let inlineFonts = undefined;

    let innerHTML = '';
    let isError = false;

    $: {
        const options = {
            display_mode: displayMode,
            font_size: fontSize,
            output: output,
            math_style: mathStyle,
            inline_fonts: inlineFonts
        };

        // Remove undefined fields
        Object.keys(options).forEach(key => options[key] === undefined && delete options[key]);

        try {
            const result = render(tex, options);
            if (result.svg && result.mathml) {
                innerHTML = result.mathml + result.svg;
            } else if (result.svg) {
                innerHTML = result.svg;
            } else if (result.mathml) {
                innerHTML = result.mathml;
            }
            isError = false;
        } catch (e) {
            innerHTML = String(e);
            isError = true;
        }
    }
</script>

{#if displayMode}
    <div class="swifttex-container {className}" style="{style}{isError ? '; color: red;' : '; display: block;'}">
        {@html innerHTML}
    </div>
{:else}
    <span class="swifttex-container {className}" style="{style}{isError ? '; color: red;' : '; display: inline-block;'}">
        {@html innerHTML}
    </span>
{/if}
