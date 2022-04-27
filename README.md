# Ray Tracer Rust
Minimalist ray tracer based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

## How to run
Modify file `world.txt` for the world definition. The file consists of a set of lines
defining a set of spheres separated by the empty line. Each sphere entry consist of the following lines
1. Radius
2. Center coordinate in the form (x, y, z)
3. Enumeration of the object material
4. Albedo color of the material in the range 0.0 <= x <= 1.0

For example
```txt
0.5
0.0 0.0 -1.0
2
0.8 0.8 0.8 0.3
```
implies the sphere of radius `0.5`, centered at `0.0 0.0 -1.0`, 
material of type `metalic`, albedo of `256*rgb(0.8 0.8 0.8)` 
and the fuzziness parameter of `0.3`

Run the project using cargo
```shell
cargo run > out/image.ppm
```
Inspect the resulting image in `out/image.ppm`

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