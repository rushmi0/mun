use mun_target::{abi::TargetDataLayout, spec::Target};

#[test]
fn data_layout_windows() {
    let layout =
        TargetDataLayout::parse(&Target::search("x86_64-pc-windows-msvc").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}

#[test]
fn data_layout_darwin() {
    let layout = TargetDataLayout::parse(&Target::search("x86_64-apple-darwin").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}

#[test]
fn data_layout_linux() {
    let layout =
        TargetDataLayout::parse(&Target::search("x86_64-unknown-linux-gnu").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}

#[test]
fn data_layout_android_aarch64() {
    let layout =
        TargetDataLayout::parse(&Target::search("aarch64-linux-android").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}

#[test]
fn data_layout_android_x86_64() {
    let layout = TargetDataLayout::parse(&Target::search("x86_64-linux-android").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}

#[test]
fn data_layout_android_armv7() {
    let layout =
        TargetDataLayout::parse(&Target::search("armv7-linux-androideabi").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}

#[test]
fn data_layout_android_i686() {
    let layout = TargetDataLayout::parse(&Target::search("i686-linux-android").unwrap()).unwrap();

    insta::assert_debug_snapshot!(layout);
}
