use bindgen::callbacks::{
    EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks,
};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct Callbacks;

// https://github.com/zmwangx/rust-ffmpeg-sys/blob/8c643bbc85a7c832781173d356290c3361c88099/build.rs#L76
impl ParseCallbacks for Callbacks {
    fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
        let ch_layout_prefix = "AV_CH_";
        let codec_cap_prefix = "AV_CODEC_CAP_";
        let codec_flag_prefix = "AV_CODEC_FLAG_";
        let error_max_size = "AV_ERROR_MAX_STRING_SIZE";

        if _name.starts_with(ch_layout_prefix) {
            Some(IntKind::ULongLong)
        } else if value >= i32::MIN as i64
            && value <= i32::MAX as i64
            && (_name.starts_with(codec_cap_prefix) || _name.starts_with(codec_flag_prefix))
        {
            Some(IntKind::UInt)
        } else if _name == error_max_size {
            Some(IntKind::Custom {
                name: "usize",
                is_signed: false,
            })
        } else if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
            Some(IntKind::Int)
        } else {
            None
        }
    }

    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        let dummy_codec_id_prefix = "AV_CODEC_ID_FIRST_";
        if original_variant_name.starts_with(dummy_codec_id_prefix) {
            Some(EnumVariantCustomBehavior::Constify)
        } else {
            None
        }
    }

    // https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-388277405
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        use MacroParsingBehavior::*;

        match name {
            "FP_INFINITE" => Ignore,
            "FP_NAN" => Ignore,
            "FP_NORMAL" => Ignore,
            "FP_SUBNORMAL" => Ignore,
            "FP_ZERO" => Ignore,
            _ => Default,
        }
    }
}

fn fetch_and_install_vcpkg() {
    let is_windows = std::env::consts::OS == "windows";
    let status = Command::new("rm")
        .arg("-rf")
        .arg("vcpkg")
        .arg("vcpkg_installed")
        .status()
        .unwrap();
    assert!(
        status.success(),
        "Failed to remove existing vcpkg directory"
    );

    let status = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("https://github.com/microsoft/vcpkg.git")
        .status()
        .unwrap();
    assert!(status.success(), "Failed to clone vcpkg repository");

    let ext = if is_windows { "bat" } else { "sh" };
    let status = Command::new(format!("./vcpkg/bootstrap-vcpkg.{}", ext))
        .arg("-disableMetrics")
        .status()
        .unwrap();
    assert!(status.success(), "Failed to bootstrap vcpkg");
}

fn install_ffmpeg(features: &Vec<&str>) {
    let mut features = features.clone();
    features.insert(0, "core");
    let port_name = format!("ffmpeg[{}]", features.join(","));

    let status = Command::new("vcpkg/vcpkg")
        .arg("install")
        .arg("--recurse")
        .arg(port_name)
        .status()
        .unwrap();
    assert!(status.success(), "Failed to install FFmpeg with vcpkg");
}

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cfg_feature = env::var("CARGO_CFG_FEATURE").unwrap();

    if profile == "release" || !Path::new("vcpkg").exists() {
        fetch_and_install_vcpkg();
    }

    let features: Vec<_> = cfg_feature
        .split(',')
        .filter(|feature| !feature.is_empty() && !feature.contains("default"))
        .collect();

    install_ffmpeg(&features);

    let mut libs = features.clone();
    libs.push("avutil");

    let lib_path = format!("{}/vcpkg/installed/x64-linux/lib", manifest_dir);
    let include_path = format!("{}/vcpkg/installed/x64-linux/include", manifest_dir);

    println!("cargo:rustc-link-search={}", lib_path);

    // TODO: refactor
    println!("cargo:rustc-link-search={}", "/usr/lib/gcc/x86_64-linux-gnu/13");
    println!("cargo:rustc-link-lib=static=stdc++");

    for lib in libs {
        // println!("cargo:warning={}", lib);
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivcpkg/installed/x64-linux/include")
        .header(format!("{}/libavcodec/avcodec.h", include_path))
        .header(format!("{}/libavformat/avformat.h", include_path))
        .header(format!("{}/libavutil/opt.h", include_path))
        .parse_callbacks(Box::new(Callbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
