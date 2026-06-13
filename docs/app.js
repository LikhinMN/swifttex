import init, { render } from './pkg/swifttex_wasm.js';

const inputEl = document.getElementById('latex-input');
const outputEl = document.getElementById('render-output');
const errorEl = document.getElementById('error-output');
const metricsEl = document.getElementById('render-metrics');
const formatSelect = document.getElementById('output-format');
const displayModeCheck = document.getElementById('display-mode');

let isWasmLoaded = false;

async function bootstrap() {
    try {
        await init();
        isWasmLoaded = true;
        updateRender();
    } catch (e) {
        showError("Failed to initialize WebAssembly module: " + e.message);
    }
}

function updateRender() {
    if (!isWasmLoaded) return;
    
    const tex = inputEl.value;
    const format = formatSelect.value;
    const displayMode = displayModeCheck.checked;
    
    if (!tex.trim()) {
        outputEl.innerHTML = "";
        hideError();
        metricsEl.innerText = "Empty";
        return;
    }

    try {
        const start = performance.now();
        
        const result = render(tex, {
            output: format,
            display_mode: displayMode,
            inline_fonts: false
        });
        
        const end = performance.now();
        metricsEl.innerText = `Rendered in ${(end - start).toFixed(2)}ms`;
        hideError();
        
        if (format === 'svg') {
            outputEl.innerHTML = result.svg;
        } else if (format === 'mathml') {
            outputEl.innerHTML = `<div style="font-size: 1.5em; overflow: auto; width: 100%; text-align: center;">${result.mathml}</div>`;
        } else {
            outputEl.innerHTML = `<div style="display:flex;flex-direction:column;gap:1rem;align-items:center;width:100%;">
                <div style="border-bottom: 1px dashed #ccc; padding-bottom: 1rem; width: 100%; text-align: center;">${result.svg}</div>
                <div style="font-size: 1.5em;">${result.mathml}</div>
            </div>`;
        }
    } catch (e) {
        showError(e.toString());
    }
}

function showError(msg) {
    errorEl.innerText = msg;
    errorEl.style.display = 'block';
}

function hideError() {
    errorEl.style.display = 'none';
}

inputEl.addEventListener('input', updateRender);
formatSelect.addEventListener('change', updateRender);
displayModeCheck.addEventListener('change', updateRender);

bootstrap();
