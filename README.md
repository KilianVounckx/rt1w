# rt1w

A simple ray tracer based on the bookseries [_Ray Tracing in One Weekend_](https://raytracing.github.io/).
Some resulting images can be found in the [outputs](outputs/) folder.
Examples can be found in the [scenes](src/scenes/) folder.

### Shapes

For now, the ray tracer supports the following shapes:
- Spheres
- Axis alligned rectangles
- Boxes (named cubes in the code, because box is a reserved keyword in rust)
- Translations of other shapes
- Rotations around the y-axis of other shapes
- Constant Mediums in any other shape (like mist)
- Shape lists (Sometimes called groups in other languages)
- Bvh Nodes (Bounding volume hierarchy)

### Materials

The ray tracer has the following materials:
- Lambertian: Scatters the ray in a random direction in the hemisphere of the surface normal.
- Metal: Scatters the ray reflected along the surface normal. Allows for a fuzz parameter which perturbs the reflection in a random direction.
- Dielectric: Scatters the ray refracted with respect to the surface normal, with a custom index of refraction.
- Diffuse Light: Does not scatter and just returns the color of the light.
- Isotropic: Scatters the ray in a random direcion. Mostly used with Constant Mediums.

### Textures

The following textures are implemented:
- Solid Color: The simplest, consisting of a single color.
- Checkers: a checkers pattern consisting of two other patterns.
- Perlin: a marble-like grayscale pattern which.
- Image: maps a picture on a shape with uv-mapping.
