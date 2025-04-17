using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using BindingsCs.Safe.Types;
using ServiceApp.View3D.Controls;
using Vector3 = System.Numerics.Vector3;

namespace ServiceApp.Avalonia.Utility;

public static class Shapes
{
    /// <summary>
    /// Generates a cube geometry as MeshGeometry3D.
    /// </summary>
    /// <param name="a">The size of the first edge.</param>
    /// <param name="b">The size of the second edge.</param>
    /// <param name="c">The size of the third edge.</param>
    /// <returns>A Vector3 enumeration representing the cube.</returns>
    public static IEnumerable<Vector3> CreateCubeGeometry(float a, float b, float c)
    {
        var vertices = new[]
        {
            new Vector3(-a / 2, -b / 2, -c / 2),
            new Vector3(a / 2, -b / 2, -c / 2),
            new Vector3(a / 2, b / 2, -c / 2),
            new Vector3(-a / 2, b / 2, -c / 2),
            new Vector3(-a / 2, -b / 2, c / 2),
            new Vector3(a / 2, -b / 2, c / 2),
            new Vector3(a / 2, b / 2, c / 2),
            new Vector3(-a / 2, b / 2, c / 2)
        };

        // Define the triangle indices for the 6 faces of the cube
        var indices = new[]
        {
            // Front face
            0, 2, 1, 0, 3, 2,
            // Back face
            4, 5, 6, 4, 6, 7,
            // Left face
            0, 7, 3, 0, 4, 7,
            // Right face
            1, 2, 6, 1, 6, 5,
            // Top face
            3, 7, 6, 3, 6, 2,
            // Bottom face
            0, 1, 5, 0, 5, 4
        };

        return IndexedMesh(vertices, indices);
    }

    /// <summary>
    /// Generates a sphere geometry as MeshGeometry3D.
    /// </summary>
    /// <param name="radius">The radius of the sphere.</param>
    /// <param name="segments">The number of horizontal segments (latitude).</param>
    /// <param name="rings">The number of vertical rings (longitude).</param>
    /// <returns>A Vector3 enumeration representing the sphere.</returns>
    public static IEnumerable<Vector3> CreateSphereGeometry(float radius, int segments = 32, int rings = 16)
    {
        var vertices = new List<Vector3>(rings * segments);

        // Generate vertices and normals
        for (var ring = 0; ring <= rings; ring++)
        {
            var phi = Math.PI * ring / rings; // Latitude angle
            var y = Math.Cos(phi); // Y-coordinate (height)
            var sinPhi = Math.Sin(phi); // Sine of latitude angle

            for (var segment = 0; segment <= segments; segment++)
            {
                var theta = 2 * Math.PI * segment / segments; // Longitude angle
                var x = Math.Cos(theta) * sinPhi; // X-coordinate
                var z = Math.Sin(theta) * sinPhi; // Z-coordinate

                var normal = new Vector3((float)x, (float)y, (float)z);
                var position = normal * radius;

                vertices.Add(new Vector3(position.X, position.Y, position.Z));
            }
        }

        // Generate triangle indices
        for (var ring = 0; ring < rings; ring++)
        for (var segment = 0; segment < segments; segment++)
        {
            var current = ring * (segments + 1) + segment;
            var next = current + segments + 1;

            yield return vertices[current + 1];
            yield return vertices[next];
            yield return vertices[current];

            yield return vertices[next + 1];
            yield return vertices[next];
            yield return vertices[current + 1];
        }
    }

    /// <summary>
    /// Creates a three-sided prism (triangular cross-section) between two points in 3D space.
    /// </summary>
    /// <param name="pointA">The starting point of the line.</param>
    /// <param name="pointB">The ending point of the line.</param>
    /// <param name="thickness">The thickness of the prism.</param>
    /// <returns>A Vector3 enumeration representing the three-sided prism.</returns>
    public static IEnumerable<Vector3> CreateLineAsPrismGeometry(Vector3 pointA, Vector3 pointB, float thickness)
    {
        // Direction vector from A to B
        var direction = Vector3.Normalize(pointB - pointA);

        // Perpendicular vectors for the triangular cross-section
        var up = Vector3.UnitY;
        if (Vector3.Cross(direction, up).LengthSquared() < 1e-6)
            up = Vector3.UnitX; // Choose a different up vector if parallel

        var side1 = Vector3.Normalize(Vector3.Cross(direction, up));
        side1 *= thickness / 2;

        var side2 = Vector3.Normalize(Vector3.Cross(direction, side1));
        side2 *= thickness / 2;

        // Define the 6 vertices of the prism (3 at each end)
        var p1 = pointA + side1;
        var p2 = pointA - side1 + side2;
        var p3 = pointA - side1 - side2;

        var p4 = pointB + side1;
        var p5 = pointB - side1 + side2;
        var p6 = pointB - side1 - side2;

        // Add vertices to the mesh
        var vertices = new[]
        {
            pointA + side1,
            pointA - side1 + side2,
            pointA - side1 - side2,
            pointB + side1,
            pointB - side1 + side2,
            pointB - side1 - side2
        };

        // Define triangles for the three sides of the prism
        var indices = new[]
        {
            0, 1, 4, 0, 4, 3,
            1, 2, 5, 1, 5, 4,
            2, 0, 3, 2, 3, 5
        };

        return IndexedMesh(vertices, indices);
    }

    /// <summary>
    /// Creates a geometry for a path defined by a list of nodes.
    /// </summary>
    /// <param name="nodes">The list of nodes defining the path.</param>
    /// <returns>An enumerable of geometry models representing the path.</returns>
    public static IEnumerable<GeometryModel> CreatePathGeometries(List<SixAxis> nodes)
    {
        var nodeGeometry = CreateCubeGeometry(3e-3f, 3e-3f, 3e-3f).ToList();
        for (var i = 0; i < nodes.Count; i++)
        {
            var node = nodes[i];

            var transformed = TransformedMesh(nodeGeometry, node);
            yield return new GeometryModel
            {
                Color = Materials.PathNode,
                Vertices = transformed
            };

            if (i < nodes.Count - 1)
            {
                var from = new Vector3((float)node.X, (float)node.Y, (float)node.Z);

                var next = nodes[i + 1];
                var to = new Vector3((float)next.X, (float)next.Y, (float)next.Z);

                var line = CreateLineAsPrismGeometry(from, to, 1e-3f);
                yield return new GeometryModel
                {
                    Color = Materials.PathEdge,
                    Vertices = line
                };
            }
        }
    }

    public static IEnumerable<Vector3> TrianglesToPointsList(TriangleBuffer buffer)
    {
        return buffer.Buffer.Select(p => new Vector3((float)p.X, (float)p.Y, (float)p.Z));
    }

    private static IEnumerable<Vector3> IndexedMesh(Vector3[] vertices, int[] indices)
    {
        foreach (var index in indices)
            yield return vertices[index];
    }

    private static IEnumerable<Vector3> TransformedMesh(IEnumerable<Vector3> vertices, SixAxis transform)
    {
        var translation = Matrix4x4.CreateTranslation((float)transform.X, (float)transform.Y, (float)transform.Z);
        var rotation = Matrix4x4.CreateFromYawPitchRoll((float)transform.Rx, (float)transform.Ry, (float)transform.Rz);

        foreach (var vector3 in vertices)
            yield return Vector3.Transform(Vector3.Transform(vector3, rotation), translation);
    }
}