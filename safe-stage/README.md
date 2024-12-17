# Safe Stage

The codebase is separated into the following crates:

- `bindings` - binary crate - run it to regenerate bindings
- `collisions` - handling of collision detection - triangles, bvhs
- `maths` - small math library - vectors, quaternions
- `models` - representing microscope - stage, chamber, retract, etc.
- `path` - pathfinding algorithms and path resolvers

# Testing

- `cargo test --all`
- `cargo bench --all`
- `cargo miri test -F ffi`
