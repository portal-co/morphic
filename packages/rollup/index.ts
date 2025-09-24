export class DOMTag<T extends WeakKey = any> {
  #wrapped: WeakRef<T>;
  constructor(val: T) {
    this.#wrapped = new WeakRef(val);
  }
  get value() {
    return this.#wrapped.deref();
  }
}
