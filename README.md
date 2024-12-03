# WebTracer: A Rust & WASM Ray tracing Engine
Currently implementing forward ray tracing

## Setting up

### Build CLI for regular use
`cargo build -p raytracer-cli --release`

### Build the WASM binding (in the web-test branch)
```sh
cd raytracer-wasm
wasm-pack build --target web --out-dir ../web-test/pkg --release
cd ../web-test
python -m http.server
cd ..
```
Open http://localhost:8000

## Main Goals
- [x] Learn Rust fundamentals by recreating my [raytracer](https://github.com/reecelikesramen/raytracer) project from CS 4212 Computer Graphics in Rust.
- [x] Separate the CLI and the core library, enable the library to compile to WASM and invoke from the web browser.
- [ ] Optimize the raytracer by parallelizing math operations to increase exposure to low-level optimization.
- [ ] Extend the raytracer with more interesting shaders, shapes, and light types.

## Renders
CLI args for all renders here: `raytracer-cli --width 1000 --height 1000 --rays-per-pixel 3600 --recursion-depth 10`

Simple sphere scene:
![3 spheres upon a plane with 3 lights casting shadows](renders/simple_sphere_scene.png)
NOTE: Scene was created by Dr. Pete Willemsen at University of Minnesota Duluth.

Cornell Room:
![Classic graphics diffuse surface test](renders/cornell_room.png)

Stanford bunny:
![Standard graphics rendering test, 3d scan of a bunny](renders/stanford_bunny.png)
Credit: Dr. Pete Willemsen

[Ray Tracing in One Weekend](https://raytracing.github.io/) Final Render:
![Final render from the online book ray tracing in one weekend](renders/raytracing_one_weekend.png)

Cityscape:
![Boxy skyscrapers and spheres made to look like trees, looks like a city](renders/cityscape.png)
Credit: Dr. Pete Willemsen

1000 spheres:
![10 x 10 x 10 spheres equally sized and spaced with various shaders, lights, and textures](renders/spheres_1K.png)
Credit: Dr. Pete Willemsen

-----

## TODO
 - SceneArgs container for disable_shadows, recursion_depth, image_width, image_height, etc...
 - Background structure for either background_color or env_map
 - Impl scene stuff so its not just public members
 - MTL parsing into shaders