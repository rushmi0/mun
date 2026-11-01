use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "i686-unknown-linux-gnu".into(),
        pointer_width: 32,
        arch: "x86".into(),
        data_layout: "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128".into(),
        options: TargetOptions {
            // Same baseline as Rust's i686 targets: SSE2.
            cpu: "pentium4".into(),
            ..super::linux_base::opts()
        },
    }
}
