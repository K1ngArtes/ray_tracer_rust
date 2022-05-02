# Ray Tracer Rust
Minimalist ray tracer based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

## How to run
Run the project using cargo
```shell
cargo run > out/image.ppm
```
Inspect the resulting image in `out/image.ppm`

Add flag `random` to generate random scene
```shell
cargo run > out/image.ppm random
```

## Configuration
Modify file `world.txt` for the world definition. The file consists of a set of lines
defining a set of spheres separated by the empty line. Each sphere entry consist of the following lines
1. Comment
1. Radius
1. Center coordinate in the form (x, y, z)
1. Enumeration of the object material
1. Albedo color of the material in the range 0.0 <= x <= 1.0 + type specific param

For example
```txt
// Center sphere. Grey color
0.5
0.0 0.0 -1.0
2
0.8 0.8 0.8 0.3
```
implies the sphere of radius `0.5`, centered at `0.0 0.0 -1.0`, 
material of type `metalic`, albedo of `256*rgb(0.8 0.8 0.8)` 
and the fuzziness parameter of `0.3`

## Material specific parameters
### Metal
#### Fuzziness
Defines how frizzy/vague the reflections would be. The higher the fuzziness, the less vague the 
reflection would be. Ranges from 0.0 (no fuzziness, perfect mirror) to 1.0

Example value: `0.8 0.6 0.2 0.0`

### Glass
#### Index of refraction
Glass doesn't have the albedo value. Index of refraction for air is `1.0`, water `1.3` and diamond is `2.4`

Example value: `1.5`

## Benchmarking
Benchmarking requires a [nightly build](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) which can be installed 
and ran using the following commands
```shell
rustup toolchain install nightly
rustup toolchain list
rustup override set nightly
cargo bench
```

## Example render
![Alt text](example_images/example_image.png?raw=true "Title")
![Alt text](example_images/glass_diffuse_metal.png?raw=true "Glass, diffuse and metal spheres")