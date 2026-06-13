import { useEffect, useRef, useState } from "react";

export function SwiftTeX({ tex, displayMode = false, fontSize = 16 }) {
  const [result, setResult] = useState(null);
  const wasmRef = useRef(null);

  useEffect(() => {
    import("swifttex").then(wasm => {
      wasmRef.current = wasm;
      const out = wasm.render(tex, { display_mode: displayMode, font_size: fontSize });
      setResult(out);
    });
  }, [tex, displayMode, fontSize]);

  if (!result) return null;
  if (result.error) return <span style={{ color: "red" }}>{result.error}</span>;
  return (
    <span
      style={{ display: displayMode ? "block" : "inline-block" }}
      dangerouslySetInnerHTML={{ __html: result.svg }}
    />
  );
}
