cd $(dirname $0)
set -e
cargo install wasm-pack
RUSTFLAGS="-C target-cpu=mvp" wasm-pack build -s portal-solutions . -Zbuild-std=std,panic_abort 
sed -i.bak -e 's*"name": "@portal-solutions/portal-solutions-"*"name": "@portal-solutions/"*g' pkg/package.json
mv pkg/package.json pkg/package.json.bak
jq '.dependencies={"@portal-solutions/morphic-wasm-dep-rollup":"^1.0.0"}'  pkg/package.json.bak > pkg/package.json
rm -rf pkg-wasm2js || true
cp -r ./pkg ./pkg-wasm2js
sed -i.bak -e 's*"name": "@portal-solutions/morphic-wasm-web"*"name": "@portal-solutions/morphic-wasm-web.wasm2js"*g' pkg-wasm2js/package.json
rm pkg-wasm2js/portal_solutions_morphic_wasm_web_bg.wasm
wasm2js pkg/portal_solutions_morphic_wasm_web_bg.wasm -o pkg-wasm2js/portal_solutions_morphic_wasm_web_bg.wasm.js
sed -i.bak -e 's*.wasm"*.wasm.js"*g' pkg-wasm2js/package.json
sed -i.bak -e 's*.wasm"*.wasm.js"*g' pkg-wasm2js/portal_solutions_morphic_wasm_web.js
