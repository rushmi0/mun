use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "aarch64-linux-android".into(),
        pointer_width: 64,
        arch: "aarch64".into(),
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".into(),
        options: TargetOptions {
            // Android requires NEON and floating-point support on all arm64 devices.
            features: "+neon,+fp-armv8".into(),
            ..super::android_base::opts()
        },
    }
}
