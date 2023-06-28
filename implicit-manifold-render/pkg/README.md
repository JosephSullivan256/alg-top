
## Building

### Native

Run

`cargo run`

and you're good to go.

### For Web

You'll need
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- a web server (I'm using [http-server](https://www.npmjs.com/package/http-server))

First build the project by running

`wasm-pack build --target web --out-name web --out-dir ./pkg`

Then, start the server by running

`npx http-server`

and go to [http://localhost:8080/index.html](http://localhost:8080/index.html).