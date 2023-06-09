use std::env;
use std::fmt::Debug;
use std::path::{PathBuf, Path};
use std::process::Command;
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
    let required_libs = [
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

    let path = "/users/drewridley/RiderProjects/Bepuvy/Bepuvy";
    let debug = false;

    let arch = "osx.13-arm64";

    let final_path = path.to_owned() + "/bin/" + (if debug {
        "Debug"
    } else {
        "Release"
    }) + "/net8.0/" + arch + "/publish/Bepuvy.a";

    let output = Command::new("dotnet")
        .arg("publish")
        .arg("/p:NativeLib=Static")
        .arg("-p:IlcInstructionSet=apple-m1")
        .arg("Bepuvy.csproj")
        .arg("-c")
        .arg(if debug {
            "Debug"
        } else {
            "Release"
        })
        .arg("-r")
        .arg("osx.13-arm64")
        .arg("-f")
        .arg("net8.0")
        .current_dir(path)
        .status()
        .expect("Failed to execute command");

    if output.success() {
        println!("cargo:rustc-link-lib=static:+verbatim={}", final_path);
        println!("cargo:rustc-link-lib=objc");
        println!("cargo:rustc-link-lib=swiftCore");
        println!("cargo:rustc-link-lib=swiftFoundation");
        println!("cargo:rustc-link-lib=icucore");
        println!("cargo:rustc-link-search=/usr/lib/swift");
        println!("cargo:rustc-link-arg=-Wl,-u,_NativeAOT_StaticInitialization");
    }
    else {
        panic!("Failed to build with NativeAOT: {}", output);
    }
}