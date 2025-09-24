import { type Wasm } from "./wasm.ts";
import { DOMTag } from "@portal-solutions/morphic-wasm-dep-rollup";
/*#__NO_SIDE_EFFECTS__*/
export function createInterface(wasm: Wasm, args: Args = {}): Interface {
  return new Impl(wasm, args);
}
export interface Interface {
  hookHTML(global: typeof globalThis): void;
}
export type Args = {
  createElement?: typeof document.createElement;
};
type Core = InstanceType<Wasm["Core"]>;
class Impl implements Interface {
  #core: Core;
  #createElement: typeof document.createElement;
  constructor(
    { Core }: Wasm,
    { createElement = document.createElement.bind(document) }: Args = {}
  ) {
    this.#createElement = createElement;
    this.#core = new Core({
        createCanvas: () => this.#createElement("canvas"),
    });
  }
  #didHookHTML: WeakSet<typeof globalThis> = new WeakSet();
  #hasHooked = this.#didHookHTML.has.bind(this.#didHookHTML);
  #addHooked = this.#didHookHTML.add.bind(this.#didHookHTML);
  #hookHTML(global: typeof globalThis) {
    if (this.#hasHooked(global)) return;
    this.#addHooked(global);
  }
  get hookHTML() {
    return (global: typeof globalThis) => this.#hookHTML(global);
  }
}
