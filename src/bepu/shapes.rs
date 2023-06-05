

#[repr(C)]
pub struct BepuBox {
    half_width: f32,
    half_height: f32,
    half_len: f32
}   

impl BepuBox {
    pub fn new(width: f32, height: f32, len: f32) -> Self {
        BepuBox {
            half_width: width * 0.5,
            half_height: height * 0.5,
            half_len: len * 0.5
        }
    }
}