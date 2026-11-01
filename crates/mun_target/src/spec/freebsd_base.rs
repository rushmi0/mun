use crate::spec::{LinkerFlavor, TargetOptions};

pub fn opts() -> TargetOptions {
    TargetOptions {
        os: "freebsd".to_string(),
        env: "".to_string(),
        vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Ld,
        ..Default::default()
    }
}
