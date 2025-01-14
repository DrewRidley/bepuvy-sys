#![feature(portable_simd)]

pub mod bepu;
pub(crate) mod ffi;
pub(crate) mod types;

pub mod prelude {
    pub use crate::bepu::{
        buffer_pool::BufferPool, dispatcher::ThreadDispatcher, simulation::Simulation,
    };
}
