import { render } from '../pkg/swifttex.js';

export class SwiftTexElement extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    static get observedAttributes() {
        return ['tex', 'display-mode', 'font-size', 'output', 'math-style', 'inline-fonts'];
    }

    connectedCallback() {
        this.renderMath();
    }

    attributeChangedCallback(name, oldValue, newValue) {
        if (oldValue !== newValue) {
            this.renderMath();
        }
    }

    get tex() {
        return this.getAttribute('tex') || this.textContent || '';
    }

    set tex(value) {
        this.setAttribute('tex', value);
    }

    renderMath() {
        const tex = this.tex;
        if (!tex.trim()) return;

        const options = {
            display_mode: this.hasAttribute('display-mode') && this.getAttribute('display-mode') !== 'false',
            font_size: this.hasAttribute('font-size') ? parseFloat(this.getAttribute('font-size')) : undefined,
            output: this.getAttribute('output') || undefined,
            math_style: this.getAttribute('math-style') || undefined,
            inline_fonts: this.hasAttribute('inline-fonts') && this.getAttribute('inline-fonts') !== 'false',
        };

        // Remove undefined fields
        Object.keys(options).forEach(key => options[key] === undefined && delete options[key]);

        try {
            const result = render(tex, options);
            let innerHTML = '';
            if (result.svg && result.mathml) {
                innerHTML = result.mathml + result.svg;
            } else if (result.svg) {
                innerHTML = result.svg;
            } else if (result.mathml) {
                innerHTML = result.mathml;
            }

            this.shadowRoot.innerHTML = `
                <style>
                    :host {
                        display: ${options.display_mode ? 'block' : 'inline-block'};
                    }
                </style>
                ${innerHTML}
            `;
        } catch (e) {
            this.shadowRoot.innerHTML = `<span style="color: red;">${String(e)}</span>`;
        }
    }
}

export function defineWebComponent(tagName = 'swift-tex') {
    if (typeof customElements !== 'undefined' && !customElements.get(tagName)) {
        customElements.define(tagName, SwiftTexElement);
    }
}
