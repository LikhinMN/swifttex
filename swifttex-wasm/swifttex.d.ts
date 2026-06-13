export type OutputFormat = "svg" | "mathml" | "both";

export interface RenderOptions {
  font_size?: number;
  display_mode?: boolean;
  inline_fonts?: boolean;
  math_style?: "display" | "text" | "script" | "scriptscript";
  output?: OutputFormat;
}

export interface RenderResult {
  svg?: string;
  mathml?: string;
  width: number;
  height: number;
}

export interface RenderError {
  error: string;
}

export function render(input: string, opts?: RenderOptions): RenderResult | RenderError;
export function version(): string;
