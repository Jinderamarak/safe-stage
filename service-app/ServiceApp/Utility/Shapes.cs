using System.Windows.Media;
using System.Windows.Media.Media3D;
using BindingsCs.Safe.Types;

namespace ServiceApp.Utility;

public static class Shapes
{
    public static MeshGeometry3D CreateCubeGeometry(double a, double b, double c)
    {
        var mesh = new MeshGeometry3D();

        mesh.Positions.Add(new Point3D(-a / 2, -b / 2, -c / 2));
        mesh.Positions.Add(new Point3D(a / 2, -b / 2, -c / 2));
        mesh.Positions.Add(new Point3D(a / 2, b / 2, -c / 2));
        mesh.Positions.Add(new Point3D(-a / 2, b / 2, -c / 2));
        mesh.Positions.Add(new Point3D(-a / 2, -b / 2, c / 2));
        mesh.Positions.Add(new Point3D(a / 2, -b / 2, c / 2));
        mesh.Positions.Add(new Point3D(a / 2, b / 2, c / 2));
        mesh.Positions.Add(new Point3D(-a / 2, b / 2, c / 2));

        // Define the triangle indices for the 6 faces of the cube
        var indices = new int[]
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

        // Add triangle indices
        foreach (var index in indices)
            mesh.TriangleIndices.Add(index);

        return mesh;
    }

