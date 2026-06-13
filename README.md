# SwiftTeX

Fast, accurate LaTeX math rendering engine — written in Rust, compiled to WebAssembly.

## Install
```
npm install swifttex
```

## Usage
```js
import init, { render } from "swifttex";
await init();

const result = render("\\frac{x^2}{y}", { display_mode: true });
console.log(result.svg);
```

## Performance
| Expression         | SwiftTeX | KaTeX  |
|--------------------|----------|--------|
| x^2                | <0.1ms   | ~0.5ms |
| \frac{x^2}{\sqrt{y}} | <1ms  | ~2ms   |

(Benchmarks on M1 MacBook Pro — update with real numbers after Sprint 6)

## License
MIT OR Apache-2.0
