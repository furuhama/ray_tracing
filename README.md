# Ray Tracing in Rust

This is a ray tracing renderer implemented in Rust.

## Features

- Basic ray tracing with reflection, refraction, and shadow support
- Multiple materials:
  - Lambertian (diffuse) material for matte surfaces
  - Metal material with adjustable fuzziness for reflective surfaces
  - Dielectric material for glass-like surfaces
- Scene composition with multiple objects
- Bounding Volume Hierarchy (BVH) for efficient ray-object intersection
- PPM image output
- Camera with adjustable parameters (position, look-at, field of view)

## Usage

To render the default scene:

```bash
make # Use release build and much faster

# or

cargo run # Use debug build
```

This will generate an `output.ppm` file containing the rendered image.

## Project Structure

- `src/`
  - `main.rs`: Scene setup and rendering pipeline
  - `types.rs`: Common type definitions
  - `ray.rs`: Ray representation
  - `vec3_glam.rs`: Vector operations using glam
  - `camera.rs`: Camera implementation
  - `image.rs`: Image output handling
  - `aabb.rs`: Axis-Aligned Bounding Box implementation
  - `bvh.rs`: Bounding Volume Hierarchy implementation
  - `material/`
    - `lambertian.rs`: Diffuse material
    - `metal.rs`: Metallic material
    - `dielectric.rs`: Glass-like material
  - `object/`
    - `sphere.rs`: Sphere primitive
    - `list.rs`: Object list container

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

3. **Dielectric**
   - Implements glass-like behavior
   - Features:
     - Refraction using Snell's law
     - Fresnel effect
     - Controllable refractive index

### Core Components

- `Vec3`: 3D vector operations (using glam)
- `Ray`: Ray representation and calculations
- `Camera`: Configurable camera system
- `AABB`: Axis-Aligned Bounding Box for optimization
- `BVH`: Bounding Volume Hierarchy for efficient scene traversal
- `Sphere`: Basic geometric primitive
- `HittableList`: Scene object container

## Development

The project is continuously evolving with new features being added. See `TODO.md` for planned improvements.

## License

MIT License
