use super::collisions::*;
use super::functions::*;
use super::handles::*;
use super::utilities::*;
use super::*;

/// Convenience collection that stores extra properties about bodies and statics, indexed by the body or static handle.
///
/// # Remarks
///
/// This is built for use cases relying on random access like the narrow phase. For maximum performance with sequential access, an index-aligned structure would be better.
#[repr(C)]
pub struct CollidableProperty<T> {
    /// Bodies and statics each have 'handle spaces', like namespaces. A body and static can have the same integer valued handle.
    /// So, we need to have two different buffers for data.
    pub simulation: SimulationHandle,
    pub pool: BufferPoolHandle,
    pub body_data: Buffer<T>,
    pub static_data: Buffer<T>,
}

impl<T> CollidableProperty<T> {
    /// Constructs a new collection to store handle-aligned body and static properties.
    ///
    /// # Arguments
    ///
    /// * `simulation`: Simulation to track.
    /// * `pool`: Pool from which to pull internal resources.
    pub fn new(simulation: SimulationHandle, pool: BufferPoolHandle) -> Self {
        let mut body_handle_to_location_mapping = Buffer::<BodyMemoryLocation>::new();
        unsafe {
            GetBodyHandleToLocationMapping(simulation, &mut body_handle_to_location_mapping);
        }
        let body_data = unsafe {
            AllocateAtLeast(
                pool,
                body_handle_to_location_mapping.len * std::mem::size_of::<T>() as i32,
            )
        };

        let mut static_handle_to_index_mapping = Buffer::<i32>::new();
        unsafe {
            GetStaticHandleToLocationMapping(simulation, &mut static_handle_to_index_mapping);
        }
        let static_data = unsafe {
            AllocateAtLeast(
                pool,
                static_handle_to_index_mapping.len * std::mem::size_of::<T>() as i32,
            )
        };

        Self {
            simulation,
            pool,
            body_data: unsafe { body_data.as_buffer() },
            static_data: unsafe { static_data.as_buffer() },
        }
    }

    pub fn allocate_body(&mut self, body_handle: BodyHandle) -> &mut T {
        if body_handle.value >= self.body_data.len {
            let copy_count_in_bytes = std::mem::size_of::<T>() * (self.body_data.len) as usize;
            let target_capacity_in_bytes =
                std::mem::size_of::<T>() * (body_handle.value + 1) as usize;
            let mut byte_buffer = ByteBuffer {
                memory: self.body_data.memory as *mut u8,
                len: self.body_data.len,
                id: self.body_data.id,
            };
            unsafe {
                ResizeToAtLeast(
                    self.pool,
                    &mut byte_buffer,
                    target_capacity_in_bytes as i32,
                    copy_count_in_bytes as i32,
                );
            }
            self.body_data = unsafe { byte_buffer.as_buffer() };
        }
        &mut self.body_data[body_handle.value]
    }

    pub fn allocate_static(&mut self, static_handle: StaticHandle) -> &mut T {
        if static_handle.value >= self.static_data.len {
            let copy_count_in_bytes = std::mem::size_of::<T>() * (self.static_data.len) as usize;
            let target_capacity_in_bytes =
                std::mem::size_of::<T>() * (static_handle.value + 1) as usize;
            let mut byte_buffer = ByteBuffer {
                memory: self.static_data.memory as *mut u8,
                len: self.static_data.len,
                id: self.static_data.id,
            };
            unsafe {
                ResizeToAtLeast(
                    self.pool,
                    &mut byte_buffer,
                    target_capacity_in_bytes as i32,
                    copy_count_in_bytes as i32,
                );
            }
            self.static_data = unsafe { byte_buffer.as_buffer() };
        }
        &mut self.static_data[static_handle.value]
    }

    pub fn allocate_collidable(&mut self, collidable_reference: CollidableReference) -> &mut T {
        if collidable_reference.mobility() == CollidableMobility::Static {
            self.allocate_static(collidable_reference.static_handle())
        } else {
            self.allocate_body(collidable_reference.body_handle())
        }
    }

    pub fn ensure_body_capacity(&mut self, capacity: i32) {
        if capacity > self.body_data.len {
            let mut byte_buffer = ByteBuffer {
                memory: self.body_data.memory as *mut u8,
                len: self.body_data.len,
                id: self.body_data.id,
            };
            unsafe {
                ResizeToAtLeast(
                    self.pool,
                    &mut byte_buffer,
                    capacity * std::mem::size_of::<T>() as i32,
                    byte_buffer.len,
                );
            }
            self.body_data = unsafe { byte_buffer.as_buffer() };
        }
    }
    pub fn ensure_static_capacity(&mut self, capacity: i32) {
        if capacity > self.static_data.len {
            let mut byte_buffer = ByteBuffer {
                memory: self.static_data.memory as *mut u8,
                len: self.static_data.len,
                id: self.static_data.id,
            };
            unsafe {
                ResizeToAtLeast(
                    self.pool,
                    &mut byte_buffer,
                    capacity * std::mem::size_of::<T>() as i32,
                    byte_buffer.len,
                );
            }
            self.static_data = unsafe { byte_buffer.as_buffer() };
        }
    }

    /// Returns all held resources.
    pub fn dispose(self) {
        unsafe {
            DeallocateById(self.pool, self.body_data.id);
            DeallocateById(self.pool, self.static_data.id);
        }
    }
}

impl<T> std::ops::Index<BodyHandle> for CollidableProperty<T> {
    type Output = T;

    fn index(&self, index: BodyHandle) -> &Self::Output {
        assert!(index.value >= 0 && index.value < self.body_data.len);
        &self.body_data[index.value]
    }
}

impl<T> std::ops::Index<StaticHandle> for CollidableProperty<T> {
    type Output = T;

    fn index(&self, index: StaticHandle) -> &Self::Output {
        assert!(index.value >= 0 && index.value < self.static_data.len);
        &self.static_data[index.value]
    }
}

impl<T> std::ops::Index<CollidableReference> for CollidableProperty<T> {
    type Output = T;

    fn index(&self, index: CollidableReference) -> &Self::Output {
        if index.mobility() == CollidableMobility::Static {
            &self[index.static_handle()]
        } else {
            &self[index.body_handle()]
        }
    }
}
