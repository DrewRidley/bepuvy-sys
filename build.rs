use std::{path::PathBuf, process::Command};

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
        println!(
            "cargo:rerun-if-changed=Bepuvy/Bepuvy/{}",
            file.unwrap().file_name().to_str().unwrap()
        );
    }
}

fn install_dotnet() -> PathBuf {
    let target_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let target_path = PathBuf::from(target_dir);
    let mut install_dir = target_path.join("dotnet");

    #[cfg(target_os = "windows")]
    {
        let install_script = format!(
            r#"
            $installDir = "{}"
            Invoke-WebRequest -Uri https://dot.net/v1/dotnet-install.ps1 -OutFile dotnet-install.ps1
            .\dotnet-install.ps1 -InstallDir $installDir -NoPath
            "#,
            install_dir.display()
        );

        let status = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(install_script)
            .status()
            .expect("failed to execute powershell script");

        if !status.success() {
            panic!("Failed to install Dotnet on Windows!");
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let install_script = format!(
            r#"
            INSTALL_DIR="{}"
            curl -sSL https://dot.net/v1/dotnet-install.sh | bash /dev/stdin --install-dir $INSTALL_DIR --no-path
            "#,
            install_dir.display()
        );

        let status = Command::new("sh")
            .arg("-c")
            .arg(install_script)
            .status()
            .expect("failed to execute shell script");

        if !status.success() {
            panic!("Failed to install Dotnet on Linux/Mac!");
        }
    }

    install_dir.push("dotnet");

    println!("Dotnet installed successfully at {}", install_dir.display());
    install_dir
}

fn main() {
    let arch = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "osx-arm64",
        ("windows", "x86_64") => "win-x64",
        ("linux", "x86_64") => "linux-x64",
        (os, arch) => panic!("{} on {} is not supported by Bepuvy.", os, arch),
    };

    register_change_detection();

    // If we don't have dotnet, we have to install it.
    let out_dir_dotnet = {
        let mut path = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
        path.push("dotnet/dotnet");
        path
    };

    let dotnet_path = if which::which("dotnet").is_ok() {
        PathBuf::from("dotnet")
    } else if out_dir_dotnet.exists() {
        out_dir_dotnet
    } else {
        install_dotnet()
    };

    let compilation_output = Command::new(dotnet_path) // Use the dotnet path from install_dotnet
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
        .arg(arch)
        .output();

    match compilation_output {
        Ok(output) => {
            let utf = String::from_utf8(output.stdout).unwrap();
            if utf.contains("error") {
                panic!(
                    "NativeAOT had one or more compilation errors within Bepuvy: \n{}",
                    utf
                );
            }
        }
        Err(e) => panic!("NativeAOT compilation failed: {:?}", e),
    }

    let ext = match std::env::consts::OS {
        "windows" => ".lib",
        _ => ".a",
    };

    //Rename the library because the linker assumes prefix 'lib'.
    std::fs::rename(
        format!("./Bepuvy/Bepuvy/dist/Bepuvy{}", ext),
        format!("./Bepuvy/Bepuvy/dist/libBepuvy{}", ext),
    )
    .expect("Failed to rename output library!");

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
        "System.Security.Cryptography.Native.OpenSsl",
    ];

    for lib in libs {
        println!("cargo:rustc-link-lib=static:-bundle,+whole-archive={lib}");
    }

    let dist_dir = std::env::current_dir()
        .expect("Failed to get current dir")
        .to_str()
        .unwrap()
        .to_owned()
        + "/Bepunvy/Bepuvy/dist";

    println!("cargo:rustc-link-search={dist_dir}");
    println!("cargo:rustc-link-lib=static:-bundle,+whole-archive=Bepuvy");
    println!("cargo:rustc-link-lib=c++");

    println!("cargo:rustc-link-arg=-ldl");
    println!("cargo:rustc-link-arg=-lm");
    //println!("cargo:rustc-link-arg=-Wl,-no_fixup_chains");
    // println!("cargo:rustc-link-arg=-framework");
    // println!("cargo:rustc-link-arg=Foundation");
    println!("cargo:rustc-link-args=-Wl,-u,_NativeAOT_StaticInitialization");

    // // The bindgen::Builder is the main entry point
    // // to bindgen, and lets you build up options for
    // // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    //     // The input header we would like to generate
    //     // bindings for.
    //     .header("/home/drewr/Documents/Projects/scratchpad/Abomination/AbominationInterop/BepuPhysicsCPP/BepuPhysics.hpp")
    //     // Tell cargo to invalidate the built crate whenever any of the
    //     // included header files changed.
    //     // .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     // Finish the builder and generate the bindingsecify C++14 standard if needed
    //     .generate()
    //     // Unwrap the Result and panic on failure.
    //     .expect("Unable to generate bindings");

    // panic!("Successfully generated bindings...");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
