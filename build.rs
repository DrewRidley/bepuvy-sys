use std::env;
use std::path::{PathBuf, Path};
use walkdir::WalkDir;


fn find_lib(filename: &str) -> Option<String> {

    let user_dir = directories::UserDirs::new().expect("Home directories should exist!");

    let path = user_dir.home_dir();
    for entry in WalkDir::new(path).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == filename {
            return Some(entry.path().display().to_string());
        }
    }
    None
}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory


    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.


    //  let lib_dir = PathBuf::from(
    //      "/Volumes/T7/Projects/scratchpad/Abomination/AbominationInterop/AbominationInterop/bin/Release/net8.0/osx.13-arm64/publish"
    //  );

    // println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap()); 

    //println!("cargo:rustc-link-lib=dylib=AbominationInterop");

    let required_libs = [
        //Generated with:
        // https://github.com/dotnet/runtime/issues/70277
        //"sfx_combined.a"

        "libbootstrapperdll.a",
        "libRuntime.WorkstationGC.a",
        "libeventpipe-disabled.a",
        "libstdc++compat.a",
        "libSystem.Native.a",
        "libSystem.IO.Compression.Native.a",
        "libSystem.Net.Security.Native.a",
        "libSystem.Security.Cryptography.Native.Apple.a",
        "libSystem.Security.Cryptography.Native.OpenSsl.a",
        "libSystem.Globalization.Native.a"
    ];

    for lib_name in required_libs {
        let lib_path = find_lib(lib_name).expect(&format!("Could not find required library: {}", lib_name));
        println!("cargo:rustc-link-lib=static:+verbatim={}", lib_path);
    }

    println!("cargo:rustc-link-lib=static:+verbatim=/Volumes/T7/Projects/scratchpad/Abomination/AbominationInterop/AbominationInterop/bin/Release/net8.0/osx.13-arm64/publish/AbominationInterop.a");
        
    
    //Required for M1 Mac
    println!("cargo:rustc-link-lib=objc");
    println!("cargo:rustc-link-lib=swiftCore");
    println!("cargo:rustc-link-lib=swiftFoundation");
    println!("cargo:rustc-link-lib=icucore");
    
    println!("cargo:rustc-link-arg=_NativeAOT_StaticInitialization");

    println!("cargo:rustc-link-search=/usr/lib/swift");


    let bindings = bindgen::Builder::default()

        .clang_args(&["-x", "c++"])
        .clang_args(&["-std=c++14"])

        //Required for MacOS apparently...
        .clang_args(&["-framework", "Foundation"])
        .clang_args(&["-framework", "Security"])
        .clang_args(&["-framework", "GSS"])

        // The input header we would like to generate
        // bindings for.
        .header("headers/BepuPhysics.h")
        .header("headers/Bodies.h")
        .header("headers/CollidableProperty.h")
        .header("headers/Collisions.h")
        .header("headers/Constraints.h")
        .header("headers/Continuity.h")
        .header("headers/Handles.h")
        .header("headers/InteropMath.h")
        .header("headers/PoseIntegration.h")
        .header("headers/Shapes.h")
        .header("headers/Statics.h")
        .header("headers/Tree.h")
        .header("headers/Utilities.h")

    


        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}