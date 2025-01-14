/// Represents an index with an associated type packed into a single integer.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TypedIndex {
    /// Bit packed representation of the typed index.
    pub packed: u32,
}

impl TypedIndex {
    /// Gets the type index of the object.
    pub fn ty(&self) -> i32 {
        ((self.packed & 0x7F000000) >> 24) as i32
    }

    /// Gets the index of the object.
    pub fn index(&self) -> i32 {
        (self.packed & 0x00FFFFFF) as i32
    }

    /// Gets whether this index actually refers to anything. The Type and Index should only be used if this is true.
    pub fn exists(&self) -> bool {
        (self.packed & (1 << 31)) > 0
    }
}

/// Points to an instance in an instance directory.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InstanceHandle {
    pub raw_value: i32,
}

impl InstanceHandle {
    pub fn index(&self) -> i32 {
        self.raw_value & 0x00FFFFFF
    }
    pub fn version(&self) -> i32 {
        (self.raw_value >> 24) & 0xF
    }
    pub fn type_index(&self) -> i32 {
        (self.raw_value >> 28) & 0x7
    }

    pub fn is_null(&self) -> bool {
        self.raw_value == 0
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BodyHandle {
    pub value: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StaticHandle {
    pub value: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ConstraintHandle {
    pub value: i32,
}

pub type SimulationHandle = InstanceHandle;
pub type BufferPoolHandle = InstanceHandle;
pub type ThreadDispatcherHandle = InstanceHandle;
