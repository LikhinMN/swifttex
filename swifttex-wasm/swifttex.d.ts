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

export function render(input: string, options?: RenderOptions): RenderResult | RenderError;

export function register_symbol(name: string, unicode_char: string): void;
export function list_plugins(): string[];
export function reset_registry(): void;
export function version(): string;
