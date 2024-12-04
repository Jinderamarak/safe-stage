# Electron microscope stage navigation system

System for safe navigation for stage inside electron microscope chamber using 3D model collisions.

# MSRV (Minimum Supported Rust Version)

- `1.83.0` - due to stabilization of floats in `const` context

## Project parts

### `collisions`

- `rust`
- crate handling 3D collisions

### `maths`

- `rust`
- crate for math operations and structures

### `service-app`

- `rust`, `tauri`, `typescript`, `react`
- testing app for 3D visualizations