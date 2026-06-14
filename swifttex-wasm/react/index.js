import React, { useMemo } from 'react';
import { render } from '../pkg/swifttex.js';

export function SwiftTeX({ tex, className, style, as, ...options }) {
    const result = useMemo(() => {
        try {
            return render(tex, options);
        } catch (e) {
            return { error: String(e) };
        }
    }, [tex, options.output, options.display_mode, options.font_size, options.inline_fonts, options.math_style]);

    const Component = as || (options.display_mode ? 'div' : 'span');

    if (result.error) {
        return React.createElement(Component, { className, style: { ...style, color: 'red' } }, result.error);
    }

    let innerHTML = '';
    if (result.svg && result.mathml) {
        innerHTML = result.mathml + result.svg; 
    } else if (result.svg) {
        innerHTML = result.svg;
    } else if (result.mathml) {
        innerHTML = result.mathml;
    }

    const mergedStyle = {
        ...style,
        display: options.display_mode ? 'block' : 'inline-block'
    };

    return React.createElement(Component, {
        className: className ? `swifttex-container ${className}` : 'swifttex-container',
        style: mergedStyle,
        dangerouslySetInnerHTML: { __html: innerHTML }
    });
}
