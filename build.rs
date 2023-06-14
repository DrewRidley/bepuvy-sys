// use std::{process::Command,fs, path::Path};

// use home_dir::HomeDirExt;


// //Finds the AOT folder with the libraries required for linking.
// //Will pick the highest version that exists, ensuring it is at least 8.0.
// fn find_aot_base() -> String {
//     let packages_dir = "~/.nuget/packages".expand_home().expect("Failed to find nuget folder!");
//     for package in fs::read_dir(packages_dir).expect("Failed to read nuget folder.").filter_map(|d| d.ok()).filter(|d| {
//         d.file_name().to_str().unwrap().starts_with("runtime.")
//     }) {
//         for folder in fs::read_dir(package.path()).expect("Failed to read a folder in package.").filter_map(|d| d.ok()) {
//             let path = folder.path();

//             if path.file_name().expect("Path should have file name").to_str().unwrap().starts_with("8.0") {
//                 return path.to_str().expect("Failed to parse resultant path as string").into();
//             }

//             if path.to_str().expect("Failed to parse path as string").starts_with("8.") {
//                 return path.to_str().expect("Failed to parse resultant path as string").into();
//             }
//         }
//     }
//     panic!("Failed to find ILCompiler");
// }

// fn main() {
//     //The path to the C# project. If none, it will be cloned from git.
//     let bepuvy_path: Option<String> = Some("/users/drewridley/RiderProjects/Bepuvy/Bepuvy".into());

//     //An override for a specific AOT linking path. Recommended for MacOS.
//     let aot_override: Option<String> = Some("/users/drewridley/.nuget/packages/runtime.osx-arm64.microsoft.dotnet.ilcompiler/8.0.0-preview.6.23313.22".into());

//     let use_existing_build: bool = true;

//     //Whether this is a debug or release build.
//     //The NativeAOT application will build with same configuration.
//     let debug = std::env::var("PROFILE").expect("PROFILE should exist!") == "debug";

//     if std::env::consts::OS != "macos" && std::env::consts::OS != "windows" && std::env::consts::OS != "linux" {
//         panic!("Bepuvy currently only suports desktop platforms");
//     }

//     let arch = match (std::env::consts::OS, std::env::consts::ARCH) {
//         ("macos", "aarch64") => "osx-arm64",
//         ("windows", "x86_64") => "win-x64",
//         ("linux", "x86_64") => "linux-x64",
//         (os, arch) => panic!("{} on {} is not supported by Bepuvy.", os, arch),
//     };

//     //Ensure bepuvy exists.
//     let bepuvy_path: String = bepuvy_path.unwrap_or_else(|| {
//         //Bepuvy-bepu must be cloned from source.
//         let build_dir = std::env::temp_dir();
//         let build_dir_str = build_dir.to_str().expect("Failed to parse build directory as string").to_owned();
    
//         if Path::new(&(build_dir_str.clone() + "/Bepuvy-bepu")).is_dir() {
//             return build_dir_str + "Bepuvy-bepu/Bepuvy";
//         }

//         let output = Command::new("git")
//             .arg("clone")
//             .arg("https://github.com/DrewRidley/Bepuvy-bepu.git")
//             .current_dir(build_dir_str.as_str())
//             .status()
//             .expect("Failed to clone bepuvy-bepu for building");

//         if output.success() {
//             build_dir_str + "Bepuvy-bepu/Bepuvy"
//         }
//         else {
//             panic!("Failed to clone bepuvy-bepu!");
//         }
//     });

//     if use_existing_build {
//         let output_dir = bepuvy_path.to_owned() + "/bin/" + (if debug {
//             "Debug"
//         } else {
//             "Release"
//         }) + "/net8.0/" + arch + "/publish";


//         println!("cargo:rustc-link-search={}", output_dir);
//         println!("cargo:rustc-link-lib=static:+verbatim=Bepuvy.a");

//         let required_libs = [
//             "libbootstrapperdll.a",
//             "libRuntime.WorkstationGC.a",
//             "libstdc++compat.a",
//             "libeventpipe-disabled.a",
//             "libSystem.Native.a",
//             "libSystem.IO.Compression.Native.a",
//             "libSystem.Net.Security.Native.a",
//             "libSystem.Security.Cryptography.Native.OpenSsl.a",
//         ];

//         let aot_base = aot_override.unwrap_or(find_aot_base());

//         //Search in the AOT base folder...
//         println!("cargo:rustc-link-search={}/sdk", aot_base);
//         println!("cargo:rustc-link-search={}/framework", aot_base);

