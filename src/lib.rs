pub mod Bepu {
    extern "C" {
        pub fn Initialize();
        pub fn GetPlatformThreadCount() -> i32;
    }
}