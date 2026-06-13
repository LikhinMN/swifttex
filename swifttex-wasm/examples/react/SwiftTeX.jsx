import { useEffect, useRef, useState } from "react";

export function SwiftTeX({ tex, displayMode = false, fontSize = 16, className = "" }) {
  const [result, setResult] = useState(null);
  const wasmRef = useRef(null);

  useEffect(() => {
    import("swifttex").then(wasm => {
      wasmRef.current = wasm;
      const out = wasm.render(tex, { display_mode: displayMode, font_size: fontSize });
      setResult(out);
    });
  }, [tex, displayMode, fontSize]);

  const baseClass = displayMode ? "swifttex-display" : "swifttex";
  const finalClass = className ? `${baseClass} ${className}` : baseClass;

  if (!result) return <span className={finalClass} style={{ width: 0, overflow: "hidden" }}>&#8203;</span>;
  if (result.error) return <span className={finalClass} style={{ color: "red" }}>{result.error}</span>;
  return (
    <span
      className={finalClass}
      dangerouslySetInnerHTML={{ __html: result.svg }}
    />
  );
}
