use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "armv7-unknown-linux-musleabihf".into(),
        pointer_width: 32,
        arch: "arm".into(),
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        options: TargetOptions {
            abi: "eabihf".into(),
            // Same baseline as Rust's armv7 hard-float targets: Thumb-2 and
            // VFPv3-D16, but not NEON.
            features: "+v7,+vfp3,-d32,+thumb2,-neon".into(),
            ..super::linux_musl_base::opts()
        },
    }
}