//         for lib_name in required_libs {
//             println!("cargo:rustc-link-lib=static:+verbatim={}", lib_name);
            
//         }

//         println!("cargo:rustc-link-arg=-ldl");
//         println!("cargo:rustc-link-arg=-lm");
//         //println!("cargo:rustc-link-args=-Wl,-u,_NativeAOT_StaticInitialization");
//         println!("cargo:rustc-link-args=-u,_NativeAOT_StaticInitialization");
//         //println!("cargo:rustc-link-arg=-Wl,-no_fixup_chains");    
//         println!("cargo:rustc-link-arg=-framework");
//         println!("cargo:rustc-link-arg=Foundation");


//     }
//     else {
        
//     let output = Command::new("dotnet8")
//     .arg("publish")
//     .arg("/p:NativeLib=Static")
//     .arg("-p:PublishAot=true")
//     .arg("-p:EnableNativeEventPipe=false")
//     .arg("-p:IlcInstructionSet=apple-m1")
//     .arg("-p:InvariantGlobalization=true")
//     .arg("Bepuvy.csproj")
//     .arg("-r")
//     .arg(arch)
//     .arg("-c")
//     .arg(if debug {"Debug"} else {"Release"})
//     .current_dir(&bepuvy_path)
//     .status()
//     .expect("Failed to execute command");
        
//     if output.success() {
//         let output_dir = bepuvy_path.to_owned() + "/bin/" + (if debug {
//             "Debug"
//         } else {
//             "Release"
//         }) + "/net8.0/" + arch + "/publish";


//         println!("cargo:rustc-link-search={}", output_dir);
//         println!("cargo:rustc-link-lib=static:+verbatim=Bepuvy.a");

//         let required_libs = [
//             "libbootstrapperdll.a",
//             "libRuntime.WorkstationGC.a",
//             "libstdc++compat.a",
//             "libeventpipe-disabled.a",
//             "libSystem.Native.a",
//             "libSystem.IO.Compression.Native.a",
//             "libSystem.Globalization.Native.a",
//             "libSystem.Net.Security.Native.a",
//             "libSystem.Security.Cryptography.Native.OpenSsl.a",
//         ];

//         let aot_base = aot_override.unwrap_or(find_aot_base());

//         //Search in the AOT base folder...
//         println!("cargo:rustc-link-search={}/sdk", aot_base);
//         println!("cargo:rustc-link-search={}/framework", aot_base);

//         for lib_name in required_libs {
//             println!("cargo:rustc-link-lib=static:+verbatim={}", lib_name);
            
//         }

//         println!("cargo:rustc-link-arg=-ldl");
//         println!("cargo:rustc-link-arg=-lm");
//         println!("cargo:rustc-link-args=-Wl,-u,_NativeAOT_StaticInitialization");

//         println!("cargo:rustc-link-arg=-Wl,-no_fixup_chains");    
//         println!("cargo:rustc-link-arg=-framework");
//         println!("cargo:rustc-link-arg=Foundation");
//         println!("cargo:rustc-link-arg=-lc++");

//     }
//     else {
//         panic!("Failed to build with NativeAOT: {}", output);
//     }
//     }
// }



// // fn main() {
// //     let output_dir = "/Users/drewridley/RiderProjects/Bepuvy/Bepuvy/bin/Release/net8.0/osx-arm64/publish";

// //     println!("cargo:rustc-link-search={}", output_dir);
// //     println!("cargo:rustc-link-lib=static:+verbatim=Bepuvy.a");
// //     let aot_base = "/users/drewridley/.nuget/packages/runtime.osx-arm64.microsoft.dotnet.ilcompiler/8.0.0-preview.6.23309.7";

// //     //Search in the AOT base folder...
// //     println!("cargo:rustc-link-search={}/sdk", aot_base);
// //     println!("cargo:rustc-link-search={}/framework", aot_base);

// //     let required_libs = [
// //         "libbootstrapperdll.a",
// //         "libRuntime.WorkstationGC.a",
// //         "libstdc++compat.a",
// //         "libeventpipe-disabled.a",
// //         "libSystem.Native.a",
// //         "libSystem.IO.Compression.Native.a",
// //         "libSystem.Globalization.Native.a",
// //         "libSystem.Net.Security.Native.a",
// //         "libSystem.Security.Cryptography.Native.OpenSsl.a",
// //     ];

// //     for lib_name in required_libs {
// //         println!("cargo:rustc-link-lib=static:+verbatim={}", lib_name);   
// //     }

// //     println!("cargo:rustc-link-arg=-ldl");
// //     println!("cargo:rustc-link-arg=-lm");
// //     println!("cargo:rustc-link-args=-Wl,-u,_NativeAOT_StaticInitialization");

