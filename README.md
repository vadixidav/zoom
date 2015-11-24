# zoom
A library for Rust particle physics

The goal of this library is to handle all particle interactions needed in a particle system in a generic way. Traits will be created for particles that allow them to be stored into octrees and quadtrees. This library will also provide Barnes-Hut trees for efficient simulations of charged or gravitational particle simulations. Eventually, support for GPU acceleration will be added, but this may require an API change.

Once Rust adds parameterized numbers, major version 2 of the API will be specified so that a generic amount of dimensions may be specified in the API and still generate static code. Until then, separate traits and structs will be provided for 1d, 2d, and 3d particle physics simulations.
