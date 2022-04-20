# Ray Tracer Rust
Minimalist ray tracer based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

## How to run
Modify file `world.txt` for the world definition. The file consists of a set of lines
where every two consecutive define a sphere. The first line defines the sphere radius,
while the second line specifies `x, y, z` coordinates of the sphere center. For example
```txt
0.5
0.0 0.0 -1.0
```
implies the sphere of radius `0.5` and centered at `0.0 0.0 -1.0`

Run the project using cargo
```shell
cargo run
```
Inspect the resulting image in `out/image.ppm`