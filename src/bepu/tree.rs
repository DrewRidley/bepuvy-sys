use glam::Vec3;

#[repr(C)]
pub struct NodeChild {
    min: Vec3,
    index: i32,
    max: Vec3,
    leaf_count: i32
}

#[repr(C)]
pub struct Node {
    a: NodeChild,
    b: NodeChild
}

#[repr(C)]
pub struct MetaNode {
    parent: i32,
    index_in_parent: i32,
    packed_flag_and_cost_change: i32
}