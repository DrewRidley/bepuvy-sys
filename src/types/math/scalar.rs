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

#[repr(C)]
pub struct BodyInertia {
    inverse_tensor: Symmetric3x3,
    inverse_mass: f32,
    // Required for consistent size with C# NativeAOT ABI.
    _pad: i32,
}

#[repr(C)]
pub struct BodyVelocity {
    linear: Vector3,
    _pad0: i32,
    angular: Vector3,
    _pad1: i32,
}
