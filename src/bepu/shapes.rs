use super::handles::*;
use super::interop_math::*;
use super::tree::*;
use super::utilities::*;

/// Shape type enum.
#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum ShapeTypes {
    Sphere = 0,
    Capsule = 1,
    Box = 2,
    Triangle = 3,
    Cylinder = 4,
    ConvexHull = 5,
    Compound = 6,
    BigCompound = 7,
    Mesh = 8,
}

/// A sphere shape.
#[repr(C)]
pub struct Sphere {
    /// Radius of the sphere.
    pub radius: f32,
}

/// A capsule shape.
#[repr(C)]
pub struct Capsule {
    /// Spherical expansion applied to the internal line segment.
    pub radius: f32,
    /// Half of the length of the internal line segment. Oriented along the local Y axis.
    pub half_length: f32,
}

/// A box shape.
#[repr(C)]
pub struct Box {
    /// Half of the box's width along its local X axis.
    pub half_width: f32,
    /// Half of the box's height along its local Y axis.
    pub half_height: f32,
    /// Half of the box's length along its local Z axis.
    pub half_length: f32,
}

impl Box {
    pub fn new(width: f32, height: f32, length: f32) -> Self {
        Self {
            half_width: width * 0.5,
            half_height: height * 0.5,
            half_length: length * 0.5,
        }
    }
}

/// A triangle shape.
#[repr(C)]
pub struct Triangle {
    /// First vertex of the triangle in local space.
    pub a: Vector3,
    /// Second vertex of the triangle in local space.
    pub b: Vector3,
    /// Third vertex of the triangle in local space.
    pub c: Vector3,
}

/// A cylinder shape.
#[repr(C)]
pub struct Cylinder {
    /// Radius of the cylinder.
    pub radius: f32,
    /// Half length of the cylinder along its local Y axis.
    pub half_length: f32,
}

#[repr(C)]
pub struct HullVertexIndex {
    /// Bundle index of the vertex.
    pub bundle_index: u16,
    /// Inner index of the vertex within its bundle.
    pub inner_index: u16,
}

/// Dummy type standing in for the compile time variable width `Vector3Wide` type.
/// Pointers to buffers of this type should be reinterpreted to either `Vector3SIMD128` or `Vector3SIMD256` depending on what SIMD width is in use.
#[repr(C)]
pub struct Vector3Wide;

/// Dummy type standing in for the compile time variable width `HullBoundingPlanes` type.
/// Pointers to buffers of this type should be reinterpreted to either `HullBoundingPlanesSIMD128` or `HullBoundingPlanesSIMD256` depending on what SIMD width is in use.
#[repr(C)]
pub struct HullBoundingPlanes;

#[repr(C)]
pub struct HullBoundingPlanesSIMD128 {
    pub normal: Vector3SIMD128,
    pub offset: Vector128F,
}

#[repr(C)]
pub struct HullBoundingPlanesSIMD256 {
    pub normal: Vector3SIMD256,
    pub offset: Vector256F,
}

/// A convex hull shape.
#[repr(C)]
pub struct ConvexHull {
    /// Bundled points of the convex hull.
    pub points: Buffer<Vector3Wide>,
    /// Bundled bounding planes associated with the convex hull's faces.
    pub bounding_planes: Buffer<HullBoundingPlanes>,
    /// Combined set of vertices used by each face. Use `face_to_vertex_indices_start` to index into this for a particular face. Indices stored in counterclockwise winding in right handed space, clockwise in left handed space.
    pub face_vertex_indices: Buffer<HullVertexIndex>,
    /// Start indices of faces in the `face_vertex_indices`.
    pub face_to_vertex_indices_start: Buffer<i32>,
}

/// Shape and pose of a child within a compound shape.
#[repr(C)]
pub struct CompoundChild {
    /// Local orientation of the child in the compound.
    pub local_orientation: Quaternion,
    /// Local position of the child in the compound.
    pub local_position: Vector3,
    /// Index of the shape within whatever shape collection holds the compound's child shape data.
    pub shape_index: TypedIndex,
}

/// Minimalist compound shape containing a list of child shapes. Does not make use of any internal acceleration structure; should be used only with small groups of shapes.
#[repr(C)]
pub struct Compound {
    /// Buffer of children within this compound.
    pub children: Buffer<CompoundChild>,
}

/// Compound shape containing a bunch of shapes accessible through a tree acceleration structure. Useful for compounds with lots of children.
#[repr(C)]
pub struct BigCompound {
    /// Acceleration structure for the compound children.
    pub tree: Tree,
    /// Buffer of children within this compound.
    pub children: Buffer<CompoundChild>,
}

/// A mesh shape.
#[repr(C)]
pub struct Mesh {
    /// Acceleration structure of the mesh.
    pub tree: Tree,
    /// Buffer of triangles composing the mesh. Triangles will only collide with tests which see the triangle as wound clockwise in right handed coordinates or counterclockwise in left handed coordinates.
    pub triangles: Buffer<Triangle>,
    pub scale: Vector3,
    pub inverse_scale: Vector3,
}

impl Mesh {
    pub fn set_scale(&mut self, scale: Vector3) {
        self.inverse_scale.x = if scale.x != 0.0 {
            1.0 / scale.x
        } else {
            f32::MAX
        };
        self.inverse_scale.y = if scale.y != 0.0 {
            1.0 / scale.y
        } else {
            f32::MAX
        };
        self.inverse_scale.z = if scale.z != 0.0 {
            1.0 / scale.z
        } else {
            f32::MAX
        };
    }
}
