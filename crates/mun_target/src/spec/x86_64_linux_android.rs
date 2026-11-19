use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "x86_64-linux-android".into(),
        pointer_width: 64,
        arch: "x86_64".into(),
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .into(),
        options: TargetOptions {
            cpu: "x86-64".into(),
            // The Android x86_64 ABI guarantees SSE4.2 and POPCNT.
            features: "+mmx,+sse,+sse2,+sse3,+ssse3,+sse4.1,+sse4.2,+popcnt".into(),
            ..super::android_base::opts()
        },
    }
}
