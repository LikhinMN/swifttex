import { useEffect, useRef, useState } from "react";

export function SwiftTeX({ tex, displayMode = false, fontSize = 16, className = "", output = "svg" }) {
  const [result, setResult] = useState(null);
  const wasmRef = useRef(null);

  useEffect(() => {
    import("swifttex").then(wasm => {
      wasmRef.current = wasm;
      const out = wasm.render(tex, { display_mode: displayMode, font_size: fontSize, output });
      setResult(out);
    });
  }, [tex, displayMode, fontSize, output]);

  const baseClass = displayMode ? "swifttex-display" : "swifttex";
  const finalClass = className ? `${baseClass} ${className}` : baseClass;

  if (!result) return <span className={finalClass} style={{ width: 0, overflow: "hidden" }}>&#8203;</span>;
  if (result.error) return <span className={finalClass} style={{ color: "red" }}>{result.error}</span>;

  if (output === "mathml" && result.mathml) {
    return <span className={finalClass} dangerouslySetInnerHTML={{ __html: result.mathml }} />;
  } else if (output === "both" && result.svg && result.mathml) {
    return (
      <span className={finalClass}>
        <span aria-hidden="true" dangerouslySetInnerHTML={{ __html: result.svg }} />
        <span aria-live="polite" style={{ position: "absolute", width: 1, height: 1, padding: 0, margin: -1, overflow: "hidden", clip: "rect(0, 0, 0, 0)", whiteSpace: "nowrap", border: 0 }} dangerouslySetInnerHTML={{ __html: result.mathml }} />
      </span>
    );
  }

  return (
    <span
      className={finalClass}
      dangerouslySetInnerHTML={{ __html: result.svg }}
    />
  );
}
