

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 {x, y, z}
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion{ 
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Quaternion {x, y, z, w}
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RigidPose {
    pub orientation: Quaternion,
    pub position: Vector3,
}

extern "C" {
    pub fn Timestep();

    pub fn SpawnCube(pos: Vector3) -> i32;

    pub fn GetBodyPose(handle: i32) -> RigidPose;
    pub fn SetupPyramidDemo();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_setup() {
        unsafe {
            SetupPyramidDemo();
        }
    }

    #[test]
    fn test_add_cube() {
        unsafe {
            SetupPyramidDemo();

            let handle = SpawnCube(Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });

            let pose = GetBodyPose(handle);
        }
    }
}