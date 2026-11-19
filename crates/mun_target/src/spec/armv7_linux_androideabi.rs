use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "armv7-none-linux-android".into(),
        pointer_width: 32,
        arch: "arm".into(),
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        options: TargetOptions {
            abi: "eabi".into(),
            // The armeabi-v7a ABI guarantees Thumb-2 and VFPv3-D16, but not NEON.
            features: "+v7,+thumb-mode,+thumb2,+vfp3,-d32,-neon".into(),
            ..super::android_base::opts()
        },
    }
}
