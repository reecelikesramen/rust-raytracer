# Raytracer

## Setting up

### Build CLI
`cargo build -p raytracer-cli --release`

### Build only the library crate for WASM by running
`cargo build -p raytracer-lib --target wasm32-unknown-unknown`

## Main Goals
- Recreate my [raytracer](https://github.com/reecelikesramen/raytracer) project from CS 4212 Computer Graphics in Rust.
- Separate the CLI and the core library, enable the library to compile to WASM and invoke from the web browser.
- Optimize the raytracer with SIMD and CUDA (separate versions) to increase exposure to low-level parallelization.
- Extend the raytracer with more interesting shaders, shapes, and light types.

## Examples
Note: the following scenes were created by Dr. Pete Willemsen at University of Minnesota Duluth.

Simple sphere scene:
![3 spheres upon a plane with 3 lights casting shadows](renders/simple_sphere_scene.png)

Spheres and triangles:
![4 spheres and 4 triangles arranged to make an intriguing pattern](renders/spheres_and_triangles.png)

Cornell Room:
![A cube room with a short box and six mirrored spheres inside](renders/cornell_room.png)

1000 spheres:
![10 x 10 x 10 spheres equally spaced with various shaders and lights](renders/spheres_1K.png)

Cityscape:
![Boxy skyscrapers and spheres made to look like trees make a city](renders/box_sphere_test.png)