    /// <summary>
    /// Generates a sphere geometry as MeshGeometry3D.
    /// </summary>
    /// <param name="radius">The radius of the sphere.</param>
    /// <param name="segments">The number of horizontal segments (latitude).</param>
    /// <param name="rings">The number of vertical rings (longitude).</param>
    /// <returns>A MeshGeometry3D representing the sphere.</returns>
    public static MeshGeometry3D CreateSphereGeometry(double radius, int segments = 32, int rings = 16)
    {
        var mesh = new MeshGeometry3D();

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

                var normal = new Vector3D(x, y, z);
                var position = normal * radius;

                mesh.Positions.Add(new Point3D(position.X, position.Y, position.Z));
            }
        }

        // Generate triangle indices
        for (var ring = 0; ring < rings; ring++)
        for (var segment = 0; segment < segments; segment++)
        {
            var current = ring * (segments + 1) + segment;
            var next = current + segments + 1;

            // Triangle 1
            mesh.TriangleIndices.Add(current + 1);
            mesh.TriangleIndices.Add(next);
            mesh.TriangleIndices.Add(current);

            // Triangle 2
            mesh.TriangleIndices.Add(next + 1);
            mesh.TriangleIndices.Add(next);
            mesh.TriangleIndices.Add(current + 1);
        }

        return mesh;
    }

    /// <summary>
    /// Creates a three-sided prism (triangular cross-section) between two points in 3D space.
    /// </summary>
    /// <param name="pointA">The starting point of the line.</param>
    /// <param name="pointB">The ending point of the line.</param>
    /// <param name="thickness">The thickness of the prism.</param>
    /// <returns>A MeshGeometry3D representing the three-sided prism.</returns>
    public static MeshGeometry3D CreateLineAsPrismGeometry(Point3D pointA, Point3D pointB, double thickness)
    {
        var mesh = new MeshGeometry3D();

        // Direction vector from A to B
        var direction = pointB - pointA;
        direction.Normalize();

        // Perpendicular vectors for the triangular cross-section
        var up = new Vector3D(0, 1, 0);
        if (Vector3D.CrossProduct(direction, up).LengthSquared <
            1e-6) up = new Vector3D(1, 0, 0); // Choose a different up vector if parallel
        var side1 = Vector3D.CrossProduct(direction, up);
        side1.Normalize();
        side1 *= thickness / 2;

        var side2 = Vector3D.CrossProduct(direction, side1);
        side2.Normalize();
        side2 *= thickness / 2;

        // Define the 6 vertices of the prism (3 at each end)
        var p1 = pointA + side1;
        var p2 = pointA - side1 + side2;
        var p3 = pointA - side1 - side2;

        var p4 = pointB + side1;
        var p5 = pointB - side1 + side2;
        var p6 = pointB - side1 - side2;

        // Add vertices to the mesh
        mesh.Positions.Add(p1);
        mesh.Positions.Add(p2);
        mesh.Positions.Add(p3);
        mesh.Positions.Add(p4);
        mesh.Positions.Add(p5);
        mesh.Positions.Add(p6);

        // Define triangles for the three sides of the prism
        // Side 1
        mesh.TriangleIndices.Add(0);
        mesh.TriangleIndices.Add(1);
        mesh.TriangleIndices.Add(4);

        mesh.TriangleIndices.Add(0);
        mesh.TriangleIndices.Add(4);
        mesh.TriangleIndices.Add(3);

        // Side 2
        mesh.TriangleIndices.Add(1);
        mesh.TriangleIndices.Add(2);
        mesh.TriangleIndices.Add(5);

        mesh.TriangleIndices.Add(1);
        mesh.TriangleIndices.Add(5);
        mesh.TriangleIndices.Add(4);

        // Side 3
        mesh.TriangleIndices.Add(2);
        mesh.TriangleIndices.Add(0);
        mesh.TriangleIndices.Add(3);

        mesh.TriangleIndices.Add(2);
        mesh.TriangleIndices.Add(3);
        mesh.TriangleIndices.Add(5);

        // Optionally compute normals for lighting (not strictly necessary)
        // Note: Normals can be averaged per vertex or computed per face
        ComputeNormals(mesh);

        return mesh;
    }

    /// <summary>
    /// Computes normals for a mesh (optional helper for lighting).
    /// </summary>
    /// <param name="mesh">The MeshGeometry3D to compute normals for.</param>
    private static void ComputeNormals(MeshGeometry3D mesh)
    {
        var normals = new Vector3D[mesh.Positions.Count];
        for (var i = 0; i < mesh.TriangleIndices.Count; i += 3)
        {
            var index0 = mesh.TriangleIndices[i];
            var index1 = mesh.TriangleIndices[i + 1];
            var index2 = mesh.TriangleIndices[i + 2];

            var p0 = mesh.Positions[index0];
            var p1 = mesh.Positions[index1];
            var p2 = mesh.Positions[index2];

            var normal = Vector3D.CrossProduct(p1 - p0, p2 - p0);
            normal.Normalize();

            normals[index0] += normal;
            normals[index1] += normal;
            normals[index2] += normal;
        }

        for (var i = 0; i < normals.Length; i++)
        {
            normals[i].Normalize();
            mesh.Normals.Add(normals[i]);
        }
    }

    public static IEnumerable<GeometryModel3D> CreatePathGeometries(List<SixAxis> nodes)
    {
        var nodeGeometry = CreateCubeGeometry(3e-3, 3e-3, 3e-3);

        for (var i = 0; i < nodes.Count; i++)
        {
            var node = nodes[i];
            var position = new Point3D(node.X, node.Y, node.Z);
            var translate = new TranslateTransform3D(position.X, position.Y, position.Z);
            var rotate = Maths.RotateTransformFromEulerAngles(node.Rx, node.Ry, node.Rz);

            var transform = new Transform3DGroup();
            transform.Children.Add(rotate);
            transform.Children.Add(translate);

            var nodeModel = new GeometryModel3D(nodeGeometry, Materials.PathNode)
            {
                Transform = transform
            };
            yield return nodeModel;

            if (i < nodes.Count - 1)
            {
                var next = nodes[i + 1];
                var line = CreateLineAsPrismGeometry(position, new Point3D(next.X, next.Y, next.Z),
                    1e-3);
                var lineModel = new GeometryModel3D(line, Materials.PathEdge);
                yield return lineModel;
            }
        }
    }

    public static List<Point3D> TrianglesToPointsList(TriangleBuffer buffer)
    {
        return buffer.Buffer.Select(p => new Point3D(p.X, p.Y, p.Z)).ToList();
    }
}