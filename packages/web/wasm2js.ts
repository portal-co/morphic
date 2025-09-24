import * as wasm from "@portal-solutions/morphic-wasm-web.wasm2js"
import { createInterface } from "./index.ts";
// export type Wasm = typeof wasm;
export default /*#__PURE__*/createInterface(wasm);