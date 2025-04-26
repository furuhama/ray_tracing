# Ray Tracing in Rust

This is a ray tracing renderer implemented in Rust, based on "Ray Tracing in One Weekend" series.

## Features

- Basic ray tracing with reflection and shadow support
- Multiple materials:
  - Lambertian (diffuse) material for matte surfaces
  - Metal material with adjustable fuzziness for reflective surfaces
- Scene composition with multiple objects
- PPM image output

## Usage

To render the default scene:

```bash
cargo run
```

This will generate an `output.ppm` file containing the rendered image.

## Project Structure

- `src/lib.rs`: Core ray tracing implementation
  - Vector operations
  - Ray and intersection calculations
  - Material system
  - Scene objects
- `src/main.rs`: Scene setup and rendering pipeline

## Scene Configuration

The current scene includes:
- A large ground sphere with yellow diffuse material
- A center sphere with reddish diffuse material
- A left sphere with reflective metal material (low fuzz)
- A right sphere with reflective metal material (high fuzz)

## Implementation Details

### Materials

1. **Lambertian (Diffuse)**
   - Implements ideal diffuse reflection
   - Controlled by albedo (color absorption)

2. **Metal**
   - Implements specular reflection
   - Controlled by:
     - albedo (color absorption)
     - fuzz (surface roughness)

### Core Components

- `Vec3`: 3D vector operations
- `Ray`: Ray representation and calculations
- `Hittable`: Trait for objects that can be hit by rays
- `Material`: Trait for surface materials
- `Sphere`: Basic geometric primitive
- `HittableList`: Scene object container

## Development

The project is continuously evolving with new features being added. See `TODO.md` for planned improvements.

## License

MIT License
