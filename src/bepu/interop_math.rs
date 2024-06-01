/// A 3D vector.
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self::zero()
    }
}

/// A quaternion.
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

/// A 128-bit vector of 4 floats.
#[repr(C)]
pub struct Vector128F {
    pub v0: f32,
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
}

/// A 256-bit vector of 8 floats.
#[repr(C)]
pub struct Vector256F {
    pub v0: f32,
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
    pub v4: f32,
    pub v5: f32,
    pub v6: f32,
    pub v7: f32,
}

/// A 128-bit vector of 4 ints.
#[repr(C)]
pub struct Vector128I {
    pub v0: i32,
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
}

/// A 256-bit vector of 8 ints.
#[repr(C)]
pub struct Vector256I {
    pub v0: i32,
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
    pub v4: i32,
    pub v5: i32,
    pub v6: i32,
    pub v7: i32,
}

/// Represents a rigid transformation.
#[repr(C)]
pub struct RigidPose {
    /// Orientation of the pose.
    pub orientation: Quaternion,
    /// Position of the pose.
    pub position: Vector3,
    pub pad: i32,
}

impl RigidPose {
    pub fn new(position: Vector3, orientation: Quaternion) -> Self {
        Self {
            position,
            orientation,
            pad: 0,
        }
    }

    pub fn from_position(position: Vector3) -> Self {
        Self::new(position, Quaternion::identity())
    }

    pub fn identity() -> Self {
        Self::from_position(Vector3::zero())
    }
}

/// Lower left triangle (including diagonal) of a symmetric 3x3 matrix.
#[repr(C)]
#[derive(Default)]
pub struct Symmetric3x3 {
    /// First row, first column of the matrix.
    pub xx: f32,
    /// Second row, first column of the matrix.
    pub yx: f32,
    /// Second row, second column of the matrix.
    pub yy: f32,
    /// Third row, first column of the matrix.
    pub zx: f32,
    /// Third row, second column of the matrix.
    pub zy: f32,
    /// Third row, third column of the matrix.
    pub zz: f32,
}

impl Symmetric3x3 {
    /// Creates a new zeroed `Symmetric3x3`.
    pub fn zero() -> Self {
        Self {
            xx: 0.0,
            yx: 0.0,
            yy: 0.0,
            zx: 0.0,
            zy: 0.0,
            zz: 0.0,
        }
    }
}

/// Vector3Wide interop type used when `Vector<float>` is 128 bits wide.
#[repr(C)]
pub struct Vector3SIMD128 {
    pub x: Vector128F,
    pub y: Vector128F,
    pub z: Vector128F,
}

/// Vector3Wide interop type used when `Vector<float>` is 256 bits wide.
#[repr(C)]
pub struct Vector3SIMD256 {
    pub x: Vector256F,
    pub y: Vector256F,
    pub z: Vector256F,
}

/// QuaternionWide interop type used when `Vector<float>` is 128 bits wide.
#[repr(C)]
pub struct QuaternionSIMD128 {
    pub x: Vector128F,
    pub y: Vector128F,
    pub z: Vector128F,
    pub w: Vector128F,
}

/// QuaternionWide interop type used when `Vector<float>` is 256 bits wide.
#[repr(C)]
pub struct QuaternionSIMD256 {
    pub x: Vector256F,
    pub y: Vector256F,
    pub z: Vector256F,
    pub w: Vector256F,
}

/// BodyInertiaWide interop type used when `Vector<float>` is 128 bits wide.
#[repr(C)]
pub struct BodyInertiaSIMD128 {
    pub inverse_inertia_xx: Vector128F,
    pub inverse_inertia_yx: Vector128F,
    pub inverse_inertia_yy: Vector128F,
    pub inverse_inertia_zx: Vector128F,
    pub inverse_inertia_zy: Vector128F,
    pub inverse_inertia_zz: Vector128F,
    pub inverse_mass: Vector128F,
}

/// BodyInertiaWide interop type used when `Vector<float>` is 256 bits wide.
#[repr(C)]
pub struct BodyInertiaSIMD256 {
    pub inverse_inertia_xx: Vector256F,
    pub inverse_inertia_yx: Vector256F,
    pub inverse_inertia_yy: Vector256F,
    pub inverse_inertia_zx: Vector256F,
    pub inverse_inertia_zy: Vector256F,
    pub inverse_inertia_zz: Vector256F,
    pub inverse_mass: Vector256F,
}

/// BodyVelocityWide interop type used when `Vector<float>` is 128 bits wide.
#[repr(C)]
pub struct BodyVelocitySIMD128 {
    pub linear: Vector3SIMD128,
    pub angular: Vector3SIMD128,
}

/// BodyVelocityWide interop type used when `Vector<float>` is 256 bits wide.
#[repr(C)]
pub struct BodyVelocitySIMD256 {
    pub linear: Vector3SIMD256,
    pub angular: Vector3SIMD256,
}