// //     println!("cargo:rustc-link-arg=-Wl,-no_fixup_chains");    
// //     println!("cargo:rustc-link-arg=-framework");
// //     println!("cargo:rustc-link-arg=Foundation");
// // }

use std::process::Command;


//Lets watch for changes on ANY of the .cs files and rebuild the bindings.
fn register_change_detection() {
    let cs_dir = std::fs::read_dir("./Bepuvy/Bepuvy").expect("Unable to read cs directory");

    for file in cs_dir.filter(|file| {
        if let Ok(f) = file {
           if let Ok(t) = f.file_type() {
                return t.is_file() && f.file_name().to_str().unwrap().ends_with(".cs");
           }
           
           return false;
        }

        false
    }) {
        //For each CS file, we need to register it with the watcher.
        println!("cargo:rerun-if-changed=Bepuvy/Bepuvy/{}", file.unwrap().file_name().to_str().unwrap());
    }
}

//Finds a suitable AOT base for a given compiler version. 
//This will try to find the most appropiate version and if theres no sufficiently similar version, it will panic.
fn find_suitable_base() {   

}

//If dotnet doesn't exist, this will download and install it.
fn install_dotnet() {
    panic!("Dotnet is not installed! Bepuvy does not support automatic installations.");
}


fn main() {
    let arch = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "osx-arm64",
        ("windows", "x86_64") => "win-x64",
        ("linux", "x86_64") => "linux-x64",
        (os, arch) => panic!("{} on {} is not supported by Bepuvy.", os, arch),
    };

    register_change_detection();

    //If we don't have dotnet we have to install it.
    if which::which("dotnet").is_err() {
        install_dotnet();
    }

    let compilation_output = Command::new("dotnet")
        .arg("publish")
        .arg("/p:NativeLib=Static")
        .arg("-p:EnableNativeEventPipe=false")
        .arg(if arch == "osx-arm64" {
            "-p:IlcInstructionSet=apple-m1"
        } else {
            "-p:IlcInstructionSet=x86-x64-v4"
        })
        .arg("-p:InvariantGlobalization=true")
        .arg("--use-current-runtime")
        .current_dir("./Bepuvy/Bepuvy")

        //Output to the dist directory so we don't have to discriminate Debug/Release folders
        .arg("-o")
        .arg("dist")

        .arg("-r")
        .arg("osx-arm64")
        .output();

    if let Err(e) = compilation_output {
        panic!("NativeAOT compilation failed: {}", e);
    }

    let ext = match std::env::consts::OS {
        "windows" => ".lib",
        _ => ".a"
    };

    //Rename the library because the linker assumes prefix 'lib'.
    std::fs::rename(format!("./Bepuvy/Bepuvy/dist/Bepuvy{}", ext), format!("./Bepuvy/Bepuvy/dist/libBepuvy{}", ext)).expect("Failed to rename output library!");
   
    //Now that we finished compilation, lets link the AOT libraries.
    let aot_base = "/users/drewridley/.nuget/packages/runtime.osx-arm64.microsoft.dotnet.ilcompiler/8.0.0-preview.6.23313.22";
    println!("cargo:rustc-link-search={}/sdk", aot_base);
    println!("cargo:rustc-link-search={}/framework", aot_base);

    let libs = vec![
        "bootstrapperdll",
        "Runtime.WorkstationGC",
        "eventpipe-disabled",
        "System.Native",
        "System.IO.Compression.Native",
        "System.Globalization.Native",
        "System.Net.Security.Native",
        "System.Security.Cryptography.Native.OpenSsl"
    ];

    for lib in libs {
        println!("cargo:rustc-link-lib=static:-bundle,+whole-archive={lib}");
    }

    let dist_dir = std::env::current_dir().expect("Failed to get current dir").to_str().unwrap().to_owned() + "/Bepuvy/Bepuvy/dist";

    println!("cargo:rustc-link-search={dist_dir}");
    println!("cargo:rustc-link-lib=static:-bundle,+whole-archive=Bepuvy");
    println!("cargo:rustc-link-lib=c++");


    println!("cargo:rustc-link-arg=-ldl");
    println!("cargo:rustc-link-arg=-lm");
    println!("cargo:rustc-link-arg=-Wl,-no_fixup_chains");    
    println!("cargo:rustc-link-arg=-framework");
    println!("cargo:rustc-link-arg=Foundation");
    println!("cargo:rustc-link-args=-Wl,-u,_NativeAOT_StaticInitialization");
}

