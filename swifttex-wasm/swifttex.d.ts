export interface RenderOptions {
  font_size?: number;
  display_mode?: boolean;
}

export interface RenderResult {
  svg: string;
  width: number;
  height: number;
}

export interface RenderError {
  error: string;
}

export function render(input: string, opts?: RenderOptions): RenderResult | RenderError;
export function version(): string;
