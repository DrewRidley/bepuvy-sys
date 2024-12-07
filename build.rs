use std::{
    env,
    path::PathBuf,
    process::{Command, Output},
};

fn check_command_output(output: &Output, context: &str) {
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Failed {context}:\nstdout: {stdout}\nstderr: {stderr}");
    }
}

fn get_home_dir() -> PathBuf {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .expect("Could not determine home directory")
}

fn find_native_aot_version() -> String {
    let csproj_content = std::fs::read_to_string("./Bepuvy/Bepuvy/Bepuvy.csproj")
        .expect("Failed to read .csproj file");

    for line in csproj_content.lines() {
        if line.contains("Microsoft.DotNet.ILCompiler") {
            if let Some(version_str) = line
                .split("Version=\"")
                .nth(1)
                .and_then(|s| s.split('\"').next())
            {
                return version_str.to_string();
            }
        }
    }

    panic!("Could not find Microsoft.DotNet.ILCompiler version in .csproj");
}

fn install_dotnet() -> PathBuf {
    let target_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");
    let target_path = PathBuf::from(target_dir);
    let install_dir = target_path.join("dotnet");

    #[cfg(target_os = "windows")]
    {
        let install_script = format!(
            r#"
            $installDir = "{}"
            $ErrorActionPreference = "Stop"
            Invoke-WebRequest -Uri https://dot.net/v1/dotnet-install.ps1 -OutFile dotnet-install.ps1
            .\dotnet-install.ps1 -InstallDir $installDir -NoPath
            "#,
            install_dir.display()
        );

        let output = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(install_script)
            .output()
            .expect("Failed to execute powershell command");

        check_command_output(&output, "installing .NET");
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let install_script = format!(
            r#"
            INSTALL_DIR="{}"
            curl -sSL https://dot.net/v1/dotnet-install.sh | bash /dev/stdin --install-dir "$INSTALL_DIR" --no-path
            "#,
            install_dir.display()
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(install_script)
            .output()
            .expect("Failed to execute shell command");

        check_command_output(&output, "installing .NET");
    }

    let dotnet_path = install_dir.join("dotnet");
    dotnet_path
}

fn register_change_detection() {
    let cs_dir = std::fs::read_dir("./Bepuvy/Bepuvy").expect("Unable to read cs directory");

    for entry in cs_dir.filter_map(|e| e.ok()) {
        if let Ok(ft) = entry.file_type() {
            if ft.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.ends_with(".cs"))
                    .unwrap_or(false)
            {
                println!(
                    "cargo:rerun-if-changed=Bepuvy/Bepuvy/{}",
                    entry.file_name().to_str().unwrap()
                );
            }
        }
    }
}

fn main() {
    println!("cargo:warning=Build script starting");

    // Get target architecture
    let arch = match (env::consts::OS, env::consts::ARCH) {
        ("macos", "aarch64") => "osx-arm64",
        ("windows", "x86_64") => "win-x64",
        ("linux", "x86_64") => "linux-x64",
        (os, arch) => panic!("{} on {} is not supported by Bepuvy.", os, arch),
    };

    register_change_detection();

    // Find .NET and set up environment
    let out_dir_dotnet =
        PathBuf::from(env::var("OUT_DIR").expect("Failed to get OUT_DIR")).join("dotnet/dotnet");

    let dotnet_path = if which::which("dotnet").is_ok() {
        PathBuf::from("dotnet")
    } else if out_dir_dotnet.exists() {
        out_dir_dotnet
    } else {
        install_dotnet()
    };

    // Build the project
    let output = Command::new(&dotnet_path)
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
        .arg("-o")
        .arg("dist")
        .arg("-r")
        .arg(arch)
        .output()
        .expect("Failed to execute dotnet command");

    check_command_output(&output, "building Bepuvy");

    // Handle library extension and renaming
    let ext = if cfg!(windows) { ".lib" } else { ".a" };
    let old_name = format!("./Bepuvy/Bepuvy/dist/Bepuvy{}", ext);
    let new_name = format!("./Bepuvy/Bepuvy/dist/libBepuvy{}", ext);

    std::fs::rename(&old_name, &new_name).expect("Failed to rename library file");

    // Find NativeAOT version and set up linking
    let nativeaot_version = find_native_aot_version();
    let home_dir = get_home_dir();

    let aot_base = home_dir
        .join(".nuget")
        .join("packages")
        .join(format!("runtime.{arch}.microsoft.dotnet.ilcompiler"))
        .join(&nativeaot_version);

    if !aot_base.exists() {
        panic!(
            "NativeAOT package not found at expected location: {}",
            aot_base.display()
        );
    }

    println!("cargo:rustc-link-search={}/sdk", aot_base.display());
    println!("cargo:rustc-link-search={}/framework", aot_base.display());

    let mut libs = vec![
        // "bootstrapperdll",
        "Runtime.WorkstationGC",
        "eventpipe-disabled",
        "System.Native",
        "System.IO.Compression.Native",
        "System.Globalization.Native",
        "System.Net.Security.Native",
        "System.Security.Cryptography.Native.OpenSsl",
    ];

    if arch == "osx-arm64" {
        libs.push("System.Security.Cryptography.Native.Apple");
    };

    for lib in libs {
        println!("cargo:rustc-link-lib=static:-bundle,+whole-archive={lib}");
    }

    println!("cargo:rustc-link-lib=static:-bundle,+whole-archive,+verbatim=libbootstrapperdll.o");
    println!(
        "cargo:rustc-link-lib=static:-bundle,+whole-archive,+verbatim=libstandalonegc-enabled.a"
    );

    let dist_dir = env::current_dir()
        .expect("Failed to get current directory")
        .join("Bepuvy/Bepuvy/dist");

    println!("cargo:rustc-link-search={}", dist_dir.display());
    println!("cargo:rustc-link-lib=static:-bundle,+whole-archive=Bepuvy");
    println!("cargo:rustc-link-lib=c++");

    println!("cargo:rustc-link-arg=-ldl");
    println!("cargo:rustc-link-arg=-lm");
    println!("cargo:rustc-link-args=-Wl,-u,_NativeAOT_StaticInitialization");
}
