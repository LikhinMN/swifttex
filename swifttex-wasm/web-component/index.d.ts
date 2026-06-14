import { RenderOptions } from '../swifttex.d.ts';

export class SwiftTexElement extends HTMLElement {
    /** The LaTeX math string to render */
    tex: string;
    /** Forces a re-render of the math content */
    renderMath(): void;
}

/**
 * Defines the custom element in the browser's CustomElementRegistry.
 * @param tagName The name of the tag to register (default: 'swift-tex')
 */
export function defineWebComponent(tagName?: string): void;

declare global {
    interface HTMLElementTagNameMap {
        'swift-tex': SwiftTexElement;
    }
}
