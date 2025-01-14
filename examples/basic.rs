use bepuvy_sys::bepu::{buffer_pool::BufferPool, simulation::Simulation};

fn main() {
    let pool = BufferPool::default();

    let sim = Simulation::new(&pool);
}
