use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const APP_NAME: &str = "Ciart Studio";
const BUNDLE_ID: &str = "com.ciart.studio";
const ICON_NAME: &str = "AppIcon";

fn profile_dir(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(3)
        .expect("OUT_DIR should live under target/<profile>/build/<pkg>/out")
        .to_path_buf()
}

pub fn build_ios() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let app = profile_dir(&out_dir).join(format!("{APP_NAME}.app"));
    if let Err(err) = fs::create_dir_all(&app) {
        println!("cargo:warning=failed to create {APP_NAME}.app: {err}");
        return;
    }
    write_info_plist("ios", &app);

    let platform = if env::var("CARGO_CFG_TARGET_ABI").as_deref() == Ok("sim") {
        "iphonesimulator"
    } else {
        "iphoneos"
    };
    compile_icon(
        &app,
        &out_dir,
        &[
            "--platform",
            platform,
            "--target-device",
            "iphone",
            "--target-device",
            "ipad",
        ],
    );
}

pub fn build_macos() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let contents = profile_dir(&out_dir).join(format!("{APP_NAME}.app/Contents"));
    let macos = contents.join("MacOS");
    let resources = contents.join("Resources");
    if let Err(err) = fs::create_dir_all(&macos).and_then(|_| fs::create_dir_all(&resources)) {
        println!("cargo:warning=failed to create {APP_NAME}.app: {err}");
        return;
    }

    let exe = macos.join(APP_NAME);
    if fs::symlink_metadata(&exe).is_err() {
        let bin = env::var("CARGO_PKG_NAME").unwrap();
        if let Err(err) = std::os::unix::fs::symlink(format!("../../../{bin}"), &exe) {
            println!("cargo:warning=failed to link {APP_NAME} executable: {err}");
        }
    }

    write_info_plist("macos", &contents);
    compile_icon(&resources, &out_dir, &["--platform", "macosx"]);
}

fn compile_icon(resources: &Path, out_dir: &Path, platform_args: &[&str]) {
    println!("cargo:rerun-if-changed=assets/{ICON_NAME}.icon");
    let icon_src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join(format!("assets/{ICON_NAME}.icon"));
    if !icon_src.is_dir() {
        return;
    }

    let output = Command::new("xcrun")
        .arg("actool")
        .arg(&icon_src)
        .arg("--compile")
        .arg(resources)
        .args(platform_args)
        .args(["--minimum-deployment-target", "26.0"])
        .args(["--app-icon", ICON_NAME])
        .arg("--output-partial-info-plist")
        .arg(out_dir.join("actool-partial.plist"))
        .output();

    match output {
        Ok(out) if out.status.success() => {}
        Ok(out) => println!(
            "cargo:warning=actool failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ),
        Err(err) => println!("cargo:warning=actool unavailable: {err}"),
    }
}

fn write_info_plist(platform: &str, dest: &Path) {
    let template = format!("assets/{platform}/Info.plist");
    println!("cargo:rerun-if-changed={template}");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let plist = match fs::read_to_string(&template) {
        Ok(plist) => plist,
        Err(err) => {
            println!("cargo:warning=failed to read {template}: {err}");
            return;
        }
    };
    let plist = plist
        .replace("${APP_NAME}", APP_NAME)
        .replace("${BUNDLE_ID}", BUNDLE_ID)
        .replace("${ICON_NAME}", ICON_NAME)
        .replace("${VERSION}", &env::var("CARGO_PKG_VERSION").unwrap());

    if let Err(err) = fs::write(dest.join("Info.plist"), plist) {
        println!("cargo:warning=failed to write Info.plist: {err}");
    }
}
