

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
    pub position: Vector3,
    pub orientation: Quaternion,
}


pub fn setup_pyramid() {
    unsafe {
        SetupPyramidDemo();
    };
}

pub fn timestep() {
    unsafe {
        Timestep();
    };
}

pub fn spawn_cube(pos: Vector3) -> i32 {
    unsafe {
        return SpawnCube(pos);
    };
}

pub fn get_pose(handle: i32) -> RigidPose {
    unsafe {
        return GetBodyPose(handle);
    };
}


extern "C" {
    fn SetupPyramidDemo();

    fn Timestep();

    fn SpawnCube(pos: Vector3) -> i32;

    fn GetBodyPose(handle: i32) -> RigidPose;
}

/*
        const int pyramidCount = 40;
        for (int pyramidIndex = 0; pyramidIndex < pyramidCount; ++pyramidIndex)
        {
            const int rowCount = 20;
            for (int rowIndex = 0; rowIndex < rowCount; ++rowIndex)
            {
                int columnCount = rowCount - rowIndex;
                for (int columnIndex = 0; columnIndex < columnCount; ++columnIndex)
                {
                    sim.Bodies.Add(BodyDescription.CreateDynamic(new Vector3(
                            (-columnCount * 0.5f + columnIndex) * boxShape.Width,
                            (rowIndex + 0.5f) * boxShape.Height,
                            (pyramidIndex - pyramidCount * 0.5f) * (boxShape.Length + 4)),
                        boxInertia, boxIndex, 0.01f));
                }
            }
        }
 */


#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::bepu::{SpawnCube, Vector3, Timestep, GetBodyPose};

    use super::{SetupPyramidDemo};


    #[test] 
    fn pyramid_demo() {
        unsafe {
            SetupPyramidDemo();

            let mut handles = vec![];

            let mut total_cube_count = 0;

            const PYRAMID_COUNT: i32 = 40;
            for pyramid_index in 0..PYRAMID_COUNT {
                const ROW_COUNT: i32 = 20;
                for row_index in 0..ROW_COUNT {
                    let column_count = ROW_COUNT - row_index;
                    for column_index in 0..column_count {
                        total_cube_count += 1;
                        handles.push(SpawnCube(
                            Vector3::new(
                                (-column_count as f32 * 0.5 + column_index as f32) * 1.0,
                                (row_index as f32 + 0.5) * 1.0,
                                (pyramid_index as f32 - PYRAMID_COUNT as f32 * 0.5) * (1.0 + 4.0),
                        )));
                    }
                }
            }

            println!("Created {} cubes", total_cube_count);

            let start_time = std::time::Instant::now();

            for _ in 0..1000 {
                Timestep();
            }
            
            //println!("Average timestep took: {:?}", avg_timestep / 100);
            println!("Total elapsed time: {:?}", start_time.elapsed());
            println!("Per timestep {:?}", start_time.elapsed() / 1000);
        }
    }
}