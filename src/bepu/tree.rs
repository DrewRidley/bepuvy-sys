use super::interop_math::*;
use super::utilities::*;

#[repr(C)]
pub struct NodeChild {
    pub min: Vector3,
    pub index: i32,
    pub max: Vector3,
    pub leaf_count: i32,
}

// Note that the format of this node implies that we don't explicitly test against the root bounding box during normal execution.
// For almost all broad phase use cases, queries will be inside the root bounding box anyway. For non-broad phase uses, the outer bounding box will likely be stored
// elsewhere - for example, in the broad phase.

/// A 2-wide tree node.
#[repr(C)]
pub struct Node {
    pub a: NodeChild,
    pub b: NodeChild,
}

// Node metadata isn't required or used during collision testing, so it is stored separately.
// This helps avoid splitting Nodes across cache lines and decreases memory bandwidth requirements during testing.
/// Metadata associated with a 2-child tree node.
#[repr(C)]
pub struct Metanode {
    pub parent: i32,
    pub index_in_parent: i32,
    pub packed_flag_and_cost_change: i32,
}

/// Pointer to a leaf's tree location.
///
/// # Remarks
///
/// The identity of a leaf is implicit in its position within the leaf array.
#[repr(C)]
pub struct Leaf {
    pub packed: u32,
}

impl Leaf {
    /// Gets the index of the node that the leaf is directly held by.
    pub fn node_index(&self) -> i32 {
        (self.packed & 0x7FFFFFFF) as i32
    }

    /// Gets which child within the owning node the leaf is in.
    pub fn child_index(&self) -> i32 {
        (self.packed >> 31) as i32
    }

    pub fn new(node_index: i32, child_index: i32) -> Self {
        assert!((child_index & !1) == 0);
        Self {
            packed: ((node_index as u32) & 0x7FFFFFFF) | ((child_index as u32) << 31),
        }
    }
}

/// A 2-child tree.
#[repr(C)]
pub struct Tree {
    /// Buffer of nodes in the tree.
    pub nodes: Buffer<Node>,
    /// Buffer of metanodes in the tree. Metanodes contain metadata that aren't read during most query operations but are useful for bookkeeping.
    pub metanodes: Buffer<Metanode>,
    /// Buffer of leaves in the tree.
    pub leaves: Buffer<Leaf>,
    /// Number of nodes in the tree.
    pub node_count: i32,
    /// Number of leaves in the tree.
    pub leaf_count: i32,
}
