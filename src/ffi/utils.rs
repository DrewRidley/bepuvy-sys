extern "C" {
    #[link_name = "Utils.GetSIMDWidth"]
    pub fn get_simd_width() -> u32;
    /// Gets the number of threads exposed by the operating system on this platform. Cores with SMT can show as having multiple threads.
    ///
    /// # Returns
    ///
    /// Number of threads exposed by the operating system on this platform.
    #[link_name = "Utils.GetPlatformThreadCount"]
    pub fn get_platform_thread_count() -> i32;

    /// Estimates the number of bytes managed by the garbage collector.
    ///
    /// # Returns
    ///
    /// Estimated number of bytes allocated from managed memory.
    #[link_name = "Utils.GetGCMemorySize"]
    pub fn get_gc_allocated_memory_size() -> u64;

}
