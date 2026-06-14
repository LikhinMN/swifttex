import * as React from 'react';
import { RenderOptions } from '../swifttex.d.ts';

export interface SwiftTeXProps extends RenderOptions {
    /** The LaTeX math string to render */
    tex: string;
    /** Additional CSS class names */
    className?: string;
    /** Inline CSS styles */
    style?: React.CSSProperties;
    /** The HTML element to render as (default: span or div based on display_mode) */
    as?: keyof React.JSX.IntrinsicElements;
}

/**
 * A fast, accessible LaTeX math component using SwiftTeX.
 */
export const SwiftTeX: React.FC<SwiftTeXProps>;
