# alg-top

Tool for computing topological invariants of level sets of submersions.

[Check it out!](https://josephsullivan256.github.io/alg-top/)

## Goals

- Compute (approximate) triangulation of submanifolds in Euclidean space which are level sets of submersion
    - Following approach in [this paper](https://doi.org/10.1016/S0021-9991(03)00275-4)
    - Prove some correctness result when submersion is Lipschitz
- Project/slice triangulation to 3D and render using [three-d](https://github.com/asny/three-d)
- Interpret triangulation as a delta complex, compute the (co)homology groups and the cup product structure over Z or a field.
    - The general theory is explained in Hatcher's [_Algberaic Topology_](http://pi.math.cornell.edu/~hatcher/AT/AT+.pdf), but computationally this will be done by using Smith normal forms, as explained for homology [here](https://www.matem.unam.mx/~omar/mathX27/smith-form.html).
- Create a Linear/Abstract Algebra library to handle relevant computations discussed above.
- Have an editor for building submersions then compute triangulations/homology.

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

## License

Licensed under the MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).