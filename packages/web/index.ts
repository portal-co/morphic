import { type Wasm } from "./wasm.ts";
/*#__NO_SIDE_EFFECTS__*/
export function createInterface(wasm: Wasm): Interface {
  return new Impl(wasm);
}
export interface Interface {
    hookHTML(global: typeof globalThis): void
}
class Impl implements Interface {
  constructor({}: Wasm) {}
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
