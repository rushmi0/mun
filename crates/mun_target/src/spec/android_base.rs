use crate::spec::{LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "android".to_string(),
        env: "".to_string(),
        vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Ld,
        // Android 15+ requires shared libraries to be 16 KB page aligned.
        pre_link_args: vec!["-z".into(), "max-page-size=16384".into()],
        ..Default::default()
    }
}
