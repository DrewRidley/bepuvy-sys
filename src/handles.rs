
pub struct TypedIndex {
    packed: u32
}

impl TypedIndex {
    pub fn get_type(&self) -> i32 {
        ((self.packed & 0x7F000000) >> 24).try_into().unwrap()
    }

    pub fn get_index(&self) -> i32 {
        (self.packed & 0x00FFFFFF).try_into().unwrap()
    }

    pub fn exists(&self) -> bool {
        (self.packed & (1 << 31)) > 0
    }
}

#[repr(C)]
pub struct InstanceHandle {
    raw_value: i32,
}

impl InstanceHandle {
    pub fn get_index(&self) -> i32 {
        self.raw_value & 0x00FFFFFF
    }

    pub fn get_version(&self) -> i32 {
        (self.raw_value >> 24) & 0xF
    }

    pub fn get_type_index(&self) -> i32 {
        (self.raw_value >> 28) & 0x7
    }

    pub fn is_null(&self) -> bool {
        self.raw_value == 0
    }
} 

pub struct BodyHandle(pub i32);
pub struct StaticHandle(pub i32);
pub struct ConstraintHandle(pub i32);

pub type SimulationHandle = InstanceHandle;
pub type BufferPoolHandle = InstanceHandle;
pub type ThreadDispatcherHandle = InstanceHandle;