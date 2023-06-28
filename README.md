# implicit-manifold

Tool for computing topological invariants of level sets of submersions, i.e., implicit manifolds.

[Check it out!](https://josephsullivan256.github.io/implicit-manifold/implicit-manifold-render/)

## Goals

- Compute (approximate) triangulation of submanifolds in Euclidean space which are level sets of submersion
    - Following approach in [this paper](https://doi.org/10.1016/S0021-9991(03)00275-4)
    - Prove some correctness result when submersion is Lipschitz
- Project/slice triangulation to 3D and render using [three-d](https://github.com/asny/three-d)
- Interpret triangulation as a delta complex, compute the (co)homology groups and the cup product structure over Z or a field.
    - The general theory is explained in Hatcher's [_Algberaic Topology_](http://pi.math.cornell.edu/~hatcher/AT/AT+.pdf), but computationally this will be done by using Smith normal forms, as explained for homology [here](https://www.matem.unam.mx/~omar/mathX27/smith-form.html).
- Create a Linear/Abstract Algebra library to handle relevant computations discussed above.
- Have an editor for building submersions then compute triangulations/homology.

## Organization

There are two cargo packages: `implicit-manifold-compute` and `implicit-manifold-render`. The former comes up with the triangulation and computes the topological invariants, the latter is a demo that renders the triangulations/allows you to play with implicit surfaces, seeing the real time changes in the topology.

## Progress

There's quite a bit left to do.

- [ ] Linear algebra 
    - [X] basic matrix operations (over integers/floats)
    - [ ] Smith Normal Form
- [ ] Marching simplices
    - [X] get Kuhn triangulation
    - [ ] intersect Kuhn triangulation with hyperplane approximations
- [ ] Topology
    - [ ] collect data from marching simplices to a delta complex
    - [ ] compute simplicial homology using Smith Normal Form
- [ ] Visualization
    - [X] adapt three-d wireframe example to test out Kuhn triangulation
    - [ ] create UI for inputting functions to compute implicit surfaces

This is my basic outline/TODO list, but more things will inevitably arise as I start checking things off.

## License

Licensed under the MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT).